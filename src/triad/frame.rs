use core::{any::Any, borrow::Borrow};

use crate::{config::{TriadConfig, TriadsConfig}, enneatype::Enneatype, triad::Triad};

/// Homonculus of the self/internalization of self/frame of judgement/meta-objective/"Who am i?"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frame
{
    Gut,
    Head,
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
    fn config<'a>(&self, config: &'a dyn Borrow<TriadsConfig>) -> TriadConfig<'a>
    {
        let triads = config.borrow();
        match self
        {
            Frame::Gut => triads.gut(),
            Frame::Head => triads.head(),
            Frame::Heart => triads.heart()
        }
    }
}