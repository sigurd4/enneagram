use crate::{edge::Edge, triad::Triad};

/// Internal strategy for one's (meta-)suffering/"who to blame?"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Fault
{
    #[display("Positive/\"everything is fine\"")]
    Positive,
    #[display("Competent/\"I take responsibility\"")]
    Competent,
    #[display("Reactive/\"it's their fault\"")]
    Reactive
}

impl Fault
{
    pub const fn all() -> [Self; 3]
    {
        [Self::Positive, Self::Competent, Self::Reactive]
    }
}

impl Triad for Fault
{
    fn edges(&self) -> &'static [Edge; 3]
    {
        match self
        {
            Fault::Positive => &[Edge::Action, Edge::Rest, Edge::Association], // 892
            Fault::Competent => &[Edge::Recovery, Edge::Repression, Edge::Catatonia], // 135
            Fault::Reactive => &[Edge::Rejection, Edge::Paranoia, Edge::Action], // 468
        }
    }
    fn expression(&self) -> &'static str
    {
        match self
        {
            Fault::Positive => "everything is fine",
            Fault::Competent => "I take responsibility",
            Fault::Reactive => "it's their fault",
        }
    }

    fn reflection(&self) -> &'static str
    {
        match self
        {
            Fault::Positive => "you delude yourself into thinking that everything is fine",
            Fault::Competent => "you hold yourself responsible",
            Fault::Reactive => "you blame others"
        }
    }
}