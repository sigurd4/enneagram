use core::{any::Any, borrow::Borrow, fmt::Debug, ops::Add};

use crate::{Clause, config::{DomainConfig, EnneagramConfig, TriadsConfig}, enneatype::Enneatype, triad::{Fault, Frame, Means, Need, Triad}};

moddef::moddef!(
    flat(pub) mod {
        extroverted_dissonance,
        extroverted_synthesis,
        body_without_organs,
        desire_machine,
        introverted_synthesis,
        introverted_dissonance
    }
);

pub fn select(config: &(impl Borrow<EnneagramConfig> + ?Sized)) -> Box<dyn Domain>
{
    let config = config.borrow();
    fn select_triads<T, N>(
        trivial_conjunction: &str,
        trivial: [T; 3],
        nontrivial_conjunction: &str,
        nontrivial: [N; 3],
        config: &EnneagramConfig
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
        
        let trivial_choices = trivial.each_ref().map(|triad| (triad.config(config), move || *triad));
        let nontrivial_choices = nontrivial.each_ref().map(|triad| (triad.config(config), move || *triad));

        let (domain_kind, codomain_kind) = {
            let [(_, lhs), ..] = trivial_choices;
            let [(_, rhs), ..] = nontrivial_choices;
            let domain = lhs() + rhs();
            (domain.kind(config), domain.reciprocal().kind(config))
        };

        println!("\x1b[u -> {codomain_kind}");

        let polymorphic_trivial_choices = trivial_choices.each_ref()
            .map(|(config, generator)| (config.expression.as_ref(), || Triviality::Trivial(generator())));
        let polymorphic_nontrivial_choices = nontrivial_choices.each_ref()
            .map(|(config, generator)| (config.expression.as_ref(), || Triviality::Nontrivial(generator())));

        let first_triad = crate::select(
            Clause::Question,
            &core::iter::chain(
                polymorphic_trivial_choices.each_ref()
                    .map(|(choice, generator)| (choice.as_ref(), generator as &dyn Fn() -> Triviality<T, N>)),
                polymorphic_nontrivial_choices.each_ref()
                    .map(|(choice, generator)| (choice.as_ref(), generator as &dyn Fn() -> Triviality<T, N>))
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
                            .map(|(choice, generator)| (choice.expression.as_ref(), generator as &dyn Fn() -> N))
                    )
                )
            },
            Triviality::Nontrivial(nontrivial_triad) => {
                (
                    crate::select(
                        Clause::Continuation(trivial_conjunction),
                        &trivial_choices.each_ref()
                            .map(|(choice, generator)| (choice.expression.as_ref(), generator as &dyn Fn() -> T))
                    ),
                    nontrivial_triad
                )
            },
        };
        let domain = trivial_triad + nontrivial_triad;
        assert_eq!(domain.kind(config), domain_kind, "Domain-kind must be invariant! (it isn't)");
        domain
    }

    let domain = crate::select::<Box<dyn Domain>>(
        Clause::Answer("please select a domain"),
        &[
            (InternalDissonance::kind(config), &|| Box::new(select_triads(", but ", Frame::all(), ", but ", Means::all(), config))),
            (InternalSynthesis::kind(config), &|| Box::new(select_triads(", but ", Frame::all(), ", ", Fault::all(), config))),
            (DesireMachine::kind(config), &|| Box::new(select_triads(", ", Frame::all(), " and ", Need::all(), config))),
            (BodyWithoutOrgans::kind(config), &|| Box::new(select_triads(", ", Fault::all(), " and ", Means::all(), config))),
            (ExternalSynthesis::kind(config), &|| Box::new(select_triads(", but ", Need::all(), ", ", Means::all(), config))),
            (ExternalDissonance::kind(config), &|| Box::new(select_triads(", but ", Need::all(), ", but ", Fault::all(), config))),
        ]
    );
    let answer = core::fmt::from_fn(|f| domain.answer(f, config));
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
            ExternalSynthesis::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Domain>)
        ).chain(
            BodyWithoutOrgans::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Domain>)
        ).chain(
            DesireMachine::all()
                .into_iter()
                .map(|domain| Box::new(domain) as Box<dyn Domain>)
        ).chain(
            InternalSynthesis::all()
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

    fn kind<'a>(&self, config: &'a dyn Borrow<DomainConfig>) -> &'a str;
    fn conscious(&self) -> &dyn Triad;
    fn subconscious(&self) -> &dyn Triad;
    fn triads(&self) -> [&dyn Triad; 2]
    {
        [self.conscious(), self.subconscious()]
    }
    fn edge(&self) -> Enneatype
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
    fn question(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Borrow<TriadsConfig>) -> core::fmt::Result;
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Borrow<TriadsConfig>) -> core::fmt::Result;
    fn answer(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Borrow<TriadsConfig>) -> core::fmt::Result
    {
        self.reciprocal().trivial(f, config)
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
    use crate::{config::Config, domain::{BodyWithoutOrgans, DesireMachine, Domain, ExternalDissonance, ExternalSynthesis, InternalDissonance, InternalSynthesis}};

    #[test]
    fn test_external_dissonance()
    {
        test_domain(ExternalDissonance::all());
    }

    #[test]
    fn test_external_conflict()
    {
        test_domain(ExternalSynthesis::all());
    }
    
    #[test]
    fn test_behaviour()
    {
        test_domain(BodyWithoutOrgans::all());
    }

    #[test]
    fn test_suffering()
    {
        test_domain(DesireMachine::all());
    }
    
    #[test]
    fn test_internal_conflict()
    {
        test_domain(InternalSynthesis::all());
    }

    #[test]
    fn test_internal_dissonance()
    {
        test_domain(InternalDissonance::all());
    }

    #[test]
    fn test_all()
    {
        let config = Config::default();

        for domain in crate::domain::all()
        {
            let q = std::fmt::from_fn(|f| domain.question(f, &config));
            let a = std::fmt::from_fn(|f| domain.answer(f, &config));
            let e = domain.edge().config(&config);
            let e = e.name.as_ref();
            println!("Q: {q}\nA: {a}\nE: {e}\n");
        }
    }

    fn test_domain<T>(domains: [T; 9])
    where
        T: Domain
    {
        let config = Config::default();

        for domain in domains
        {
            let q = std::fmt::from_fn(|f| domain.question(f, &config));
            let a = std::fmt::from_fn(|f| domain.answer(f, &config));
            let e = domain.edge().config(&config);
            let e = e.name.as_ref();
            println!("Q: {q}\nA: {a}\nE: {e}\n");
        }
    }
}