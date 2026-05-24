use core::{borrow::Borrow, f64::consts::TAU};

use crate::{config::{EdgeConfig, EdgesConfig, EnneagramConfig, TriadsConfig}, personality::Personality, pivot::Pivot, triad::Triad};

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
        number.checked_sub(1)
            .and_then(|i| Self::all()
                .get(i as usize)
                .copied()
            ).expect("Enneagram numbers must be within the range of 1-9.")
    }

    pub const fn all() -> &'static [Self; 9]
    {
        &[Self::Recovery, Self::Association, Self::Repression, Self::Rejection, Self::Catatonia, Self::Paranoia, Self::Disorganization, Self::Action, Self::Rest]
    }

    pub fn config<'a>(&self, config: &'a (impl Borrow<EdgesConfig> + ?Sized)) -> &'a EdgeConfig
    {
        let edges = config.borrow();
        match self
        {
            Enneatype::Recovery => &edges.recovery,
            Enneatype::Association => &edges.association,
            Enneatype::Repression => &edges.repression,
            Enneatype::Rejection => &edges.rejection,
            Enneatype::Catatonia => &edges.catatonia,
            Enneatype::Paranoia => &edges.paranoia,
            Enneatype::Disorganization => &edges.disorganization,
            Enneatype::Action => &edges.action,
            Enneatype::Rest => &edges.rest,
        }
    }

    pub fn number(&self) -> u8
    {
        let number = *self as u8;
        assert!((1..=9).contains(&number), "Enneagram numbers must be within the range of 1-9.");
        number
    }
    pub fn index(&self) -> usize
    {
        self.number()
            .checked_sub(1)
            .expect("Enneagram numbers must start at 1.") as usize
    }

    pub fn common_triads(edges: &[Enneatype]) -> Vec<Box<dyn Triad>>
    {
        crate::triad::all()
            .into_iter()
            .filter(|triad| {
                let triads_edges = triad.edges();
                edges.iter()
                    .all(|edge| triads_edges.contains(edge))
            }).collect::<Vec<_>>()
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
            let next = if prev == Some(introverted)
            {
                extroverted
            }
            else
            {
                introverted
            };
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
        Personality::from_triads(
            self.triads()
                .each_ref()
                .map(|triad| &**triad)
        )
    }

    pub fn angle(&self) -> f64
    {
        let number = self.number();
        number as f64/9.0*TAU
    }

    pub fn position(&self) -> [f64; 2]
    {
        let angle = self.angle();
        let (sine, cosine) = angle.sin_cos();
        [sine, cosine]
    }

    pub fn info(&self, f: &mut core::fmt::Formatter, config: &(impl Borrow<EnneagramConfig> + ?Sized)) -> core::fmt::Result
    {
        Self::common_info(core::slice::from_ref(self), f, config)
    }

    pub fn common_info(edges: &[Enneatype], f: &mut core::fmt::Formatter, config: &(impl Borrow<EnneagramConfig> + ?Sized)) -> core::fmt::Result
    {
        let config = config.borrow();

        for edge in edges
        {
            let number = edge.number();
            let config = edge.config(config);
            writeln!(f, "Enneagram {number} {}", config.name)?;
        }

        for triad in Self::common_triads(edges)
        {
            let numbers = triad.edges()
                .into_iter()
                .map(|edge| edge.number())
                .map(|number| format!("{number}"))
                .collect::<String>();
            let config = triad.config(config);
            write!(f, "\n{numbers} {}", config.description)?;
        }
        Ok(())
    }

    pub fn affirmation(&self, f: &mut core::fmt::Formatter, config: &dyn Borrow<TriadsConfig>) -> core::fmt::Result
    {
        self.personality()
            .affirmation(f, config)
    }

    pub fn pivot(&self) -> Pivot
    {
        Pivot::new(*self)
    }
}

#[cfg(test)]
mod test
{
    use crate::{config::Config, enneatype::Enneatype};

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

        for edge in Enneatype::Action.path()
        {
            println!("{}", edge.config(&config).name)
        }
    }

    #[test]
    fn test_path2()
    {
        let config = Config::default();

        for edge in Enneatype::Repression.path()
        {
            println!("{}", edge.config(&config).name)
        }
    }

    #[test]
    fn test_ordering()
    {
        let edges = Enneatype::all();
        assert!(edges.is_sorted())
    }
}