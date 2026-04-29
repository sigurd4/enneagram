use core::any::Any;

use crate::{enneatype::Enneatype, triad::Triad};

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
        [Frame::Gut, Frame::Head, Frame::Heart]
    }
}

impl Triad for Frame
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn equals(&self, other: &dyn Triad) -> bool
    {
        other.as_any().downcast_ref().is_some_and(|other| self == other)
    }

    fn edges(&self) -> &'static [crate::enneatype::Enneatype; 3]
    {
        match self
        {
            Frame::Gut => &[Enneatype::Action, Enneatype::Rest, Enneatype::Recovery], // 891
            Frame::Head => &[Enneatype::Catatonia, Enneatype::Paranoia, Enneatype::Disorganization], // 567
            Frame::Heart => &[Enneatype::Association, Enneatype::Repression, Enneatype::Rejection], // 234
        }
    }
    fn expression(&self) -> &'static str
    {
        match self
        {
            Frame::Gut => "I am my urges, my concience hurts me",
            Frame::Head => "I am my thoughts, my fear hurts me",
            Frame::Heart => "I am my emotions, my feelings hurt me"
        }
    }
    fn reflection(&self) -> &'static str
    {
        match self
        {
            Frame::Gut => "you have become your urges, your conscience hurts you",
            Frame::Head => "you have become your thoughts, your fear hurts you",
            Frame::Heart => "you have become your emotions, your feelings hurt you"
        }
    }
    fn affirmation(&self) -> &'static str
    {
        match self
        {
            Frame::Gut => "follow my gut",
            Frame::Head => "use my head",
            Frame::Heart => "follow my heart",
        }
    }
}