use core::ops::Add;

use crate::{domain::Domain, triad::{Frame, Need, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Suffering
{
    pub introverted: Frame,
    pub extroverted: Need
}

impl Suffering
{
    pub fn all() -> [Suffering; 9]
    {
        use {Frame::*, Need::*};

        [
            Gut + Attachment, Head + Attachment, Heart + Attachment,
            Gut + Frustration, Head + Frustration, Heart + Frustration,
            Gut + Rejection, Head + Rejection, Heart + Rejection,
        ]
    }
}

impl Add<Need> for Frame
{
    type Output = Suffering;

    fn add(self, rhs: Need) -> Self::Output
    {
        Suffering {
            introverted: self,
            extroverted: rhs
        }
    }
}
impl Add<Frame> for Need
{
    type Output = Suffering;

    fn add(self, rhs: Frame) -> Self::Output
    {
        Suffering {
            introverted: rhs,
            extroverted: self
        }
    }
}

impl Domain for Suffering
{
    fn conscious(&self) -> &dyn Triad
    {
        &self.introverted
    }
    fn subconscious(&self) -> &dyn Triad
    {
        &self.extroverted
    }
    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.introverted.expression(), self.extroverted.expression())
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.introverted.reflection(), self.extroverted.reflection())
    }
}