use serde::{Deserialize, Serialize};

use crate::{
    config::{Fallback, PartialTriadConfig, TriadConfig, TriadsConfig, Property}
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NeedConfig
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "369", skip_serializing_if = "Option::is_none")]
    attachment: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "147", skip_serializing_if = "Option::is_none")]
    frustration: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "258", skip_serializing_if = "Option::is_none")]
    rejection: Option<PartialTriadConfig<'static>>
}

impl NeedConfig
{
    crate::config::getter!([_, c.enneagram.triads.need].description.as_str() -> &str);

    crate::config::getter!([_, c.enneagram.triads.need].attachment |= -> TriadConfig<'a>);

    crate::config::getter!([_, c.enneagram.triads.need].frustration |= -> TriadConfig<'a>);

    crate::config::getter!([_, c.enneagram.triads.need].rejection |= -> TriadConfig<'a>);
}

impl Property for NeedConfig
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a Self 
    {
        self
    }
}
impl<C> Property<NeedConfig> for C
where
    C: Property<TriadsConfig>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a NeedConfig
    {
        self.property(fallback).need(fallback)
    }
}
