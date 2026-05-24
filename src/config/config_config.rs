use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigConfig
{
    pub(in crate::config) directories: Vec<PathBuf>
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