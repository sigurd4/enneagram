use core::any::Any;

use crate::{enneatype::Enneatype, triad::Triad};

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
        [Fault::Positive, Fault::Competent, Fault::Reactive]
    }
}

impl Triad for Fault
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn equals(&self, other: &dyn Triad) -> bool
    {
        other.as_any().downcast_ref().is_some_and(|other| self == other)
    }
    
    fn edges(&self) -> &'static [Enneatype; 3]
    {
        match self
        {
            Fault::Positive => &[Enneatype::Disorganization, Enneatype::Rest, Enneatype::Association], // 792
            Fault::Competent => &[Enneatype::Recovery, Enneatype::Repression, Enneatype::Catatonia], // 135
            Fault::Reactive => &[Enneatype::Rejection, Enneatype::Paranoia, Enneatype::Action], // 468
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
            Fault::Positive => "you tell yourself that everything is fine",
            Fault::Competent => "you hold yourself responsible",
            Fault::Reactive => "you blame others"
        }
    }
    fn affirmation(&self) -> &'static str
    {
        match self
        {
            Fault::Positive => "stay positive",
            Fault::Competent => "take responsibility",
            Fault::Reactive => "blame others"
        }
    }
}