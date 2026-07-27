use serde::{Deserialize, Serialize};

use crate::{
    config::{Fallback, Property, PartialTriadConfig, TriadConfig, TriadsConfig}
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrameConfig
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "891", skip_serializing_if = "Option::is_none")]
    gut: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "567", skip_serializing_if = "Option::is_none")]
    head: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "234", skip_serializing_if = "Option::is_none")]
    heart: Option<PartialTriadConfig<'static>>
}

impl FrameConfig
{
    crate::config::getter!([_, c.enneagram.triads.frame].description.as_str() -> &str);

    crate::config::getter!([_, c.enneagram.triads.frame].gut |= -> TriadConfig<'a>);

    crate::config::getter!([_, c.enneagram.triads.frame].head |= -> TriadConfig<'a>);

    crate::config::getter!([_, c.enneagram.triads.frame].heart |= -> TriadConfig<'a>);
}

impl Property for FrameConfig
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a Self 
    {
        self
    }
}
impl<C> Property<FrameConfig> for C
where
    C: Property<TriadsConfig>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a FrameConfig
    {
        self.property(fallback).frame(fallback)
    }
}
