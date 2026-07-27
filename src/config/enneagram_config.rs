use serde::{Deserialize, Serialize};

use crate::{
    config::{Fallback, Property, Config, DomainConfig, EdgesConfig, TriadsConfig}
};

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

impl Property for EnneagramConfig
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a Self
    {
        self
    }
}
impl<C> Property<EnneagramConfig> for C
where
    C: Property<Config>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a EnneagramConfig
    {
        self.property(fallback).enneagram(fallback)
    }
}
