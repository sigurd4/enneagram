use core::{any::Any, borrow::Borrow};

use crate::{config::{TriadConfig, TriadsConfig}, enneatype::Enneatype, triad::Triad};

/// External strategy towards suffering / means
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Means
{
    Assertive,
    Compliant,
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

    fn edges(&self) -> &'static [Enneatype; 3]
    {
        match self
        {
            Means::Assertive => &[Enneatype::Repression, Enneatype::Disorganization, Enneatype::Action], // 378
            Means::Compliant => &[Enneatype::Paranoia, Enneatype::Recovery, Enneatype::Association], // 612
            Means::Withdrawn => &[Enneatype::Rest, Enneatype::Rejection, Enneatype::Catatonia], // 945
        }
    }
    fn config<'a>(&self, config: &'a dyn Borrow<TriadsConfig>) -> TriadConfig<'a>
    {
        let triads = config.borrow();
        let means = triads.means();
        match self
        {
            Means::Assertive => means.assertive(),
            Means::Compliant => means.compliant(),
            Means::Withdrawn => means.withdrawn()
        }
    }
    fn kind<'a>(&self, config: &'a dyn Borrow<TriadsConfig>) -> &'a str
    {
        let triads = config.borrow();
        let means = triads.means();
        means.description()
    }
}