use core::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShowConfig
{
    pub path_lines: bool,
    pub boundary_lines: bool,
    pub pivot_lines: bool,
    pub triad_lines: bool,
}

impl Borrow<ShowConfig> for Config
{
    fn borrow(&self) -> &ShowConfig
    {
        &self.show
    }
}