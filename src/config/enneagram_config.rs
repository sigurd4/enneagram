use core::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::config::{Config, DomainConfig, EdgesConfig, TriadsConfig};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EnneagramConfig
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<EdgesConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triads: Option<TriadsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<DomainConfig>
}

impl EnneagramConfig
{
    pub fn edges(&self) -> &EdgesConfig
    {
        Config::fallback(self, |c| &c.edges)
    }
    pub fn triads(&self) -> &TriadsConfig
    {
        Config::fallback(self, |c| &c.triads)
    }
    pub fn domains(&self) -> &DomainConfig
    {
        Config::fallback(self, |c| &c.domains)
    }
}

impl Borrow<EnneagramConfig> for Config
{
    fn borrow(&self) -> &EnneagramConfig
    {
        &self.enneagram()
    }
}