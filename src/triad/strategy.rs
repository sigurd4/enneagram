use core::any::Any;

use crate::{edge::Edge, triad::Triad};

/// External strategy towards suffering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Strategy
{
    #[display("Assertive/\"I can change it\"")]
    Assertive,
    #[display("Compliant/\"I can tolerate it\"")]
    Compliant,
    #[display("Withdrawn/\"I can avoid it\"")]
    Withdrawn
}

impl Strategy
{
    pub const fn all() -> [Self; 3]
    {
        [Self::Assertive, Self::Compliant, Self::Withdrawn]
    }
}

impl Triad for Strategy
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
            Strategy::Assertive => &[Edge::Repression, Edge::Disorganization, Edge::Action], // 378
            Strategy::Compliant => &[Edge::Paranoia, Edge::Recovery, Edge::Association], // 612
            Strategy::Withdrawn => &[Edge::Rest, Edge::Rejection, Edge::Catatonia], // 945
        }
    }
    fn expression(&self) -> &'static str
    {
        match self
        {
            Self::Assertive => "I can change it",
            Self::Compliant => "I can tolerate it",
            Self::Withdrawn => "I can avoid it"
        }
    }

    fn reflection(&self) -> &'static str
    {
        match self
        {
            Self::Assertive => "you believe you can change it",
            Self::Compliant => "you believe you can tolerate it",
            Self::Withdrawn => "you believe you can avoid it"
        }
    }
}