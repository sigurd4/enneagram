use crate::triad::{Fault, Frame, Need, Means, Triad};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Personality
{
    pub frame: Frame,
    pub strategy: Means,
    pub fault: Fault,
    pub need: Need
}

impl Personality
{
    pub fn affirmation(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result
    {
        write!(f, "i will {}, {}, and {}.",
            self.frame.affirmation(),
            self.fault.affirmation(),
            self.strategy.affirmation()
        )
    }

    pub fn from_triads(triads: [&dyn Triad; 4]) -> Self
    {
        let (frame, strategy, fault, need) = triads.into_iter()
            .map(|triad| {
                let any = triad.as_any();
                any.downcast_ref()
                    .map(|frame| (Some(frame), None, None, None))
                    .or_else(|| any.downcast_ref()
                        .map(|strategy| (None, Some(strategy), None, None))
                    ).or_else(|| any.downcast_ref()
                        .map(|fault| (None, None, Some(fault), None))
                    ).or_else(|| any.downcast_ref()
                        .map(|need| (None, None, None, Some(need)))
                    ).expect("The triad type is not r{ecognized. Something must have gone horribly wrong!")
            }).reduce(|lhs, rhs| (
                lhs.0.map(Some).xor(rhs.0.map(Some)).map(|inner| inner.expect("Unsound personality. Conflicting personality frame")),
                lhs.1.map(Some).xor(rhs.1.map(Some)).map(|inner| inner.expect("Unsound personality. Conflicting personality strategy")),
                lhs.2.map(Some).xor(rhs.2.map(Some)).map(|inner| inner.expect("Unsound personality. Conflicting personality fault")),
                lhs.3.map(Some).xor(rhs.3.map(Some)).map(|inner| inner.expect("Unsound personality. Conflicting personality need")),
            )).expect("Unsound personality, No triads given. Impossible!");
        Personality {
            frame: *frame.expect("Unsound personality. Undefined personality frame"),
            strategy: *strategy.expect("Unsound personality. Undefined personality strategy"),
            fault: *fault.expect("Unsound personality. Undefined personality fault"),
            need: *need.expect("Unsound personality. Undefined personality need")
        }
    }
}