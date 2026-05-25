use std::borrow::Cow;

use serde::{Deserialize, Serialize};

crate::config::def_unitary!(
    struct TriadConfig for PartialTriadConfig
    {
        description: str,
        expression: str,
        reflection: str,
        affirmation: str
    }
);