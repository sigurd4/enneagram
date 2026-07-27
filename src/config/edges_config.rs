use serde::{Deserialize, Serialize};

use crate::{
    config::{Fallback, Property, EdgeConfig, EnneagramConfig, PartialEdgeConfig}
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgesConfig
{
    #[serde(rename = "1", skip_serializing_if = "Option::is_none")]
    recovery: Option<PartialEdgeConfig<'static>>,
    #[serde(rename = "2", skip_serializing_if = "Option::is_none")]
    association: Option<PartialEdgeConfig<'static>>,
    #[serde(rename = "3", skip_serializing_if = "Option::is_none")]
    repression: Option<PartialEdgeConfig<'static>>,
    #[serde(rename = "4", skip_serializing_if = "Option::is_none")]
    rejection: Option<PartialEdgeConfig<'static>>,
    #[serde(rename = "5", skip_serializing_if = "Option::is_none")]
    catatonia: Option<PartialEdgeConfig<'static>>,
    #[serde(rename = "6", skip_serializing_if = "Option::is_none")]
    paranoia: Option<PartialEdgeConfig<'static>>,
    #[serde(rename = "7", skip_serializing_if = "Option::is_none")]
    disorganization: Option<PartialEdgeConfig<'static>>,
    #[serde(rename = "8", skip_serializing_if = "Option::is_none")]
    action: Option<PartialEdgeConfig<'static>>,
    #[serde(rename = "9", skip_serializing_if = "Option::is_none")]
    rest: Option<PartialEdgeConfig<'static>>
}

impl EdgesConfig
{
    crate::config::getter!([_, c.enneagram.edges].recovery |= -> EdgeConfig<'a>);

    crate::config::getter!([_, c.enneagram.edges].association |= -> EdgeConfig<'a>);

    crate::config::getter!([_, c.enneagram.edges].repression |= -> EdgeConfig<'a>);

    crate::config::getter!([_, c.enneagram.edges].rejection |= -> EdgeConfig<'a>);

    crate::config::getter!([_, c.enneagram.edges].catatonia |= -> EdgeConfig<'a>);

    crate::config::getter!([_, c.enneagram.edges].paranoia |= -> EdgeConfig<'a>);

    crate::config::getter!([_, c.enneagram.edges].disorganization |= -> EdgeConfig<'a>);

    crate::config::getter!([_, c.enneagram.edges].action |= -> EdgeConfig<'a>);

    crate::config::getter!([_, c.enneagram.edges].rest |= -> EdgeConfig<'a>);
}

impl Property for EdgesConfig
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a Self
    {
        self
    }
}
impl<C> Property<EdgesConfig> for C
where
    C: Property<EnneagramConfig>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a EdgesConfig
    {
        self.property(fallback).edges(fallback)
    }
}
