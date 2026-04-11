use core::any::Any;

use crate::{edge::Edge, triad::Triad};

/// External strategy towards suffering / means
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Means
{
    #[display("Assertive/\"I can change it\"")]
    Assertive,
    #[display("Compliant/\"I can tolerate it\"")]
    Compliant,
    #[display("Withdrawn/\"I can avoid it\"")]
    Withdrawn
}

impl Means
{
    pub const fn all() -> [Self; 3]
    {
        [Means::Assertive, Means::Compliant, Means::Withdrawn]
    }
}

impl Triad for Means
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn equals(&self, other: &dyn Triad) -> bool
    {
        other.as_any().downcast_ref().is_some_and(|other| self == other)
    }

    fn edges(&self) -> &'static [Edge; 3]
    {
        match self
        {
            Means::Assertive => &[Edge::Repression, Edge::Disorganization, Edge::Action], // 378
            Means::Compliant => &[Edge::Paranoia, Edge::Recovery, Edge::Association], // 612
            Means::Withdrawn => &[Edge::Rest, Edge::Rejection, Edge::Catatonia], // 945
        }
    }
    fn expression(&self) -> &'static str
    {
        match self
        {
            Means::Assertive => "I can change it",
            Means::Compliant => "I can tolerate it",
            Means::Withdrawn => "I can avoid it"
        }
    }
    fn reflection(&self) -> &'static str
    {
        match self
        {
            Means::Assertive => "you believe you can change it",
            Means::Compliant => "you believe you can tolerate it",
            Means::Withdrawn => "you believe you can avoid it"
        }
    }
    fn affirmation(&self) -> &'static str
    {
        match self
        {
            Means::Assertive => "change it",
            Means::Compliant => "tolerate it",
            Means::Withdrawn => "avoid it"
        }
    }
}