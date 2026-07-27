use core::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::config::{Fallback, Property, EnneagramConfig};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainConfig
{
    #[serde(rename = "A+B", skip_serializing_if = "Option::is_none")]
    pub introverted_dissonance: Option<String>,
    #[serde(rename = "A+C", skip_serializing_if = "Option::is_none")]
    pub introverted_synthesis: Option<String>,
    #[serde(rename = "A&D", skip_serializing_if = "Option::is_none")]
    pub desire_machine: Option<String>,
    #[serde(rename = "C&B", skip_serializing_if = "Option::is_none")]
    pub body_without_organs: Option<String>,
    #[serde(rename = "D+B", skip_serializing_if = "Option::is_none")]
    pub extroverted_synthesis: Option<String>,
    #[serde(rename = "D+C", skip_serializing_if = "Option::is_none")]
    pub extroverted_dissonance: Option<String>
}

impl DomainConfig
{
    crate::config::getter!([_, c.enneagram.domains].introverted_dissonance.deref() -> &str);

    crate::config::getter!([_, c.enneagram.domains].introverted_synthesis.deref() -> &str);

    crate::config::getter!([_, c.enneagram.domains].desire_machine.deref() -> &str);

    crate::config::getter!([_, c.enneagram.domains].body_without_organs.deref() -> &str);

    crate::config::getter!([_, c.enneagram.domains].extroverted_synthesis.deref() -> &str);

    crate::config::getter!([_, c.enneagram.domains].extroverted_dissonance.deref() -> &str);
}

impl Property for DomainConfig
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a Self
    {
        self
    }
}
impl<C> Property<DomainConfig> for C
where
    C: Property<EnneagramConfig>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a DomainConfig
    {
        self.property(fallback).domains(fallback)
    }
}
