use serde::{Deserialize, Serialize};

use crate::{
    config::{Fallback, Property, PartialTriadConfig, TriadConfig, TriadsConfig}
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeansConfig
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "378", skip_serializing_if = "Option::is_none")]
    assertive: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "612", skip_serializing_if = "Option::is_none")]
    compliant: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "945", skip_serializing_if = "Option::is_none")]
    withdrawn: Option<PartialTriadConfig<'static>>
}

impl MeansConfig
{
    crate::config::getter!([_, c.enneagram.triads.means].description.as_str() -> &str);

    crate::config::getter!([_, c.enneagram.triads.means].assertive |= -> TriadConfig<'a>);

    crate::config::getter!([_, c.enneagram.triads.means].compliant |= -> TriadConfig<'a>);

    crate::config::getter!([_, c.enneagram.triads.means].withdrawn |= -> TriadConfig<'a>);
}

impl Property for MeansConfig
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a Self 
    {
        self
    }
}
impl<C> Property<MeansConfig> for C
where
    C: Property<TriadsConfig>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a MeansConfig
    {
        self.property(fallback).means(fallback)
    }
}
