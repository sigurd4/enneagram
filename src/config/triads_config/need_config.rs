use core::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::config::{Config, EnneagramConfig, PartialTriadConfig, TriadConfig, TriadsConfig};

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
    crate::config::getter!([_, c.enneagram.triads.need].attachment |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads.need].frustration |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads.need].rejection |= -> TriadConfig<'_>);
}

impl Borrow<NeedConfig> for TriadsConfig
{
    fn borrow(&self) -> &NeedConfig
    {
        self.need()
    }
}
impl Borrow<NeedConfig> for EnneagramConfig
{
    fn borrow(&self) -> &NeedConfig
    {
        self.triads().borrow()
    }
}
impl Borrow<NeedConfig> for Config
{
    fn borrow(&self) -> &NeedConfig
    {
        self.enneagram().borrow()
    }
}