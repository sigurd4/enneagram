use core::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::config::{Config, EnneagramConfig, PartialTriadConfig, TriadConfig, TriadsConfig};

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
    crate::config::getter!([_, c.enneagram.triads.fault].positive |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads.fault].competent |= -> TriadConfig<'_>);
    crate::config::getter!([_, c.enneagram.triads.fault].reactive |= -> TriadConfig<'_>);

}

impl Borrow<FaultConfig> for TriadsConfig
{
    fn borrow(&self) -> &FaultConfig
    {
        self.fault()
    }
}
impl Borrow<FaultConfig> for EnneagramConfig
{
    fn borrow(&self) -> &FaultConfig
    {
        self.triads().borrow()
    }
}
impl Borrow<FaultConfig> for Config
{
    fn borrow(&self) -> &FaultConfig
    {
        self.enneagram().borrow()
    }
}