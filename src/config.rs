use core::{convert::Infallible, fmt::Display, ops::Deref, str::FromStr};
use std::{borrow::Cow, collections::{HashSet, VecDeque}, env::VarError, fs::File, path::{Path, PathBuf}, sync::{Arc, LazyLock, Mutex}};

use ratatui_3d::Rgb;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};

fn rgb_to_string(value: &Rgb) -> String
{
    let Rgb(r, g, b) = value;

    format!("{r:02X}{g:02X}{b:02X}")
}
fn str_to_rgb(src: &str) -> Rgb
{
    let rgb = u32::from_str_radix(src, 16)
        .expect(&format!("Unable to parse RBG hexadecimal color '{src}'."));

    assert!(rgb <= 0xFFFFFF, "RGB color cannot have alpha-channel.");

    Rgb(
        (rgb >> 16) as u8,
        (rgb >> 8) as u8,
        rgb as u8
    )
}
fn serialize_rgb<S>(value: &Rgb, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let hex = rgb_to_string(value);

    hex.serialize(serializer)
}
fn deserialize_rgb<'de, D>(deserializer: D) -> Result<Rgb, D::Error>
where
    D: Deserializer<'de>
{
    let src = String::deserialize(deserializer)?;

    Ok(str_to_rgb(&src))
}

fn serialize_digit<S>(value: &Box<[[i8; 2]]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    value.iter()
        .map(|[x, y]| format!("{x},{y}"))
        .collect::<Vec<_>>()
        .join(" ")
        .serialize(serializer)
}
fn deserialize_digit<'de, D>(deserializer: D) -> Result<Box<[[i8; 2]]>, D::Error>
where
    D: Deserializer<'de>
{
    Ok(
        String::deserialize(deserializer)?
            .split(" ")
            .map(|xy| xy.split(",")
                .map(|e| i8::from_str_radix(e, 10).expect("Failed parsing 8-bit int coordinate of digit."))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Each point of the digit must contain exactly two coordinates.")
            ).collect::<Vec<_>>()
            .into()
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigConfig
{
    directories: Vec<PathBuf>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShowConfig
{
    pub path_lines: bool,
    pub boundary_lines: bool,
    pub pivot_lines: bool,
    pub triad_lines: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColorConfig
{
    #[serde(serialize_with = "serialize_rgb", deserialize_with = "deserialize_rgb")]
    pub surface: Rgb,
    #[serde(serialize_with = "serialize_rgb", deserialize_with = "deserialize_rgb")]
    pub wire: Rgb,
    #[serde(serialize_with = "serialize_rgb", deserialize_with = "deserialize_rgb")]
    pub dyed: Rgb,
    #[serde(serialize_with = "serialize_rgb", deserialize_with = "deserialize_rgb")]
    pub glare: Rgb,
    #[serde(serialize_with = "serialize_rgb", deserialize_with = "deserialize_rgb")]
    pub sun: Rgb
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgeConfig
{
    pub name: String,
    pub pivot: String,
    #[serde(serialize_with = "serialize_digit", deserialize_with = "deserialize_digit")]
    pub digit: Box<[[i8; 2]]>
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgesConfig
{
    pub recovery: EdgeConfig,
    pub association: EdgeConfig,
    pub repression: EdgeConfig,
    pub rejection: EdgeConfig,
    pub catatonia: EdgeConfig,
    pub paranoia: EdgeConfig,
    pub disorganization: EdgeConfig,
    pub action: EdgeConfig,
    pub rest: EdgeConfig
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriadConfig
{
    pub description: String,
    pub expression: String,
    pub reflection: String,
    pub affirmation: String
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriadsConfig
{
    pub positive: TriadConfig,
    pub competent: TriadConfig,
    pub reactive: TriadConfig,
    pub gut: TriadConfig,
    pub head: TriadConfig,
    pub heart: TriadConfig,
    pub assertive: TriadConfig,
    pub compliant: TriadConfig,
    pub withdrawn: TriadConfig,
    pub attachment: TriadConfig,
    pub frustration: TriadConfig,
    pub rejection: TriadConfig
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainConfig
{
    pub introverted_dissonance: String,
    pub introverted_synthesis: String,
    pub desire_machine: String,
    pub body_without_organs: String,
    pub extroverted_synthesis: String,
    pub extroverted_dissonance: String
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EnneagramConfig
{
    pub triads: TriadsConfig,
    pub edges: EdgesConfig,
    pub domains: DomainConfig
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ConfigData
{
    config: ConfigConfig,
    pub show: ShowConfig,
    pub color: ColorConfig,
    pub enneagram: EnneagramConfig
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default = "Config::read_default", deny_unknown_fields)]
pub struct Config(ConfigData);

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

impl Default for ConfigConfig
{
    fn default() -> Self
    {
        Self {
            directories: Config::default_config_dirs()
        }
    }
}

impl Default for ShowConfig
{
    fn default() -> Self
    {
        Self {
            path_lines: true,
            boundary_lines: true,
            pivot_lines: true,
            triad_lines: true,
        }
    }
}

impl Default for ColorConfig
{
    fn default() -> Self
    {
        Self {
            surface: Rgb(0xF0, 0xFF, 0xFF),
            wire: Rgb(0xFF, 0x00, 0x00),
            dyed: Rgb(0xFF, 0xC0, 0x40),
            glare: Rgb(0xFF, 0xFF, 0xFF),
            sun: Rgb(0xFF, 0xFF, 0xFF)
        }
    }
}

impl Default for EdgesConfig
{
    fn default() -> Self
    {
        Self {
            recovery: EdgeConfig {
                name: "Recovery".into(),
                pivot: "how will you manage your frustration?".into(),
                digit: [[-1, 3], [0, 4], [0, -4], [-1, -4], [1, -4]].into()
            },
            association: EdgeConfig {
                name: "Association".into(),
                pivot: "how will you gain worth?".into(),
                digit: [[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 0], [-3, -4], [3, -4], [3, -3]].into()
            },
            repression: EdgeConfig {
                name: "Repression".into(),
                pivot: "how will you repress your shame?".into(),
                digit: [[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [1, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3]].into()
            },
            rejection: EdgeConfig {
                name: "Rejection".into(),
                pivot: "how will you deal with your longing?".into(),
                digit: [[3, 1], [-3, 1], [1, 4], [1, -4]].into()
            },
            catatonia: EdgeConfig {
                name: "Catatonia".into(),
                pivot: "how will you gain security?".into(),
                digit: [[3, 4], [-3, 4], [-3, 1], [2, 1], [3, 0], [3, -3], [2, -4], [-3, -4]].into()
            },
            paranoia: EdgeConfig {
                name: "Paranoia".into(),
                pivot: "how will you deal with your insecurity?".into(),
                digit: [[3, 4], [0, 4], [-3, -1], [-2, 0], [2, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3], [-3, -1]].into()
            },
            disorganization: EdgeConfig {
                name: "Disorganization".into(),
                pivot: "how will you handle your fear?".into(),
                digit: [[-3, 4], [3, 4], [-3, -4]].into()
            },
            action: EdgeConfig {
                name: "Action".into(),
                pivot: "how will you gain control?".into(),
                digit: [[1, 0], [3, 1], [3, 3], [2, 4], [-2, 4], [-3, 3], [-3, 1], [-1, 0], [-3, -1], [-3, -3], [-2, -4], [2, -4], [3, -3], [3, -1], [1, 0], [-1, 0]].into()
            },
            rest: EdgeConfig {
                name: "Rest".into(),
                pivot: "how will you suppress your anger?".into(),
                digit: [[3, 1], [2, 0], [-2, 0], [-3, 1], [-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [-3, -4]].into()
            }
        }
    }
}

impl Default for TriadsConfig
{
    fn default() -> Self
    {
        Self {
            positive: TriadConfig {
                description: "Positive/\"everything is fine\"".into(),
                expression: "everything is fine".into(),
                reflection: "you tell yourself that everything is fine".into(),
                affirmation: "stay positive".into()
            },
            competent: TriadConfig {
                description: "Competent/\"I take responsibility\"".into(),
                expression: "I take responsibility".into(),
                reflection: "you hold yourself responsible".into(),
                affirmation: "take responsibility".into()
            },
            reactive: TriadConfig {
                description: "Reactive/\"it's their fault\"".into(),
                expression: "it's their fault".into(),
                reflection: "you blame others".into(),
                affirmation: "blame others".into()
            },
            gut: TriadConfig {
                description: "Gut/\"I am my urges, my concience hurts me\"".into(),
                expression: "I am my urges, my concience hurts me".into(),
                reflection: "you have become your urges, your conscience hurts you".into(),
                affirmation: "follow my gut".into()
            },
            head: TriadConfig {
                description: "Head/\"I am my thoughts, my fear hurts me\"".into(),
                expression: "I am my thoughts, my fear hurts me".into(),
                reflection: "you have become your thoughts, your fear hurts you".into(),
                affirmation: "use my head".into()
            },
            heart: TriadConfig {
                description: "Heart/\"I am my emotions, my feelings hurt me\"".into(),
                expression: "I am my emotions, my feelings hurt me".into(),
                reflection: "you have become your emotions, your feelings hurt you".into(),
                affirmation: "follow my heart".into()
            },
            assertive: TriadConfig {
                description: "Assertive/\"I can change it\"".into(),
                expression: "I can change it".into(),
                reflection: "you believe you can change it".into(),
                affirmation: "change it".into()
            },
            compliant: TriadConfig {
                description: "Compliant/\"I can tolerate it\"".into(),
                expression: "I can tolerate it".into(),
                reflection: "you believe you can tolerate it".into(),
                affirmation: "tolerate it".into()
            },
            withdrawn: TriadConfig {
                description: "Withdrawn/\"I can avoid it\"".into(),
                expression: "I can avoid it".into(),
                reflection: "you believe you can avoid it".into(),
                affirmation: "avoid it".into()
            },
            attachment: TriadConfig {
                description: "Attachment/\"I need freedom\"".into(),
                expression: "I need freedom".into(),
                reflection: "you crave freedom".into(),
                affirmation: "be free".into()
            },
            frustration: TriadConfig {
                description: "Frustration/\"I need control\"".into(),
                expression: "I need control".into(),
                reflection: "you crave control".into(),
                affirmation: "be in control".into()
            },
            rejection: TriadConfig {
                description: "Rejection/\"I need love\"".into(),
                expression: "I need love".into(),
                reflection: "you crave love".into(),
                affirmation: "be accepted".into()
            }
        }
    }
}

impl Default for DomainConfig
{
    fn default() -> Self
    {
        Self {
            introverted_dissonance: "introverted dissonance".into(),
            introverted_synthesis: "introverted synthesis".into(),
            desire_machine: "desire-machine".into(),
            body_without_organs: "body without organs".into(),
            extroverted_synthesis: "extroverted synthesis".into(),
            extroverted_dissonance: "extroverted dissonance".into()
        }
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
        let env = std::env::var(variable)
            .map_err(|error| FindEnvDirectoryError::MissingVariable {
                error,
                variable: variable.to_string()
            })?;
        let mut dir = match PathBuf::from_str(&env)
        {
            Ok(dir) => dir,
            Err(infallible) => match infallible {}
        };
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

    fn user_config_dir() -> Result<PathBuf, FindUserConfigDirectoryError>
    {
        let xdg_config_home_dir = Self::xdg_config_home_dir()?;
        let mut config_dir = xdg_config_home_dir.join(Path::new("enneagram"));
        config_dir = loop
        {
            match Self::find_directory(&config_dir)
            {
                Ok(config_dir) => break config_dir.into_owned(),
                Err(error) => match error
                {
                    FindDirectoryError::Nonexistant { path } => std::fs::create_dir(&path)
                        .map_err(|error| FindUserConfigDirectoryError::Creation(CreateDirectoryError::Failed {
                            path,
                            error
                        }))?,
                    FindDirectoryError::NotADirectory { path } => return Err(
                        FindUserConfigDirectoryError::Creation(CreateDirectoryError::NotADirectory { path })
                    )
                },
            }
        };
        Ok(config_dir)
    }

    pub fn add_config_dirs<'a>(new_config_dirs: impl IntoIterator<Item: Into<Cow<'a, Path>>>) -> Vec<PathBuf>
    {
        let mut config_dirs = CONFIG_DIRS.lock()
            .expect("Failed to lock config directory search paths cache.");
        for search_path in new_config_dirs.into_iter()
            .map(Into::into)
        {
            if let Ok(config_dir) = Self::find_directory(search_path)
                && let config_dir = config_dir.into_owned()
                && !config_dirs.contains(&config_dir)
            {
                config_dirs.push(config_dir);
            }
        }
        config_dirs.clone()
    }

    pub fn default_config_dirs() -> Vec<PathBuf>
    {
        let config_dirs = vec![
            match Self::user_config_dir()
            {
                Ok(config_dir) => config_dir.into(),
                Err(error) => panic!("User configuration directory not found: {error}")
            },
            PathBuf::from(SYSTEMWISE_CONFIG_DIR)
        ];
        Self::add_config_dirs(config_dirs.iter().map(PathBuf::as_path));
        config_dirs
    }

    pub fn config_dirs() -> Vec<PathBuf>
    {
        Self::add_config_dirs(
            Self::default_config_dirs()
        )
    }

    pub fn config_path<'a>(config: &'a str) -> Cow<'a, Path>
    {
        let mut config_path = Cow::from(Path::new(config));
        if config_path.file_name()
            .expect(&format!("Configuration '{}' has no filename", config_path.to_string_lossy()))
            == config_path.as_os_str()
        {
            for config_dir in Self::config_dirs()
            {
                let config_search_path = config_dir.join(&config_path);
                if config_search_path.exists() && config_search_path.is_file()
                {
                    config_path = config_search_path.into();
                    break
                }
            }
        }
        let _ = config_path.extension()
            .and_then(|extension| extension.to_str())
            .filter(|extension| *extension == "yaml")
            .expect(&format!("Configuration '{}' should have extension '.yaml'", config_path.to_string_lossy()));
        config_path
    }
    pub fn default_config_path() -> Cow<'static, Path>
    {
        Self::config_path("default.yaml")
    }

    fn write_config_path(&self, config_path: &std::path::Path)
    {
        let yaml = serde_saphyr::to_string(self)
            .expect(&format!("Unable to serialize configuration '{}'.", config_path.to_string_lossy()));
        std::fs::write(config_path, yaml)
            .expect(&format!("Unable to create configuration '{}'.", config_path.to_string_lossy()))
    }

    fn read_generic_type_path<T>(config_path: &std::path::Path) -> T
    where
        T: DeserializeOwned
    {
        let yaml = File::open(config_path)
            .expect(&format!("Unable to serialize configuration '{}'.", config_path.to_string_lossy()));
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
    use ratatui_3d::Rgb;

use crate::config::{Config, rgb_to_string, str_to_rgb};

    #[test]
    fn test_serde()
    {
        let config = Config::default();

        let yaml = serde_saphyr::to_string(&config).expect("Serialization failed.");

        println!("{yaml}");

        let config2 = serde_saphyr::from_str(&yaml).expect("Deserialization failed.");

        assert_eq!(config, config2, "Serialized then deserialized config differ from original.")
    }

    #[test]
    fn test_hex()
    {
        let rgb = Rgb(0, 10, 0);

        let hex = rgb_to_string(&rgb);

        println!("{}", hex);

        let rgb_decoded = str_to_rgb(&hex);

        assert_eq!(rgb, rgb_decoded)
    }
}