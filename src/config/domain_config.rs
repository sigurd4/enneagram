use core::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainConfig
{
    pub introverted_dissonance: String,
    pub introverted_synthesis: String,
    pub desire_machine: String,
    pub body_without_organs: String,
    pub extroverted_synthesis: String,
    pub extroverted_dissonance: String
}

impl Borrow<DomainConfig> for Config
{
    fn borrow(&self) -> &DomainConfig
    {
        &self.enneagram.domains
    }
}