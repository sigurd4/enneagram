use core::{any::Any, ops::Add};

use crate::{domain::Domain, triad::{Means, Fault, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Behaviour
{
    pub introverted: Fault,
    pub extroverted: Means,
}

impl Behaviour
{
    pub fn all() -> [Behaviour; 9]
    {
        use {Fault::*, Means::*};

        [
            Positive + Assertive, Competent + Assertive, Reactive + Assertive,
            Positive + Compliant, Competent + Compliant, Reactive + Compliant,
            Positive + Withdrawn, Competent + Withdrawn, Reactive + Withdrawn
        ]
    }

    pub fn kind() -> &'static str
    {
        "behaviour"
    }
}

impl Add<Means> for Fault
{
    type Output = Behaviour;

    fn add(self, rhs: Means) -> Self::Output
    {
        Behaviour {
            introverted: self,
            extroverted: rhs
        }
    }
}
impl Add<Fault> for Means
{
    type Output = Behaviour;

    fn add(self, rhs: Fault) -> Self::Output
    {
        Behaviour {
            introverted: rhs,
            extroverted: self
        }
    }
}

impl Domain for Behaviour
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn equals(&self, other: &dyn Domain) -> bool
    {
        other.as_any().downcast_ref().is_some_and(|other| self == other)
    }
    
    fn kind(&self) -> &'static str
    {
        Self::kind()
    }
    fn conscious(&self) -> &dyn Triad
    {
        &self.extroverted
    }
    fn subconscious(&self) -> &dyn Triad
    {
        &self.introverted
    }
    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.extroverted.expression(), self.introverted.expression())
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.extroverted.reflection(), self.introverted.reflection())
    }
}