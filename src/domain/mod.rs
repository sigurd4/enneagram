use core::{any::Any, fmt::Debug, ops::Add};

use crate::{Clause, edge::Edge, triad::{Fault, Frame, Need, Means, Triad}};

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

pub fn select() -> Box<dyn Domain>
{
    fn select_triads<T, N>(
        trivial_conjunction: &str,
        trivial: [T; 3],
        nontrivial_conjunction: &str,
        nontrivial: [N; 3]
    ) -> <T as Add<N>>::Output
    where
        T: Triad + Copy + Add<N, Output: Domain>,
        N: Triad + Copy
    {
        enum Triviality<T, N>
        {
            Trivial(T),
            Nontrivial(N)
        }
        
        let trivial_choices = trivial.map(|triad| (triad.expression(), move || triad));
        let nontrivial_choices = nontrivial.map(|triad| (triad.expression(), move || triad));

        let (domain_kind, codomain_kind) = {
            let [(_, lhs), ..] = trivial_choices;
            let [(_, rhs), ..] = nontrivial_choices;
            let domain = lhs() + rhs();
            (domain.kind(), domain.reciprocal().kind())
        };

        println!("\x1b[u\x1b 8 -> {codomain_kind}");

        let polymorphic_trivial_choices = trivial_choices.each_ref()
            .map(|(expression, generator)| (*expression, || Triviality::Trivial(generator())));
        let polymorphic_nontrivial_choices = nontrivial_choices.each_ref()
            .map(|(expression, generator)| (*expression, || Triviality::Nontrivial(generator())));

        let first_triad = crate::select(
            Clause::Question,
            &core::iter::chain(
                polymorphic_trivial_choices.each_ref()
                    .map(|(choice, generator)| (*choice, generator as &dyn Fn() -> Triviality<T, N>)),
                polymorphic_nontrivial_choices.each_ref()
                    .map(|(choice, generator)| (*choice, generator as &dyn Fn() -> Triviality<T, N>))
            ).collect::<Vec<_>>()
        );
        let (trivial_triad, nontrivial_triad) = match first_triad
        {
            Triviality::Trivial(trivial_triad) => {
                (
                    trivial_triad,
                    crate::select(
                        Clause::Continuation(nontrivial_conjunction),
                        &nontrivial_choices.each_ref()
                            .map(|(choice, generator)| (*choice, generator as &dyn Fn() -> N))
                    )
                )
            },
            Triviality::Nontrivial(nontrivial_triad) => {
                (
                    crate::select(
                        Clause::Continuation(trivial_conjunction),
                        &trivial_choices.each_ref()
                            .map(|(choice, generator)| (*choice, generator as &dyn Fn() -> T))
                    ),
                    nontrivial_triad
                )
            },
        };
        let domain = trivial_triad + nontrivial_triad;
        assert_eq!(domain.kind(), domain_kind, "Domain-kind must be invariant! (it isn't)");
        domain
    }

    let domain = crate::select::<Box<dyn Domain>>(
        Clause::Answer("please select a domain"),
        &[
            (InternalDissonance::kind(), &|| Box::new(select_triads(", but ", Frame::all(), ", but ", Means::all()))),
            (InternalConflict::kind(), &|| Box::new(select_triads(", but ", Frame::all(), ", ", Fault::all()))),
            (Suffering::kind(), &|| Box::new(select_triads(", ", Frame::all(), " and ", Need::all()))),
            (Behaviour::kind(), &|| Box::new(select_triads(", ", Fault::all(), " and ", Means::all()))),
            (ExternalConflict::kind(), &|| Box::new(select_triads(", but ", Need::all(), ", ", Means::all()))),
            (ExternalDissonance::kind(), &|| Box::new(select_triads(", but ", Need::all(), ", but ", Fault::all()))),
        ]
    );
    let answer = core::fmt::from_fn(|f| domain.answer(f));
    println!("A: {answer}");

    domain
}

pub fn all() -> [Box<dyn Domain>; 6*9]
{
    core::iter::empty()
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
        ).collect::<Vec<_>>()
        .try_into()
        .expect("The enneagram is defined by 54 unique domains. Wrong number of domains!")
}

pub trait Domain: Debug + Any + 'static
{
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, other: &dyn Domain) -> bool;

    fn kind(&self) -> &'static str;
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

    #[allow(unused)]
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