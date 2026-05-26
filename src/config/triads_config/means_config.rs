use core::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::config::{Config, EnneagramConfig, PartialTriadConfig, TriadConfig, TriadsConfig};

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
    crate::config::getter!([_, c.enneagram.triads.means].assertive |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads.means].compliant |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads.means].withdrawn |= -> TriadConfig<'_>);
}

impl Borrow<MeansConfig> for TriadsConfig
{
    fn borrow(&self) -> &MeansConfig
    {
        self.means()
    }
}
impl Borrow<MeansConfig> for EnneagramConfig
{
    fn borrow(&self) -> &MeansConfig
    {
        self.triads().borrow()
    }
}
impl Borrow<MeansConfig> for Config
{
    fn borrow(&self) -> &MeansConfig
    {
        self.enneagram().borrow()
    }
}