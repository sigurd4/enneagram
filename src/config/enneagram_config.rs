use serde::{Deserialize, Serialize};

use crate::config::{DomainConfig, EdgesConfig, TriadsConfig};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EnneagramConfig
{
    pub triads: TriadsConfig,
    pub edges: EdgesConfig,
    pub domains: DomainConfig
}