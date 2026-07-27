use core::{any::Any, ops::Add};

use crate::{
    config::{DomainConfig, TriadsConfig, Fallback, Property},
    domain::Domain,
    triad::{Frame, Need, Triad}
};

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
            Gut + Attachment,
            Head + Attachment,
            Heart + Attachment,
            Gut + Frustration,
            Head + Frustration,
            Heart + Frustration,
            Gut + Rejection,
            Head + Rejection,
            Heart + Rejection
        ]
    }

    pub fn kind<'a>(config: &'a (impl Property<DomainConfig> + ?Sized), fallback: &'a Fallback) -> &'a str
    {
        config.property(fallback).desire_machine(fallback)
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

    fn kind<'a>(&self, config: &'a dyn Property<DomainConfig>, fallback: &'a Fallback) -> &'a str
    {
        Self::kind(config, fallback)
    }

    fn conscious(&self) -> &dyn Triad
    {
        &self.introverted
    }

    fn subconscious(&self) -> &dyn Triad
    {
        &self.extroverted
    }

    fn question(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Property<TriadsConfig>, fallback: &Fallback) -> core::fmt::Result
    {
        write!(
            f,
            "{} and {}",
            self.introverted.config(config, fallback).expression,
            self.extroverted.config(config, fallback).expression
        )
    }

    fn trivial(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Property<TriadsConfig>, fallback: &Fallback) -> core::fmt::Result
    {
        write!(
            f,
            "{} and {}",
            self.introverted.config(config, fallback).reflection,
            self.extroverted.config(config, fallback).reflection
        )
    }
}
