use serde::{Deserialize, Serialize};

use crate::config::{Fallback, Property, Config};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShowConfig
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_lines: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boundary_lines: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_lines: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triad_lines: Option<bool>
}

impl ShowConfig
{
    crate::config::getter!([_, c.show].path_lines -> bool);

    crate::config::getter!([_, c.show].boundary_lines -> bool);

    crate::config::getter!([_, c.show].pivot_lines -> bool);

    crate::config::getter!([_, c.show].triad_lines -> bool);
}

impl Property for ShowConfig
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a Self
    {
        self
    }
}
impl<C> Property<ShowConfig> for C
where
    C: Property<Config>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a ShowConfig
    {
        self.property(fallback).show(fallback)
    }
}
