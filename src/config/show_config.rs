use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShowConfig
{
    pub path_lines: bool,
    pub boundary_lines: bool,
    pub pivot_lines: bool,
    pub triad_lines: bool,
}

impl Default for ShowConfig
{
    fn default() -> Self
    {
        Self {
            path_lines: true,
            boundary_lines: true,
            pivot_lines: true,
            triad_lines: true,
        }
    }
}