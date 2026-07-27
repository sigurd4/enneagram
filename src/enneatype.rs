use core::{f64::consts::TAU};

use crate::{
    config::{EdgeConfig, EdgesConfig, EnneagramConfig, TriadsConfig, Fallback, Property},
    personality::Personality,
    pivot::Pivot,
    triad::Triad
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Enneatype
{
    Recovery = 1,
    Association = 2,
    Repression = 3,
    Rejection = 4,
    Catatonia = 5,
    Paranoia = 6,
    Disorganization = 7,
    Action = 8,
    Rest = 9
}

impl Enneatype
{
    pub fn new(number: u8) -> Self
    {
        assert_ne!(number, 0, "There is no enneatype with the number 0.");
        assert!((1..=9).contains(&number), "Enneagram numbers must be within the range of 1-9.");
        number
            .checked_sub(1)
            .and_then(|i| Self::all().get(i as usize).copied())
            .expect("Enneagram numbers must be within the range of 1-9.")
    }

    pub const fn all() -> &'static [Self; 9]
    {
        &[
            Self::Recovery,
            Self::Association,
            Self::Repression,
            Self::Rejection,
            Self::Catatonia,
            Self::Paranoia,
            Self::Disorganization,
            Self::Action,
            Self::Rest
        ]
    }

    pub fn config<'a>(&self, config: &'a (impl Property<EdgesConfig> + ?Sized), fallback: &'a Fallback) -> EdgeConfig<'a>
    {
        let edges = config.property(fallback);
        match self
        {
            Enneatype::Recovery => edges.recovery(fallback),
            Enneatype::Association => edges.association(fallback),
            Enneatype::Repression => edges.repression(fallback),
            Enneatype::Rejection => edges.rejection(fallback),
            Enneatype::Catatonia => edges.catatonia(fallback),
            Enneatype::Paranoia => edges.paranoia(fallback),
            Enneatype::Disorganization => edges.disorganization(fallback),
            Enneatype::Action => edges.action(fallback),
            Enneatype::Rest => edges.rest(fallback)
        }
    }

    pub fn number(&self) -> u8
    {
        let number = *self as u8;
        assert!((1..=9).contains(&number), "Enneagram numbers must be within the range of 1-9.");
        number
    }

    pub fn common_triads(edges: &[Enneatype]) -> Vec<Box<dyn Triad>>
    {
        crate::triad::all()
            .into_iter()
            .filter(|triad| {
                let triads_edges = triad.edges();
                edges.iter().all(|edge| triads_edges.contains(edge))
            })
            .collect::<Vec<_>>()
    }

    pub fn triads(&self) -> [Box<dyn Triad>; 4]
    {
        Self::common_triads(core::slice::from_ref(self))
            .try_into()
            .expect("Each enneatype must consist of exactly 4 triads.")
    }

    pub fn path(&self) -> impl Iterator<Item = Self>
    {
        let mut prev = None;
        let mut first = Some(*self);
        let mut edge = *self;
        core::iter::from_fn(move || {
            let first_one = first?;
            let pivot = edge.pivot();
            let extroverted = pivot.extroverted();
            let introverted = pivot.introverted();
            assert_ne!(extroverted, introverted);
            let prev = prev.replace(edge);
            let next = if prev == Some(introverted) { extroverted } else { introverted };
            assert!(next.pivot().is_adjacent_to(edge), "Coherence of the enneagram path is wrong");
            if next == first_one
            {
                first = None
            }
            Some(core::mem::replace(&mut edge, next))
        })
    }

    pub fn personality(&self) -> Personality
    {
        Personality::from_triads(self.triads().each_ref().map(|triad| &**triad))
    }

    pub fn angle(&self) -> f64
    {
        let number = self.number();
        number as f64 / 9.0 * TAU
    }

    pub fn position(&self) -> [f64; 2]
    {
        let angle = self.angle();
        let (sine, cosine) = angle.sin_cos();
        [sine, cosine]
    }

    pub fn info(&self, f: &mut core::fmt::Formatter, config: &(impl Property<EnneagramConfig> + ?Sized), fallback: &Fallback) -> core::fmt::Result
    {
        Self::common_info(core::slice::from_ref(self), f, config, fallback)
    }

    pub fn common_info(edges: &[Enneatype], f: &mut core::fmt::Formatter, config: &(impl Property<EnneagramConfig> + ?Sized), fallback: &Fallback) -> core::fmt::Result
    {
        let config = config.property(fallback);

        for edge in edges
        {
            let number = edge.number();
            let config = edge.config(config, fallback);
            writeln!(f, "\x1b[1;33mEnneagram {number} {}\x1b[0m", config.name)?;
        }

        for triad in Self::common_triads(edges)
        {
            let numbers = triad
                .edges()
                .into_iter()
                .map(|edge| edge.number())
                .map(|number| format!("{number}"))
                .collect::<String>();
            let kind = format!("\x1b[3;90m# {}:\x1b[0m", triad.kind(config, fallback));
            let config = triad.config(config, fallback);
            write!(f, "\n{kind}\n\x1b[0;97m{numbers} {}\x1b[0m", config.description)?;
        }
        Ok(())
    }

    pub fn affirmation(&self, f: &mut core::fmt::Formatter, config: &dyn Property<TriadsConfig>, fallback: &Fallback) -> core::fmt::Result
    {
        self.personality().affirmation(f, config, fallback)
    }

    pub fn pivot(&self) -> Pivot
    {
        Pivot::new(*self)
    }
}

#[cfg(test)]
mod test
{
    use crate::{config::{Config, Fallback}, enneatype::Enneatype};

    #[test]
    fn test_pos()
    {
        let pos = Enneatype::Repression.position();
        println!("{pos:?}")
    }

    #[test]
    fn test_path1()
    {
        let config = Config::default();
        let fallback = Fallback::default();

        for edge in Enneatype::Action.path()
        {
            println!("{}", edge.config(&config, &fallback).name)
        }
    }

    #[test]
    fn test_path2()
    {
        let config = Config::default();
        let fallback = Fallback::default();

        for edge in Enneatype::Repression.path()
        {
            println!("{}", edge.config(&config, &fallback).name)
        }
    }

    #[test]
    fn test_ordering()
    {
        let edges = Enneatype::all();
        assert!(edges.is_sorted())
    }
}
