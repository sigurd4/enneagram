use serde::{Deserialize, Serialize};

use crate::config::EdgeConfig;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgesConfig
{
    pub recovery: EdgeConfig,
    pub association: EdgeConfig,
    pub repression: EdgeConfig,
    pub rejection: EdgeConfig,
    pub catatonia: EdgeConfig,
    pub paranoia: EdgeConfig,
    pub disorganization: EdgeConfig,
    pub action: EdgeConfig,
    pub rest: EdgeConfig
}

impl Default for EdgesConfig
{
    fn default() -> Self
    {
        Self {
            recovery: EdgeConfig {
                name: "Recovery".into(),
                pivot: "how will you manage your frustration?".into(),
                digit: [[-1, 3], [0, 4], [0, -4], [-1, -4], [1, -4]].into()
            },
            association: EdgeConfig {
                name: "Association".into(),
                pivot: "how will you gain worth?".into(),
                digit: [[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 0], [-3, -4], [3, -4], [3, -3]].into()
            },
            repression: EdgeConfig {
                name: "Repression".into(),
                pivot: "how will you repress your shame?".into(),
                digit: [[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [1, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3]].into()
            },
            rejection: EdgeConfig {
                name: "Rejection".into(),
                pivot: "how will you deal with your longing?".into(),
                digit: [[3, 1], [-3, 1], [1, 4], [1, -4]].into()
            },
            catatonia: EdgeConfig {
                name: "Catatonia".into(),
                pivot: "how will you gain security?".into(),
                digit: [[3, 4], [-3, 4], [-3, 1], [2, 1], [3, 0], [3, -3], [2, -4], [-3, -4]].into()
            },
            paranoia: EdgeConfig {
                name: "Paranoia".into(),
                pivot: "how will you deal with your insecurity?".into(),
                digit: [[3, 4], [0, 4], [-3, -1], [-2, 0], [2, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3], [-3, -1]].into()
            },
            disorganization: EdgeConfig {
                name: "Disorganization".into(),
                pivot: "how will you handle your fear?".into(),
                digit: [[-3, 4], [3, 4], [-3, -4]].into()
            },
            action: EdgeConfig {
                name: "Action".into(),
                pivot: "how will you gain control?".into(),
                digit: [[1, 0], [3, 1], [3, 3], [2, 4], [-2, 4], [-3, 3], [-3, 1], [-1, 0], [-3, -1], [-3, -3], [-2, -4], [2, -4], [3, -3], [3, -1], [1, 0], [-1, 0]].into()
            },
            rest: EdgeConfig {
                name: "Rest".into(),
                pivot: "how will you suppress your anger?".into(),
                digit: [[3, 1], [2, 0], [-2, 0], [-3, 1], [-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [-3, -4]].into()
            }
        }
    }
}