use core::{any::Any, fmt::{Debug, Display}};

use crate::enneatype::Enneatype;

moddef::moddef!(
    flat(pub) mod {
        fault,
        frame,
        need,
        means
    }
);

pub fn triangulate(edges: &[Enneatype; 3]) -> Box<dyn Triad>
{
    let [triad] = core::iter::empty()
        .chain(
            Fault::all()
                .into_iter()
                .filter(|triad| triad.edges() == edges)
                .map(|triad| Box::new(triad) as Box<dyn Triad>)
        ).chain(
            Frame::all()
                .into_iter()
                .filter(|triad| triad.edges() == edges)
                .map(|triad| Box::new(triad) as Box<dyn Triad>)
        ).chain(
            Need::all()
                .into_iter()
                .filter(|triad| triad.edges() == edges)
                .map(|triad| Box::new(triad) as Box<dyn Triad>)
        ).chain(
            Means::all()
                .into_iter()
                .filter(|triad| triad.edges() == edges)
                .map(|triad| Box::new(triad) as Box<dyn Triad>)
        ).collect::<Vec<_>>()
        .try_into()
        .expect("Exactly one triad should match the set of three edges.");
    triad
}

pub fn all() -> [Box<dyn Triad>; 4*3]
{
    core::iter::empty()
        .chain(
            Fault::all()
                .into_iter()
                .map(|triad| Box::new(triad) as Box<dyn Triad>)
        ).chain(
            Frame::all()
                .into_iter()
                .map(|triad| Box::new(triad) as Box<dyn Triad>)
        ).chain(
            Need::all()
                .into_iter()
                .map(|triad| Box::new(triad) as Box<dyn Triad>)
        ).chain(
            Means::all()
                .into_iter()
                .map(|triad| Box::new(triad) as Box<dyn Triad>)
        ).collect::<Vec<_>>()
        .try_into()
        .expect("The enneagram is defined by 4 triads each consisting of 3 states, 12 in total. Wrong number of states!")
}

pub trait Triad: Debug + Display + Any
{
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, other: &dyn Triad) -> bool;

    fn edges(&self) -> &'static [Enneatype; 3];
    fn expression(&self) -> &'static str;
    fn reflection(&self) -> &'static str;
    fn affirmation(&self) -> &'static str;

    fn lines(&self) -> [[Enneatype; 2]; 3]
    {
        let edges @ &[.., mut prev] = self.edges();
        edges.map(|edge| [core::mem::replace(&mut prev, edge), edge])
    }
}

#[cfg(test)]
mod test
{
    use crate::triad::{Fault, Frame, Need, Means, Triad};

    #[test]
    fn test_frame()
    {
        test_triads(&Frame::all());
    }
    #[test]
    fn test_strategy()
    {
        test_triads(&Means::all());
    }
    #[test]
    fn test_fault()
    {
        test_triads(&Fault::all());
    }
    #[test]
    fn test_need()
    {
        test_triads(&Need::all());
    }

    #[test]
    fn test_all()
    {
        for triad in &crate::triad::all()
        {
            test_triad(&**triad);
        }
    }

    fn test_triad<T>(triad: &T)
    where
        T: Triad + ?Sized
    {
        println!("Q: {}\nA: {}\n", triad.expression(), triad.reflection());
        let edges = triad.edges();
        let reconstruction = crate::triad::triangulate(edges);
        assert!(triad.equals(&*reconstruction), "Triad must match its own reconstruction!")
    }

    fn test_triads<'a, T>(triads: impl IntoIterator<Item = &'a T>)
    where
        T: Triad
    {
        for triad in triads
        {
            test_triad(triad);
        }
    }
}