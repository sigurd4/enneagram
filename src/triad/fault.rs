use core::any::Any;

use crate::{config::{TriadConfig, TriadsConfig}, enneatype::Enneatype, triad::Triad};

/// Internal strategy for one's (meta-)suffering/"who to blame?"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault
{
    Positive,
    Competent,
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
    fn config<'a>(&self, config: TriadsConfig<'a>) -> TriadConfig<'a>
    {
        match self
        {
            Fault::Positive => config.positive,
            Fault::Competent => config.competent,
            Fault::Reactive => config.reactive,
        }
    }
}