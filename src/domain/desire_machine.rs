use core::{any::Any, borrow::Borrow, ops::Add};

use crate::{config::{DomainConfig, TriadsConfig}, domain::Domain, triad::{Frame, Need, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesireMachine
{
    pub introverted: Frame,
    pub extroverted: Need
}

impl DesireMachine
{
    pub fn all() -> [DesireMachine; 9]
    {
        use {Frame::*, Need::*};

        [
            Gut + Attachment, Head + Attachment, Heart + Attachment,
            Gut + Frustration, Head + Frustration, Heart + Frustration,
            Gut + Rejection, Head + Rejection, Heart + Rejection,
        ]
    }

    pub fn kind<'a>(config: &'a (impl Borrow<DomainConfig> + ?Sized)) -> &'a str
    {
        config.borrow().desire_machine()
    }
}

impl Add<Need> for Frame
{
    type Output = DesireMachine;

    fn add(self, rhs: Need) -> Self::Output
    {
        DesireMachine {
            introverted: self,
            extroverted: rhs
        }
    }
}
impl Add<Frame> for Need
{
    type Output = DesireMachine;

    fn add(self, rhs: Frame) -> Self::Output
    {
        DesireMachine {
            introverted: rhs,
            extroverted: self
        }
    }
}

impl Domain for DesireMachine
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
        &self.introverted
    }
    fn subconscious(&self) -> &dyn Triad
    {
        &self.extroverted
    }
    fn question(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Borrow<TriadsConfig>) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.introverted.config(config).expression, self.extroverted.config(config).expression)
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Borrow<TriadsConfig>) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.introverted.config(config).reflection, self.extroverted.config(config).reflection)
    }
}