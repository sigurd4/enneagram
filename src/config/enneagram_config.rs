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
    crate::config::getter!([_, c.enneagram].edges -> &EdgesConfig);
    crate::config::getter!([_, c.enneagram].triads -> &TriadsConfig);
    crate::config::getter!([_, c.enneagram].domains -> &DomainConfig);
}

impl Borrow<EnneagramConfig> for Config
{
    fn borrow(&self) -> &EnneagramConfig
    {
        &self.enneagram()
    }
}