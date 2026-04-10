use core::ops::Add;

use crate::{domain::Domain, triad::{Strategy, Fault, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Behaviour
{
    pub introverted: Fault,
    pub extroverted: Strategy,
}

impl Behaviour
{
    pub fn all() -> [Behaviour; 9]
    {
        use {Fault::*, Strategy::*};

        [
            Positive + Assertive, Competent + Assertive, Reactive + Assertive,
            Positive + Compliant, Competent + Compliant, Reactive + Compliant,
            Positive + Withdrawn, Competent + Withdrawn, Reactive + Withdrawn
        ]
    }
}

impl Add<Strategy> for Fault
{
    type Output = Behaviour;

    fn add(self, rhs: Strategy) -> Self::Output
    {
        Behaviour {
            introverted: self,
            extroverted: rhs
        }
    }
}
impl Add<Fault> for Strategy
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