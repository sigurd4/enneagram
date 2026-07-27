use core::{
    fmt::{Debug, Display},
    ops::BitOrAssign
};
use std::{
    borrow::Cow,
    env::VarError,
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
    string::ToString
};

use serde::{Deserialize, Serialize};

moddef::moddef!(
    flat(pub) mod {
        color_config,
        domain_config,
        edge_config,
        edges_config,
        enneagram_config,
        show_config,
        triad_config,
        triads_config
    },
    mod {
        presets
    }
);

macro_rules! member {
    ($fallback:expr, [$this:expr, $conf:ident$(.$subconf:ident)*].$member:ident) => {
        $fallback.fallback($this.$member.as_ref(), |$conf| (|| {
            $(let $conf = $conf.$subconf.as_ref()?;)*

            $conf.$member.as_ref()
        })())
    };
    ($fallback:expr, [$this:expr, $conf:ident$(.$subconf:ident)*].$member:ident |=) => {
        $fallback.fallback_fold($this.$member.as_ref(), |$conf| (|| {
            $(let $conf = $conf.$subconf.as_ref()?;)*

            $conf.$member.as_ref()
        })())
    };
}
use member;

macro_rules! getter {
    ([_, $conf:ident$(.$subconf:ident)*].$member:ident$(.$deref:ident())* -> &$ty:ty) => {
        pub fn $member<'a>(&'a self, fallback: &'a crate::config::Fallback) -> &'a $ty
        {
            crate::config::member!(fallback, [self, $conf$(.$subconf)*].$member)$(.$deref())*
        }
    };
    ([_, $conf:ident$(.$subconf:ident)*].$member:ident$(.$deref:ident())* -> $ty:ty) => {
        pub fn $member(&self, fallback: &'_ crate::config::Fallback) -> $ty
        {
            *crate::config::member!(fallback, [self, $conf$(.$subconf)*].$member)$(.$deref())*
        }
    };
    ([_, $conf:ident$(.$subconf:ident)*].$member:ident$(.$deref:ident())* |= -> $ty:ty) => {
        pub fn $member<'a>(&'a self, fallback: &'a crate::config::Fallback) -> $ty
        {
            crate::config::member!(fallback, [self, $conf$(.$subconf)*].$member |=)$(.$deref())*
        }
    };
}
use getter;

macro_rules! def_unitary {
    (
        struct $name:ident for $partial:ident
        {
            $($var:ident: $var_ty:ty),+$(,)?
        }
    ) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[derive(Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $partial<'a>
        {
            $(
                #[serde(skip_serializing_if = "Option::is_none")]
                $var: Option<Cow<'a, $var_ty>>,
            )+
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name<'a>
        {
            $(
                pub $var: Cow<'a, $var_ty>,
            )+
        }

        impl<'a> core::ops::BitOrAssign<&'a $partial<'a>> for $partial<'a>
        {
            fn bitor_assign(&mut self, rhs: &'a $partial<'a>)
            {
                let Self { $($var,)+ } = rhs;

                $(
                    if let Some(v) = $var.as_ref() { self.$var.get_or_insert_with(|| Cow::Borrowed(v.as_ref())); }
                )+
            }
        }

        impl<'a> From<&'a $partial<'a>> for $partial<'a>
        {
            fn from(value: &'a $partial<'a>) -> Self
            {
                let Self { $($var,)+ } = value;

                Self {
                    $(
                        $var: $var.as_ref().map(|c| Cow::Borrowed(c.as_ref())),
                    )+
                }
            }
        }

        impl<'a> TryFrom<&'a $partial<'a>> for $name<'a>
        {
            type Error = &'a $partial<'a>;

            fn try_from(value: &'a $partial<'a>) -> Result<Self, Self::Error>
            {
                let $partial { $($var,)+ } = value;

                Ok(Self {
                    $(
                        $var: Cow::Borrowed($var.as_ref().ok_or(value)?.as_ref()),
                    )+
                })
            }
        }
        impl<'a> TryFrom<$partial<'a>> for $name<'a>
        {
            type Error = $partial<'a>;

            fn try_from(value: $partial<'a>) -> Result<Self, Self::Error>
            {
                let $partial { $($var,)+ } = value.clone();

                Ok(Self {
                    $(
                        $var: $var.ok_or(value.clone())?,
                    )+
                })
            }
        }
    };
}
use def_unitary;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Config
{
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<ShowConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<ColorConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enneagram: Option<EnneagramConfig>
}

impl Config
{
    crate::config::getter!([_, c].show -> &ShowConfig);

    crate::config::getter!([_, c].color -> &ColorConfig);

    crate::config::getter!([_, c].enneagram -> &EnneagramConfig);
}

impl Property for Config
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a Self
    {
        self
    }
}

const DEFAULT_CONFIG_FILENAME: &str = "default.yaml";
const DEFAULT_CONFIG_DATA: &str = include_str!("../../presets/default.yaml");
const HOME_DIRECTORY_ENV_VARIABLE: &str = "HOME";
const CONFIG_SUBDIRECTORY_UNIX: &str = ".config";
const XDG_CONFIG_HOME_DIRECTORY_ENV_VARIABLE: &str = "XDG_CONFIG_HOME";

pub trait Property<T: Property = Self>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a T;
}

#[derive(Clone)]
pub struct Fallback
{
    default: Config,
    fallback: Vec<Config>
}
impl Default for Fallback
{
    fn default() -> Self
    {
        let default_path = Config::config_path(DEFAULT_CONFIG_FILENAME);
        let default_preset = serde_saphyr::from_str::<Config>(DEFAULT_CONFIG_DATA).expect("Failed to parse original default config.");

        if !default_path.exists()
        {
            default_preset.write_config(DEFAULT_CONFIG_FILENAME);
        }

        Self {
            default: Config::read_config(DEFAULT_CONFIG_FILENAME),
            fallback: Vec::new()
        }
    }
}
impl Fallback
{
    fn iter(&self) -> core::iter::Chain<core::slice::Iter<'_, Config>, core::iter::Once<&'_ Config>>
    {
        self.fallback.iter().chain(core::iter::once(&self.default))
    }

    fn fallback<'a, T>(&'a self, initial: Option<&'a T>, getter: impl Fn(&'a Config) -> Option<&'a T>) -> &'a T
    {
        initial
            .into_iter()
            .chain(self.iter().filter_map(|config| getter(config)))
            .next()
            .expect("No configuration contained the requested value.")
    }

    fn fallback_fold<'a, T, V>(&'a self, initial: Option<&'a V>, getter: impl Fn(&'a Config) -> Option<&'a V>) -> T
    where
        &'a V: TryInto<T, Error = &'a V> + Into<V>,
        V: TryInto<T, Error = V> + BitOrAssign<&'a V> + Debug + 'a
    {
        let mut fold = None;
        for value in initial.into_iter().chain(self.iter().filter_map(|config| getter(config)))
        {
            let fold_taken = match fold.take()
            {
                None => value.into(),
                Some(mut fold_taken) =>
                {
                    fold_taken |= value;
                    fold_taken
                }
            };
            match fold_taken.try_into()
            {
                Ok(output) => return output,
                Err(fold_taken) => fold = Some(fold_taken)
            }
        }
        fold.expect("No configuration contained the requested value.").try_into().expect("Item incomplete.")
    }

    pub fn add_fallback(&mut self, fallback_config: Config)
    {
        self.fallback.push(fallback_config);
    }
}

impl Config
{
    fn find_directory<'a>(dir: impl Into<Cow<'a, Path>>) -> Result<Cow<'a, Path>, FindDirectoryError>
    {
        let dir = dir.into();
        if !dir.exists()
        {
            return Err(FindDirectoryError::Nonexistant { path: dir.into() });
        }
        if !dir.is_dir()
        {
            return Err(FindDirectoryError::NotADirectory { path: dir.into() });
        }
        Ok(dir)
    }

    fn find_env_directory(variable: &str) -> Result<PathBuf, FindEnvDirectoryError>
    {
        // Extract string from environment.
        let env = std::env::var(variable).map_err(|error| FindEnvDirectoryError::MissingVariable {
            error,
            variable: variable.to_string()
        })?;

        if env.is_empty()
        {
            return Err(FindEnvDirectoryError::EmptyVariable { variable: variable.to_string() });
        }

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

    fn create_directory(dir: &Path, upon_creation: impl Fn()) -> Result<Cow<'_, Path>, CreateDirectoryError>
    {
        loop
        {
            match Self::find_directory(dir)
            {
                Ok(dir) => return Ok(dir),
                Err(error) => match error
                {
                    FindDirectoryError::Nonexistant { path } =>
                    {
                        std::fs::create_dir(&path).map_err(|error| CreateDirectoryError::Failed { path, error })?;
                        upon_creation()
                    }
                    FindDirectoryError::NotADirectory { path } => return Err(CreateDirectoryError::NotADirectory { path }.into())
                }
            }
        }
    }

    fn config_dir() -> Result<PathBuf, FindUserConfigDirectoryError>
    {
        // Construct as subdirectory of $XDG_CONFIG_DIR
        let mut config_dir = Self::xdg_config_home_dir()?.join(Path::new("enneagram"));

        // Verify and create if needed.
        config_dir = Self::create_directory(&config_dir, || {
            #[cfg(feature = "presets")]
            for (config_name, yaml) in presets::PRESETS
            {
                let config_path = Self::config_path(config_name);

                std::fs::write(&config_path, yaml).expect(&format!("Failed to write preset '{config_name}' to '{}'", config_path.to_string_lossy()))
            }
        })?
        .into_owned();

        Ok(config_dir)
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
        if config_path.components().last().is_some_and(|last| last.as_os_str().to_string_lossy() == config_path)
        {
            let user_config_dir = match Self::config_dir()
            {
                Ok(config_dir) => config_dir,
                Err(error) => panic!("User configuration directory not found: {error}")
            };

            config_path = user_config_dir.join(config_path).into()
        }

        // Check extension.
        if !config_path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension == "yaml")
        {
            panic!("Configuration '{}' should have extension '.yaml'", config_path.to_string_lossy())
        }

        config_path
    }

    pub fn default_config_path() -> Cow<'static, Path>
    {
        Config::config_path(DEFAULT_CONFIG_FILENAME)
    }

    fn write_config_path(&self, config_path: &Path)
    {
        // Serialize to yaml.
        let yaml = serde_saphyr::to_string_with_options(
            self,
            serde_saphyr::ser_options! {
                empty_as_braces: true,
                indent_step: 2,
                compact_list_indent: true,
                tagged_enums: false,
                prefer_block_scalars: true,
                quote_all: false,
                yaml_12: false
            }
        )
        .expect(&format!("Unable to serialize configuration '{}'.", config_path.to_string_lossy()));

        // Write to filesystem.
        std::fs::write(config_path, yaml).expect(&format!("Unable to create configuration '{}'.", config_path.to_string_lossy()))
    }

    #[allow(unused)]
    pub fn write_config(&self, config: &str)
    {
        self.write_config_path(&Self::config_path(config))
    }

    fn read_config_path(config_path: &Path) -> Config
    {
        // Read from filesystem.
        let yaml = File::open(config_path).expect(&format!("Unable to serialize configuration '{}'.", config_path.to_string_lossy()));

        // Deserialize from yaml.
        serde_saphyr::from_reader(yaml)
            //serde_saphyr::from_str_validate(&yaml) // TODO: implement validator
            .expect(&format!("Unable to parse configuration '{}'.", config_path.to_string_lossy()))
    }

    pub fn read_config(config: &str) -> Config
    {
        Self::read_config_path(&Self::config_path(config))
    }
}

enum FindDirectoryError
{
    Nonexistant
    {
        path: PathBuf
    },
    NotADirectory
    {
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
    MissingVariable
    {
        error: VarError, variable: String
    },
    EmptyVariable
    {
        variable: String
    },
    NotFound
    {
        error: FindDirectoryError, variable: String
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
            Self::EmptyVariable { variable } => write!(f, "variable '${variable}' is empty."),
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
            }
            | FindEnvDirectoryError::EmptyVariable { variable }
                if variable == XDG_CONFIG_HOME_DIRECTORY_ENV_VARIABLE =>
            {
                match Config::home_dir() // Try to use ~/.config instead.
            {
                Ok(home_dir) => match Config::find_directory(home_dir.join(Path::new(CONFIG_SUBDIRECTORY_UNIX)))
                {
                    Ok(config_dir) => Err(config_dir.into_owned()),
                    Err(_) => Ok(Self(error))
                },
                Err(FindHomeDirectoryError(error)) => Ok(Self(error))
            }
            }
            _ => Ok(Self(error))
        }
    }
}

enum CreateDirectoryError
{
    Failed
    {
        path: PathBuf, error: std::io::Error
    },
    NotADirectory
    {
        path: PathBuf
    }
}

impl Display for CreateDirectoryError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Failed { path, error } => write!(f, "Creation of directory '{}' failed. {error}", path.to_string_lossy()),
            Self::NotADirectory { path } => write!(
                f,
                "Creation of directory '{}' failed. A file by the same name already exists and is not a directory.",
                path.to_string_lossy()
            )
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
