use serde::{Deserialize, Serialize};

use crate::{
    config::{Fallback, Property, EnneagramConfig}
};

moddef::moddef!(
    flat(pub) mod {
        frame_config,
        means_config,
        fault_config,
        need_config,
    }
);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriadsConfig
{
    #[serde(rename = "A", skip_serializing_if = "Option::is_none")]
    frame: Option<FrameConfig>,
    #[serde(rename = "B", skip_serializing_if = "Option::is_none")]
    means: Option<MeansConfig>,
    #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
    fault: Option<FaultConfig>,
    #[serde(rename = "D", skip_serializing_if = "Option::is_none")]
    need: Option<NeedConfig>
}

impl TriadsConfig
{
    crate::config::getter!([_, c.enneagram.triads].frame -> &FrameConfig);

    crate::config::getter!([_, c.enneagram.triads].means -> &MeansConfig);

    crate::config::getter!([_, c.enneagram.triads].fault -> &FaultConfig);

    crate::config::getter!([_, c.enneagram.triads].need -> &NeedConfig);
}

impl Property for TriadsConfig
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a Self
    {
        self
    }
}
impl<C> Property<TriadsConfig> for C
where
    C: Property<EnneagramConfig>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a TriadsConfig
    {
        self.property(fallback).triads(fallback)
    }
}
