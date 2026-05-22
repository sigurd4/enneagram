use core::{any::Any, ops::Add};

use crate::{config::{DomainConfig, TriadsConfig}, domain::Domain, triad::{Fault, Frame, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InternalSynthesis
{
    pub thesis: Frame,
    pub anti_thesis: Fault
}

impl InternalSynthesis
{
    pub fn all() -> [InternalSynthesis; 9]
    {
        use {Frame::*, Fault::*};

        [
            Gut + Positive, Head + Positive, Heart + Positive,
            Gut + Competent, Head + Competent, Heart + Competent,
            Gut + Reactive, Head + Reactive, Heart + Reactive,
        ]
    }

    pub fn kind<'a>(config: DomainConfig<'a>) -> &'a str
    {
        config.introverted_synthesis
    }
}

impl Add<Fault> for Frame
{
    type Output = InternalSynthesis;

    fn add(self, rhs: Fault) -> Self::Output
    {
        InternalSynthesis {
            thesis: self,
            anti_thesis: rhs
        }
    }
}
impl Add<Frame> for Fault
{
    type Output = InternalSynthesis;

    fn add(self, rhs: Frame) -> Self::Output
    {
        InternalSynthesis {
            thesis: rhs,
            anti_thesis: self
        }
    }
}

impl Domain for InternalSynthesis
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn equals(&self, other: &dyn Domain) -> bool
    {
        other.as_any().downcast_ref().is_some_and(|other| self == other)
    }
    
    fn kind<'a>(&self, config: DomainConfig<'a>) -> &'a str
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
    fn question(&self, f: &mut core::fmt::Formatter<'_>, config: TriadsConfig<'_>) -> core::fmt::Result
    {
        write!(f, "{}, but {}", self.thesis.config(config).expression, self.anti_thesis.config(config).expression)
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>, config: TriadsConfig<'_>) -> core::fmt::Result
    {
        write!(f, "{}, so {}", self.thesis.config(config).reflection, self.anti_thesis.config(config).reflection)
    }
}