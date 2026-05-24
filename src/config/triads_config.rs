use serde::{Deserialize, Serialize};

use crate::config::TriadConfig;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriadsConfig
{
    #[serde(rename = "891")]
    pub gut: TriadConfig,
    #[serde(rename = "567")]
    pub head: TriadConfig,
    #[serde(rename = "234")]
    pub heart: TriadConfig,
    #[serde(rename = "378")]
    pub assertive: TriadConfig,
    #[serde(rename = "612")]
    pub compliant: TriadConfig,
    #[serde(rename = "945")]
    pub withdrawn: TriadConfig,
    #[serde(rename = "792")]
    pub positive: TriadConfig,
    #[serde(rename = "135")]
    pub competent: TriadConfig,
    #[serde(rename = "468")]
    pub reactive: TriadConfig,
    #[serde(rename = "369")]
    pub attachment: TriadConfig,
    #[serde(rename = "147")]
    pub frustration: TriadConfig,
    #[serde(rename = "258")]
    pub rejection: TriadConfig
}