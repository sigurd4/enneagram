use crate::{edge::Edge, triad::Triad};

/// Homonculus of the self/internalization of self/frame of judgement/meta-objective/"Who am i?"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Frame
{
    #[display("Gut/\"I am my urges, my concience hurts me\"")]
    Gut,
    #[display("Head/\"I am my thoughts, my fear hurts me\"")]
    Head,
    #[display("Heart/\"I am my emotions, my feelings hurt me\"")]
    Heart
}

impl Frame
{
    pub const fn all() -> [Self; 3]
    {
        [Self::Gut, Self::Head, Self::Heart]
    }
}

impl Triad for Frame
{
    fn edges(&self) -> &'static [crate::edge::Edge; 3]
    {
        match self
        {
            Frame::Gut => &[Edge::Action, Edge::Rest, Edge::Recovery], // 891
            Frame::Head => &[Edge::Catatonia, Edge::Paranoia, Edge::Disorganization], // 567
            Frame::Heart => &[Edge::Association, Edge::Repression, Edge::Rejection], // 234
        }
    }
    fn expression(&self) -> &'static str
    {
        match self
        {
            Self::Gut => "I am my urges, my concience hurts me",
            Self::Head => "I am my thoughts, my fear hurts me",
            Self::Heart => "I am my emotions, my feelings hurt me"
        }
    }

    fn reflection(&self) -> &'static str
    {
        match self
        {
            Self::Gut => "you have become your urges, your conscience hurts you",
            Self::Head => "you have become your thoughts, your fear hurts you",
            Self::Heart => "you have become your emotions, your feelings hurt you"
        }
    }
}