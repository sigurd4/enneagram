use core::any::Any;

use crate::{edge::Edge, triad::Triad};

/// Need/object of desire/"what hole do you have in your soul?"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Need
{
    #[display("Attachment/\"I need freedom\"")]
    Attachment,
    #[display("Frustration/\"I need control\"")]
    Frustration,
    #[display("Rejection/\"I need love\"")]
    Rejection
}
impl Need
{
    pub const fn all() -> [Self; 3]
    {
        [Need::Attachment, Need::Frustration, Need::Rejection]
    }
}

impl Triad for Need
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
            Need::Attachment => &[Edge::Repression, Edge::Paranoia, Edge::Rest], // 369
            Need::Frustration => &[Edge::Recovery, Edge::Rejection, Edge::Disorganization], // 147
            Need::Rejection => &[Edge::Association, Edge::Catatonia, Edge::Action], // 258
        }
    }
    fn expression(&self) -> &'static str
    {
        match self
        {
            Need::Attachment => "I need freedom",
            Need::Frustration => "I need control",
            Need::Rejection => "I need love",
        }
    }
    fn reflection(&self) -> &'static str
    {
        match self
        {
            Need::Attachment => "you crave freedom",
            Need::Frustration => "you crave control",
            Need::Rejection => "you crave love"
        }
    }
    fn affirmation(&self) -> &'static str
    {
        match self
        {
            Need::Attachment => "be free",
            Need::Frustration => "be in control",
            Need::Rejection => "be accepted"
        }
    }
}