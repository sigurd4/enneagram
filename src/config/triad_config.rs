use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriadConfig
{
    pub description: String,
    pub expression: String,
    pub reflection: String,
    pub affirmation: String
}