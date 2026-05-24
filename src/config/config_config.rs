use core::borrow::Borrow;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigConfig
{
    pub(in crate::config) directories: Vec<PathBuf>
}

impl Borrow<ConfigConfig> for Config
{
    fn borrow(&self) -> &ConfigConfig
    {
        &self.config
    }
}