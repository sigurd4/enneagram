use core::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::config::{Config, EnneagramConfig, PartialTriadConfig, TriadConfig};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriadsConfig
{
    #[serde(rename = "891", skip_serializing_if = "Option::is_none")]
    gut: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "567", skip_serializing_if = "Option::is_none")]
    head: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "234", skip_serializing_if = "Option::is_none")]
    heart: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "378", skip_serializing_if = "Option::is_none")]
    assertive: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "612", skip_serializing_if = "Option::is_none")]
    compliant: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "945", skip_serializing_if = "Option::is_none")]
    withdrawn: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "792", skip_serializing_if = "Option::is_none")]
    positive: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "135", skip_serializing_if = "Option::is_none")]
    competent: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "468", skip_serializing_if = "Option::is_none")]
    reactive: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "369", skip_serializing_if = "Option::is_none")]
    attachment: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "147", skip_serializing_if = "Option::is_none")]
    frustration: Option<PartialTriadConfig<'static>>,
    #[serde(rename = "258", skip_serializing_if = "Option::is_none")]
    rejection: Option<PartialTriadConfig<'static>>
}

impl TriadsConfig
{
    crate::config::getter!([_, c.enneagram.triads].gut |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].head |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].heart |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].assertive |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].compliant |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].withdrawn |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].positive |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].competent |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].reactive |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].attachment |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].frustration |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads].rejection |= -> TriadConfig<'_>);
}

impl Borrow<TriadsConfig> for EnneagramConfig
{
    fn borrow(&self) -> &TriadsConfig
    {
        self.triads()
    }
}
impl Borrow<TriadsConfig> for Config
{
    fn borrow(&self) -> &TriadsConfig
    {
        self.enneagram().borrow()
    }
}