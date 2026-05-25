use core::{borrow::Borrow, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::config::{Config, EnneagramConfig};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainConfig
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introverted_dissonance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introverted_synthesis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desire_machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_without_organs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extroverted_synthesis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extroverted_dissonance: Option<String>
}

impl DomainConfig
{
    pub fn introverted_dissonance(&self) -> &str
    {
        crate::config::member!([self, c.enneagram.domains].introverted_dissonance).deref()
    }
    pub fn introverted_synthesis(&self) -> &str
    {
        crate::config::member!([self, c.enneagram.domains].introverted_synthesis).deref()
    }
    pub fn desire_machine(&self) -> &str
    {
        crate::config::member!([self, c.enneagram.domains].desire_machine).deref()
    }
    pub fn body_without_organs(&self) -> &str
    {
        crate::config::member!([self, c.enneagram.domains].body_without_organs).deref()
    }
    pub fn extroverted_synthesis(&self) -> &str
    {
        crate::config::member!([self, c.enneagram.domains].extroverted_synthesis).deref()
    }
    pub fn extroverted_dissonance(&self) -> &str
    {
        crate::config::member!([self, c.enneagram.domains].extroverted_dissonance).deref()
    }
}

impl Borrow<DomainConfig> for EnneagramConfig
{
    fn borrow(&self) -> &DomainConfig
    {
        self.domains()
    }
}
impl Borrow<DomainConfig> for Config
{
    fn borrow(&self) -> &DomainConfig
    {
        self.enneagram().borrow()
    }
}