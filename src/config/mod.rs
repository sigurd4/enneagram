use core::{fmt::Display, ops::Deref, str::FromStr};
use std::{borrow::Cow, env::VarError, fs::File, path::{Path, PathBuf}, sync::{Arc, LazyLock, Mutex, MutexGuard}};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

moddef::moddef!(
    flat(pub) mod {
        color_config,
        config_config,
        domain_config,
        edge_config,
        edges_config,
        enneagram_config,
        show_config,
        triad_config,
        triads_config
    }
);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default = "Config::read_default", deny_unknown_fields)]
pub struct Config(ConfigData);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ConfigData
{
    config: ConfigConfig,
    pub show: ShowConfig,
    pub color: ColorConfig,
    pub enneagram: EnneagramConfig
}

impl Deref for Config
{
    type Target = ConfigData;

    fn deref(&self) -> &Self::Target
    {
        &self.0 
    }
}
impl From<ConfigData> for Config
{
    fn from(data: ConfigData) -> Self
    {
        Self(data)
    }
}
impl From<Config> for ConfigData
{
    fn from(config: Config) -> ConfigData
    {
        config.0
    }
}

/// First-in-first-out buffer of fallback configs. Will be emptied upon next reading of a config.
static FALLBACK_FIFO: LazyLock<Arc<Mutex<Vec<String>>>> = LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

/// Configuration directories added throughout runtime.
static CONFIG_DIRS: LazyLock<Arc<Mutex<Vec<PathBuf>>>> = LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

const SYSTEMWISE_CONFIG_DIR: &str = "/etc/enneagram";
const DEFAULT_CONFIG_FILENAME: &str = "default.yaml";

enum FindDirectoryError
{
    Nonexistant {
        path: PathBuf
    },
    NotADirectory {
        path: PathBuf
    }
}

impl Display for FindDirectoryError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Nonexistant { path } => write!(f, "directory '{}' doesn't exist.", path.to_string_lossy()),
            Self::NotADirectory { path } => write!(f, "directory '{}' isn't a directory.", path.to_string_lossy())
        }
    }
}

enum FindEnvDirectoryError
{
    MissingVariable {
        error: VarError,
        variable: String
    },
    NotFound {
        error: FindDirectoryError,
        variable: String
    }
}

impl Display for FindEnvDirectoryError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::MissingVariable { error, variable } => match error
            {
                VarError::NotPresent => write!(f, "variable '${variable}' not defined. {error}"),
                VarError::NotUnicode(_os_string) => write!(f, "unable to parse variable '${variable}'. {error}")
            },
            Self::NotFound { error, variable } => write!(f, "variable '${variable}' i.e. {error}")
        }
    }
}

struct FindHomeDirectoryError(FindEnvDirectoryError);

impl Display for FindHomeDirectoryError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let Self(error) = self;
        write!(f, "Unable to locate home-directory: {error}")
    }
}
impl From<FindEnvDirectoryError> for FindHomeDirectoryError
{
    fn from(error: FindEnvDirectoryError) -> Self
    {
        Self(error) // All possibilities are bad.
    }
}

struct FindXdgConfigHomeDirectoryError(FindEnvDirectoryError);

impl Display for FindXdgConfigHomeDirectoryError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let Self(error) = self;

        write!(f, "Unable to locate directory for user configuration: {error}")
    }
}
impl TryFrom<FindEnvDirectoryError> for FindXdgConfigHomeDirectoryError
{
    type Error = PathBuf;

    fn try_from(error: FindEnvDirectoryError) -> Result<Self, Self::Error>
    {
        match &error
        {
            FindEnvDirectoryError::MissingVariable {
                error: VarError::NotPresent,
                variable
            } if variable == HOME_DIRECTORY_ENV_VARIABLE => match Config::home_dir() // Try to use ~/.config instead.
            {
                Ok(home_dir) => match Config::find_directory(home_dir.join(Path::new(CONFIG_SUBDIRECTORY_UNIX)))
                {
                    Ok(config_dir) => Err(config_dir.into_owned()),
                    Err(_) => Ok(Self(error))
                },
                Err(FindHomeDirectoryError(error)) => Ok(Self(error))
            },
            _ => Ok(Self(error))
        }
    }
}

enum CreateDirectoryError
{
    Failed {
        path: PathBuf,
        error: std::io::Error
    },
    NotADirectory {
        path: PathBuf        
    }
}

impl Display for CreateDirectoryError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Failed {
                path,
                error
            } => write!(f, "Creation of directory '{}' failed. {error}", path.to_string_lossy()),
            Self::NotADirectory {
                path
            } => write!(f, "Creation of directory '{}' failed. A file by the same name already exists and is not a directory.", path.to_string_lossy())
        }
    }
}

enum FindUserConfigDirectoryError
{
    Unavailable(FindXdgConfigHomeDirectoryError),
    Creation(CreateDirectoryError)
}

impl From<FindXdgConfigHomeDirectoryError> for FindUserConfigDirectoryError
{
    fn from(error: FindXdgConfigHomeDirectoryError) -> Self
    {
        Self::Unavailable(error)
    }
}
impl From<CreateDirectoryError> for FindUserConfigDirectoryError
{
    fn from(error: CreateDirectoryError) -> Self
    {
        Self::Creation(error)
    }
}
impl Display for FindUserConfigDirectoryError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Unavailable(error) => write!(f, "Configuration directory unavailable. {error}"),
            Self::Creation(error) => write!(f, "Unable to create configuration directory. {error}")
        }
    }
}

const HOME_DIRECTORY_ENV_VARIABLE: &str = "HOME";
const CONFIG_SUBDIRECTORY_UNIX: &str = ".config";
const XDG_CONFIG_HOME_DIRECTORY_ENV_VARIABLE: &str = "XDG_CONFIG_HOME";

impl Config
{
    fn pop_fallback() -> Option<Self>
    {
        let config_path = {
            let mut fallback = FALLBACK_FIFO.lock()
                .expect("Failed to lock fallback queue upon pop.");
            fallback.pop()
        }?;

        Some(Self::read_config(&config_path))
    }

    pub fn push_fallback(fallback_config_path: String)
    {
        let mut fallback = FALLBACK_FIFO.lock()
            .expect(&format!("Failed to lock fallback queue upon push of '{fallback_config_path}'."));
        fallback.push(fallback_config_path);
    }

    pub fn read_default() -> Self
    {
        let default_config_path = Self::default_config_path();
        if !default_config_path.exists()
        {
            Config::default().write_config_path(&default_config_path);
        }
        if let Some(fallback) = Self::pop_fallback()
        {
            return fallback
        }
        Self::read_default_config()
    }

    fn find_directory<'a>(dir: impl Into<Cow<'a, Path>>) -> Result<Cow<'a, Path>, FindDirectoryError>
    {
        let dir = dir.into();
        if !dir.exists()
        {
            return Err(FindDirectoryError::Nonexistant {
                path: dir.into()
            })
        }
        if !dir.is_dir()
        {
            return Err(FindDirectoryError::NotADirectory {
                path: dir.into()
            })
        }
        Ok(dir)
    }

    fn find_env_directory(variable: &str) -> Result<PathBuf, FindEnvDirectoryError>
    {
        // Extract string from environment.
        let env = std::env::var(variable)
            .map_err(|error| FindEnvDirectoryError::MissingVariable {
                error,
                variable: variable.to_string()
            })?;

        // Create path-buffer from string.
        let mut dir = match PathBuf::from_str(&env)
        {
            Ok(dir) => dir,
            Err(infallible) => match infallible {}
        };

        // Verify directory.
        dir = Self::find_directory(dir)
            .map_err(|error| FindEnvDirectoryError::NotFound {
                error,
                variable: variable.to_string()
            })?
            .into_owned();

        Ok(dir)
    }

    fn home_dir() -> Result<PathBuf, FindHomeDirectoryError>
    {
        Ok(Self::find_env_directory(HOME_DIRECTORY_ENV_VARIABLE)?)
    }

    fn xdg_config_home_dir() -> Result<PathBuf, FindXdgConfigHomeDirectoryError>
    {
        match Self::find_env_directory(XDG_CONFIG_HOME_DIRECTORY_ENV_VARIABLE)
        {
            Ok(dir) => Ok(dir),
            Err(error) => match error.try_into()
            {
                Ok(error) => Err(error),
                Err(dir) => Ok(dir)
            }
        }
    }

    fn create_directory(dir: &Path) -> Result<Cow<'_, Path>, CreateDirectoryError>
    {
        loop
        {
            match Self::find_directory(dir)
            {
                Ok(dir) => return Ok(dir),
                Err(error) => match error
                {
                    FindDirectoryError::Nonexistant { path } => std::fs::create_dir(&path)
                        .map_err(|error| CreateDirectoryError::Failed {
                            path,
                            error
                        })?,
                    FindDirectoryError::NotADirectory { path } => {
                        return Err(CreateDirectoryError::NotADirectory { path }.into())
                    }
                }
            }
        }
    }

    fn user_config_dir() -> Result<PathBuf, FindUserConfigDirectoryError>
    {
        // Construct as subdirectory of $XDG_CONFIG_DIR
        let mut config_dir = Self::xdg_config_home_dir()?
            .join(Path::new("enneagram"));

        // Verify and create if needed.
        config_dir = Self::create_directory(&config_dir)?
            .into_owned();

        Ok(config_dir)
    }

    fn config_dirs_lock<'a>() -> MutexGuard<'a, Vec<PathBuf>>
    {
        CONFIG_DIRS.lock()
            .expect("Failed to lock config directory search paths cache.")
    }

    fn add_config_dir<'a>(nex_config_dir: impl Into<Cow<'a, Path>>)
    {
        let search_path = nex_config_dir.into();
        if let Ok(config_dir) = Self::find_directory(search_path)
        {
            let config_dir = config_dir.into_owned();
            let mut config_dirs_lock = Self::config_dirs_lock();
            if config_dirs_lock.contains(&config_dir)
            {
                return
            }
            config_dirs_lock.push(config_dir)
        }
    }

    fn add_config_dirs<'a>(new_config_dirs: impl IntoIterator<Item: Into<Cow<'a, Path>>>)
    {
        for search_path in new_config_dirs.into_iter()
        {
            Self::add_config_dir(search_path)
        }
    }

    fn default_config_dirs() -> Vec<PathBuf>
    {
        let user_config_dir = match Self::user_config_dir()
        {
            Ok(config_dir) => config_dir.into(),
            Err(error) => panic!("User configuration directory not found: {error}")
        };

        let config_dirs = vec![
            user_config_dir,
            PathBuf::from(SYSTEMWISE_CONFIG_DIR)
        ];
        Self::add_config_dirs(config_dirs.iter().map(PathBuf::as_path));
        config_dirs
    }

    fn config_dirs() -> Vec<PathBuf>
    {
        Self::add_config_dirs(Self::default_config_dirs());
        Self::config_dirs_lock().clone()
    }

    pub fn config_path<'a>(config: &'a str) -> Cow<'a, Path>
    {
        // Construct path.
        let mut config_path = Cow::from(Path::new(config));

        // Add extension if missing.
        if config_path.extension().is_none()
        {
            config_path = config_path.with_extension("yaml").into()
        }

        // Search in directories if only filename is provided.
        if config_path.components()
            .last()
            .is_some_and(|last| last.as_os_str().to_string_lossy() == config_path)
        {
            // Check visited config dirs.
            let mut config_path_full = None;
            for config_dir in Self::config_dirs()
            {
                let config_search_path = config_dir.join(&config_path);
                if config_search_path.exists() && config_search_path.is_file()
                {
                    config_path_full = Some(config_search_path.into());
                    break
                }
            }

            // Fallback to user config dir.
            config_path = match config_path_full
            {
                Some(config_path) => config_path,
                None => {
                    let user_config_dir = match Self::user_config_dir()
                    {
                        Ok(config_dir) => config_dir,
                        Err(error) => panic!("User configuration directory not found: {error}")
                    };

                    user_config_dir.join(config_path).into()
                }
            }
        }

        // Check extension.
        if !config_path.extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension == "yaml")
        {
            panic!("Configuration '{}' should have extension '.yaml'", config_path.to_string_lossy())
        }

        config_path
    }
    pub fn default_config_path() -> Cow<'static, Path>
    {
        Self::config_path(DEFAULT_CONFIG_FILENAME)
    }

    fn write_config_path(&self, config_path: &std::path::Path)
    {
        // Serialize to yaml.
        let yaml = serde_saphyr::to_string(self)
            .expect(&format!("Unable to serialize configuration '{}'.", config_path.to_string_lossy()));

        // Write to filesystem.
        std::fs::write(config_path, yaml)
            .expect(&format!("Unable to create configuration '{}'.", config_path.to_string_lossy()))
    }

    fn read_generic_type_path<T>(config_path: &std::path::Path) -> T
    where
        T: DeserializeOwned
    {
        // Read from filesystem.
        let yaml = File::open(config_path)
            .expect(&format!("Unable to serialize configuration '{}'.", config_path.to_string_lossy()));

        // Deserialize from yaml.
        serde_saphyr::from_reader(yaml)
            //serde_saphyr::from_str_validate(&yaml) // TODO: implement validator
            .expect(&format!("Unable to parse configuration '{}'.", config_path.to_string_lossy()))
    }

    fn init_hook(&self)
    {
        Self::add_config_dirs(self.config.directories.iter().map(PathBuf::as_path));
    }

    fn read_config_path(config_path: &std::path::Path) -> Config
    {
        let config = Self::read_generic_type_path::<Config>(config_path);
        config.init_hook();
        config
    }

    fn read_default_config() -> Config
    {
        let config = Config(Self::read_generic_type_path(&Self::default_config_path()));
        config.init_hook();
        config
    }

    #[allow(unused)]
    pub fn write_config(self, config: &str)
    {
        self.write_config_path(&Self::config_path(config))
    }

    pub fn read_config(config: &str) -> Config
    {
        Self::read_config_path(&Self::config_path(config))
    }
}

#[cfg(test)]
mod test
{
    use crate::config::Config;

    #[test]
    fn test_serde()
    {
        let config = Config::default();

        let yaml = serde_saphyr::to_string(&config).expect("Serialization failed.");

        println!("{yaml}");

        let config2 = serde_saphyr::from_str(&yaml).expect("Deserialization failed.");

        assert_eq!(config, config2, "Serialized then deserialized config differ from original.")
    }
}