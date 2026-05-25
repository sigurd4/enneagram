use core::{any::Any, borrow::Borrow, ops::Add};

use crate::{config::{DomainConfig, TriadsConfig}, domain::Domain, triad::{Frame, Means, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InternalDissonance
{
    pub thesis: Frame,
    pub anti_thesis: Means
}

impl InternalDissonance
{
    pub fn all() -> [InternalDissonance; 9]
    {
        use {Frame::*, Means::*};

        [
            Gut + Assertive, Head + Assertive, Heart + Assertive,
            Gut + Compliant, Head + Compliant, Heart + Compliant,
            Gut + Withdrawn, Head + Withdrawn, Heart + Withdrawn,
        ]
    }

    pub fn kind<'a>(config: &'a (impl Borrow<DomainConfig> + ?Sized)) -> &'a str
    {
        config.borrow().introverted_dissonance()
    }
}

impl Add<Means> for Frame
{
    type Output = InternalDissonance;

    fn add(self, rhs: Means) -> Self::Output
    {
        InternalDissonance {
            thesis: self,
            anti_thesis: rhs
        }
    }
}
impl Add<Frame> for Means
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
    
    fn kind<'a>(&self, config: &'a dyn Borrow<DomainConfig>) -> &'a str
    {
        Self::kind(config)
    }
    fn conscious(&self) -> &dyn Triad
    {
        &self.thesis
    }
    fn subconscious(&self) -> &dyn Triad
    {
        &self.anti_thesis
    }
    fn question(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Borrow<TriadsConfig>) -> core::fmt::Result
    {
        write!(f, "{}, but {}", self.thesis.config(config).expression, self.anti_thesis.config(config).expression)
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Borrow<TriadsConfig>) -> core::fmt::Result
    {
        write!(f, "{}, but {}", self.thesis.config(config).reflection, self.anti_thesis.config(config).reflection)
    }
}