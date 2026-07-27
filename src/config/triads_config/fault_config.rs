use serde::{Deserialize, Serialize};

use crate::{
    config::{Fallback, Property, PartialTriadConfig, TriadConfig, TriadsConfig}
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaultConfig
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "792", skip_serializing_if = "Option::is_none")]
    positive: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "135", skip_serializing_if = "Option::is_none")]
    competent: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "468", skip_serializing_if = "Option::is_none")]
    reactive: Option<PartialTriadConfig<'static>>
}

impl FaultConfig
{
    crate::config::getter!([_, c.enneagram.triads.fault].description.as_str() -> &str);

    crate::config::getter!([_, c.enneagram.triads.fault].positive |= -> TriadConfig<'a>);

    crate::config::getter!([_, c.enneagram.triads.fault].competent |= -> TriadConfig<'a>);

    crate::config::getter!([_, c.enneagram.triads.fault].reactive |= -> TriadConfig<'a>);
}

impl Property for FaultConfig
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a Self 
    {
        self
    }
}
impl<C> Property<FaultConfig> for C
where
    C: Property<TriadsConfig>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a FaultConfig
    {
        self.property(fallback).fault(fallback)
    }
}
