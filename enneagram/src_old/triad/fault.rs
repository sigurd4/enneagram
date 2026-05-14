use core::any::Any;

use crate::{enneatype::Enneatype, triad::{Triad, ITriad}};

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

impl From<Fault> for Triad
{
    fn from(fault: Fault) -> Self
    {
        Self::Fault(fault)
    }
}

impl ITriad for Fault
{
    fn all() -> [Self; 3]
    {
        [Fault::Positive, Fault::Competent, Fault::Reactive]
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