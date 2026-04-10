use core::{any::Any, ops::Add};

use crate::{domain::Domain, triad::{Strategy, Frame, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InternalDissonance
{
    pub thesis: Frame,
    pub anti_thesis: Strategy
}

impl InternalDissonance
{
    pub fn all() -> [InternalDissonance; 9]
    {
        use {Frame::*, Strategy::*};

        [
            Gut + Assertive, Head + Assertive, Heart + Assertive,
            Gut + Compliant, Head + Compliant, Heart + Compliant,
            Gut + Withdrawn, Head + Withdrawn, Heart + Withdrawn,
        ]
    }
}

impl Add<Strategy> for Frame
{
    type Output = InternalDissonance;

    fn add(self, rhs: Strategy) -> Self::Output
    {
        InternalDissonance {
            thesis: self,
            anti_thesis: rhs
        }
    }
}
impl Add<Frame> for Strategy
{
    type Output = InternalDissonance;

    fn add(self, rhs: Frame) -> Self::Output
    {
        InternalDissonance {
            thesis: rhs,
            anti_thesis: self
        }
    }
}

impl Domain for InternalDissonance
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn equals(&self, other: &dyn Domain) -> bool
    {
        other.as_any().downcast_ref().is_some_and(|other| self == other)
    }
    
    fn conscious(&self) -> &dyn Triad
    {
        &self.thesis
    }
    fn subconscious(&self) -> &dyn Triad
    {
        &self.anti_thesis
    }
    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{}, but {}", self.thesis.expression(), self.anti_thesis.expression())
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{}, so {}", self.thesis.reflection(), self.anti_thesis.reflection())
    }
}