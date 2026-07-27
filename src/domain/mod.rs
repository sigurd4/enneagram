use core::{any::Any, fmt::Debug};

use crate::{
    config::{DomainConfig, Fallback, Property, TriadsConfig}, enneatype::Enneatype, triad::Triad
};

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

pub fn all() -> [Box<dyn Domain>; 6 * 9]
{
    core::iter::empty()
        .chain(ExternalDissonance::all().into_iter().map(|domain| Box::new(domain) as Box<dyn Domain>))
        .chain(ExternalSynthesis::all().into_iter().map(|domain| Box::new(domain) as Box<dyn Domain>))
        .chain(BodyWithoutOrgans::all().into_iter().map(|domain| Box::new(domain) as Box<dyn Domain>))
        .chain(DesireMachine::all().into_iter().map(|domain| Box::new(domain) as Box<dyn Domain>))
        .chain(InternalSynthesis::all().into_iter().map(|domain| Box::new(domain) as Box<dyn Domain>))
        .chain(InternalDissonance::all().into_iter().map(|domain| Box::new(domain) as Box<dyn Domain>))
        .collect::<Vec<_>>()
        .try_into()
        .expect("The enneagram is defined by 54 unique domains. Wrong number of domains!")
}

pub trait Domain: Debug + Any + 'static
{
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, other: &dyn Domain) -> bool;

    fn kind<'a>(&self, config: &'a dyn Property<DomainConfig>, fallback: &'a Fallback) -> &'a str;
    fn conscious(&self) -> &dyn Triad;
    fn subconscious(&self) -> &dyn Triad;
    fn triads(&self) -> [&dyn Triad; 2]
    {
        [self.conscious(), self.subconscious()]
    }
    fn edge(&self) -> Enneatype
    {
        let triads = self.triads();
        let mut edges = triads
            .into_iter()
            .map(|triad| triad.edges().map(Some))
            .reduce(|mut triad, other_triad| {
                for edge in triad.iter_mut().filter(|edge| edge.is_some() && !other_triad.contains(edge))
                {
                    *edge = None
                }
                triad
            })
            .into_iter()
            .flatten()
            .flatten();
        let edge = edges
            .next()
            .expect("The conscious and the subconscious must agree on a single common personality! No agreement");
        assert_eq!(
            {
                let mut rest = edges.collect::<Vec<_>>();
                rest.dedup();
                rest
            },
            [],
            "The conscious and the subconscious must agree on a single common personality! Ambiguous overlap"
        );
        edge
    }

    #[allow(unused)]
    fn question(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Property<TriadsConfig>, fallback: &Fallback) -> core::fmt::Result;
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Property<TriadsConfig>, fallback: &Fallback) -> core::fmt::Result;
    fn answer(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Property<TriadsConfig>, fallback: &Fallback) -> core::fmt::Result
    {
        self.reciprocal().trivial(f, config, fallback)
    }

    fn reciprocal(&self) -> Box<dyn Domain>
    {
        // This is dumb but should work
        let edge = self.edge();
        let triads = self.triads();
        let mut codomains = crate::domain::all().into_iter().filter(|domain| {
            !self.equals(&**domain) && edge == domain.edge() && !domain.triads().into_iter().any(|other_triad| triads.iter().any(|triad| triad.equals(other_triad)))
        });
        let codomain = codomains.next().expect("This domain has no reciprocal codomain!");
        assert_eq!(codomains.collect::<Vec<_>>().len(), 0, "The reciprocal codomain of this domain cannot be ambiguous!");
        codomain
    }
}

#[cfg(test)]
mod test
{
    use crate::{
        config::{Config, Fallback},
        domain::{BodyWithoutOrgans, DesireMachine, Domain, ExternalDissonance, ExternalSynthesis, InternalDissonance, InternalSynthesis}
    };

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
        let fallback = Fallback::default();

        for domain in crate::domain::all()
        {
            let q = std::fmt::from_fn(|f| domain.question(f, &config, &fallback));
            let a = std::fmt::from_fn(|f| domain.answer(f, &config, &fallback));
            let e = domain.edge().config(&config, &fallback);
            let e = e.name.as_ref();
            println!("Q: {q}\nA: {a}\nE: {e}\n");
        }
    }

    fn test_domain<T>(domains: [T; 9])
    where
        T: Domain
    {
        let config = Config::default();
        let fallback = Fallback::default();

        for domain in domains
        {
            let q = std::fmt::from_fn(|f| domain.question(f, &config, &fallback));
            let a = std::fmt::from_fn(|f| domain.answer(f, &config, &fallback));
            let e = domain.edge().config(&config, &fallback);
            let e = e.name.as_ref();
            println!("Q: {q}\nA: {a}\nE: {e}\n");
        }
    }
}
