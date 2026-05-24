use serde::{Deserialize, Serialize};

use crate::config::{DomainConfig, EdgesConfig, TriadsConfig};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnneagramConfig
{
    pub edges: EdgesConfig,
    pub triads: TriadsConfig,
    pub domains: DomainConfig
}