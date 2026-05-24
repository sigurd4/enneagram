use core::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::config::{Config, EdgeConfig};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgesConfig
{
    #[serde(rename = "1")]
    pub recovery: EdgeConfig,
    #[serde(rename = "2")]
    pub association: EdgeConfig,
    #[serde(rename = "3")]
    pub repression: EdgeConfig,
    #[serde(rename = "4")]
    pub rejection: EdgeConfig,
    #[serde(rename = "5")]
    pub catatonia: EdgeConfig,
    #[serde(rename = "6")]
    pub paranoia: EdgeConfig,
    #[serde(rename = "7")]
    pub disorganization: EdgeConfig,
    #[serde(rename = "8")]
    pub action: EdgeConfig,
    #[serde(rename = "9")]
    pub rest: EdgeConfig
}

impl Borrow<EdgesConfig> for Config
{
    fn borrow(&self) -> &EdgesConfig
    {
        &self.enneagram.edges
    }
}