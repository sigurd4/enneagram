use core::{any::Any, ops::Add};

use crate::{domain::Domain, triad::{Strategy, Need, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalConflict
{
    pub thesis: Need,
    pub anti_thesis: Strategy
}

impl ExternalConflict
{
    pub fn all() -> [ExternalConflict; 9]
    {
        use {Need::*, Strategy::*};

        [
            Attachment + Assertive, Frustration + Assertive, Rejection + Assertive,
            Attachment + Compliant, Frustration + Compliant, Rejection + Compliant,
            Attachment + Withdrawn, Frustration + Withdrawn, Rejection + Withdrawn
        ]
    }
}

impl Add<Strategy> for Need
{
    type Output = ExternalConflict;

    fn add(self, rhs: Strategy) -> Self::Output
    {
        ExternalConflict {
            thesis: self,
            anti_thesis: rhs
        }
    }
}
impl Add<Need> for Strategy
{
    type Output = ExternalConflict;

    fn add(self, rhs: Need) -> Self::Output
    {
        ExternalConflict {
            thesis: rhs,
            anti_thesis: self
        }
    }
}

impl Domain for ExternalConflict
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
        &self.anti_thesis
    }
    fn subconscious(&self) -> &dyn Triad
    {
        &self.thesis
    }
    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{}, but {}", self.anti_thesis.expression(), self.thesis.expression())
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{}, because {}", self.anti_thesis.reflection(), self.thesis.reflection())
    }
}