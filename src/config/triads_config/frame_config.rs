use core::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::config::{Config, EnneagramConfig, PartialTriadConfig, TriadConfig, TriadsConfig};

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
    heart: Option<PartialTriadConfig<'static>>,
}

impl FrameConfig
{
    crate::config::getter!([_, c.enneagram.triads.frame].description.as_str() -> &str);
    crate::config::getter!([_, c.enneagram.triads.frame].gut |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads.frame].head |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads.frame].heart |= -> TriadConfig<'_>);
}

impl Borrow<FrameConfig> for TriadsConfig
{
    fn borrow(&self) -> &FrameConfig
    {
        self.frame()
    }
}
impl Borrow<FrameConfig> for EnneagramConfig
{
    fn borrow(&self) -> &FrameConfig
    {
        self.triads().borrow()
    }
}
impl Borrow<FrameConfig> for Config
{
    fn borrow(&self) -> &FrameConfig
    {
        self.enneagram().borrow()
    }
}