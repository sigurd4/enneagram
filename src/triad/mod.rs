use core::{any::Any, fmt::Debug};

use crate::edge::Edge;

moddef::moddef!(
    flat(pub) mod {
        fault,
        frame,
        need,
        strategy
    }
);

pub fn all() -> [Box<dyn Triad>; 4*3]
{
    let mut chain = core::iter::empty()
        .chain(
            Fault::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Triad>)
        ).chain(
            Frame::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Triad>)
        ).chain(
            Need::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Triad>)
        ).chain(
            Strategy::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Triad>)
        );
    let all = chain.next_chunk()
            .expect("The enneagram is defined by 4 triads each consisting of 3 states, 12 in total. Not enough states!");
    assert_eq!(chain.collect::<Vec<_>>().len(), 0, "The enneagram is defined by 4 triads each consisting of 3 states, 12 in total. Too many states!");
    all
}

pub trait Triad: Debug + Any
{
    fn edges(&self) -> &'static [Edge; 3];
    fn expression(&self) -> &'static str;
    fn reflection(&self) -> &'static str;
}

#[cfg(test)]
mod test
{
    use crate::triad::{Fault, Frame, Need, Strategy, Triad};

    #[test]
    fn test_frame()
    {
        test_triad(Frame::all());
    }
    #[test]
    fn test_strategy()
    {
        test_triad(Strategy::all());
    }
    #[test]
    fn test_fault()
    {
        test_triad(Fault::all());
    }
    #[test]
    fn test_need()
    {
        test_triad(Need::all());
    }

    #[test]
    fn test_all()
    {
        for triad in crate::triad::all()
        {
            println!("Q: {}\nA: {}\n", triad.expression(), triad.reflection());
        }
    }

    fn test_triad<T>(triads: [T; 3])
    where
        T: Triad
    {
        for triad in triads
        {
            println!("Q: {}\nA: {}\n", triad.expression(), triad.reflection());
        }
    }
}