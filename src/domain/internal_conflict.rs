use core::{any::Any, ops::Add};

use crate::{domain::Domain, triad::{Fault, Frame, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InternalConflict
{
    pub thesis: Frame,
    pub anti_thesis: Fault
}

impl InternalConflict
{
    pub fn all() -> [InternalConflict; 9]
    {
        use {Frame::*, Fault::*};

        [
            Gut + Positive, Head + Positive, Heart + Positive,
            Gut + Competent, Head + Competent, Heart + Competent,
            Gut + Reactive, Head + Reactive, Heart + Reactive,
        ]
    }
}

impl Add<Fault> for Frame
{
    type Output = InternalConflict;

    fn add(self, rhs: Fault) -> Self::Output
    {
        InternalConflict {
            thesis: self,
            anti_thesis: rhs
        }
    }
}
impl Add<Frame> for Fault
{
    type Output = InternalConflict;

    fn add(self, rhs: Frame) -> Self::Output
    {
        InternalConflict {
            thesis: rhs,
            anti_thesis: self
        }
    }
}

impl Domain for InternalConflict
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