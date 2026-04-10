use core::{any::Any, fmt::Debug};

use crate::{edge::Edge, triad::{Fault, Frame, Need, Strategy, Triad}};

moddef::moddef!(
    flat(pub) mod {
        external_dissonance,
        external_conflict,
        behaviour,
        suffering,
        internal_conflict,
        internal_dissonance
    }
);

enum Duality
{
    Thesis,
    AntiThesis
}

enum Directionality
{
    Introverted,
    Extroverted
}

pub fn select() -> Box<dyn Domain>
{
    fn select_triad<T>(all: [T; 3]) -> T
    where
        T: Triad + Copy
    {
        let choices = all.map(|triad| (triad.expression(), move || triad));

        crate::select::<T>(
            None,
            &choices.each_ref()
                .map(|(choice, generator)| (*choice, generator as &dyn Fn() -> T))
        )
    }

    crate::select(
        Some("please select a domain"),
        &[
            (ExternalDissonance::kind(), &|| Box::new(ExternalDissonance {
                anti_thesis: select_triad(Fault::all()),
                thesis: select_triad(Need::all())
            })),
            (InternalConflict::kind(), &|| Box::new(InternalConflict {
                thesis: select_triad(Frame::all()),
                anti_thesis: select_triad(Fault::all())
            })),
            (Suffering::kind(), &|| Box::new(Suffering {
                introverted: select_triad(Frame::all()),
                extroverted: select_triad(Need::all())
            })),
            (Behaviour::kind(), &|| Box::new(Behaviour {
                introverted: select_triad(Fault::all()),
                extroverted: select_triad(Strategy::all())
            })),
            (ExternalConflict::kind(), &|| Box::new(ExternalConflict {
                thesis: select_triad(Need::all()),
                anti_thesis: select_triad(Strategy::all())
            })),
            (ExternalDissonance::kind(), &|| Box::new(ExternalDissonance {
                anti_thesis: select_triad(Fault::all()),
                thesis: select_triad(Need::all())
            })),
        ]
    )
}

pub fn all() -> [Box<dyn Domain>; 6*9]
{
    let mut chain = core::iter::empty()
        .chain(
            ExternalDissonance::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Domain>)
        ).chain(
            ExternalConflict::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Domain>)
        ).chain(
            Behaviour::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Domain>)
        ).chain(
            Suffering::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Domain>)
        ).chain(
            InternalConflict::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Domain>)
        ).chain(
            InternalDissonance::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Domain>)
        );
    let all = chain.next_chunk()
            .expect("The enneagram is defined by 54 unique domains. Not enough domains!");
    assert_eq!(chain.collect::<Vec<_>>().len(), 0, "The enneagram is defined by 54 unique domains. Too many domains!");
    all
}

pub trait Domain: Debug + Any + 'static
{
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, other: &dyn Domain) -> bool;

    fn conscious(&self) -> &dyn Triad;
    fn subconscious(&self) -> &dyn Triad;
    fn triads(&self) -> [&dyn Triad; 2]
    {
        [self.conscious(), self.subconscious()]
    }
    fn edge(&self) -> Edge
    {
        let triads = self.triads();
        let mut edges = triads.into_iter()
            .map(|triad| triad.edges().map(|edge| Some(edge)))
            .reduce(|mut triad, other_triad| {
                for edge in triad.iter_mut()
                    .filter(|edge| edge.is_some() && !other_triad.contains(edge))
                {
                    *edge = None
                }
                triad
            }).into_iter()
            .flatten()
            .filter_map(|edge| edge);
        let edge = edges.next().expect("The conscious and the subconscious must agree on a single common personality! No agreement");
        assert_eq!({
            let mut rest = edges.collect::<Vec<_>>();
            rest.dedup();
            rest
        }, [], "The conscious and the subconscious must agree on a single common personality! Ambiguous overlap");
        edge
    }

    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
    fn answer(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        self.reciprocal().trivial(f)
    }

    fn reciprocal(&self) -> Box<dyn Domain>
    {
        // This is dumb but should work
        let edge = self.edge();
        let triads = self.triads();
        let mut codomains = crate::domain::all()
            .into_iter()
            .filter(|domain| !self.equals(&**domain)
                && edge == domain.edge()
                && !domain.triads()
                    .into_iter()
                    .any(|other_triad| triads.iter().any(|triad| triad.equals(other_triad)))
            );
        let codomain = codomains.next().expect("This domain has no reciprocal codomain!");
        assert_eq!(codomains.collect::<Vec<_>>().len(), 0, "The reciprocal codomain of this domain cannot be ambiguous!");
        codomain
    }
}

#[cfg(test)]
mod test
{
    use crate::domain::{Behaviour, Domain, ExternalConflict, ExternalDissonance, InternalConflict, InternalDissonance, Suffering};

    #[test]
    fn test_external_dissonance()
    {
        test_domain(ExternalDissonance::all());
    }

    #[test]
    fn test_external_conflict()
    {
        test_domain(ExternalConflict::all());
    }
    
    #[test]
    fn test_behaviour()
    {
        test_domain(Behaviour::all());
    }

    #[test]
    fn test_suffering()
    {
        test_domain(Suffering::all());
    }
    
    #[test]
    fn test_internal_conflict()
    {
        test_domain(InternalConflict::all());
    }

    #[test]
    fn test_internal_dissonance()
    {
        test_domain(InternalDissonance::all());
    }

    #[test]
    fn test_all()
    {
        for domain in crate::domain::all()
        {
            let q = std::fmt::from_fn(|f| domain.question(f));
            let a = std::fmt::from_fn(|f| domain.answer(f));
            let e = domain.edge();
            println!("Q: {q}\nA: {a}\nE: {e}\n");
        }
    }

    fn test_domain<T>(domains: [T; 9])
    where
        T: Domain
    {
        for domain in domains
        {
            let q = std::fmt::from_fn(|f| domain.question(f));
            let a = std::fmt::from_fn(|f| domain.answer(f));
            let e = domain.edge();
            println!("Q: {q}\nA: {a}\nE: {e}\n");
        }
    }
}