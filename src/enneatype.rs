use core::f64::consts::TAU;

#[cfg(feature = "artwork")]
use ratatui_3d::{Mesh, Vertex};

use crate::{personality::Personality, pivot::Pivot, triad::Triad};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(enum_display::EnumDisplay)]
pub enum Enneatype
{
    #[display("Recovery/Gradient")]
    Recovery = 1,
    #[display("Association/Superego")]
    Association = 2,
    #[display("Repression/Ego")]
    Repression = 3,
    #[display("Rejection/Id")]
    Rejection = 4,
    #[display("Catatonia")]
    Catatonia = 5,
    #[display("Paranoia")]
    Paranoia = 6,
    #[display("Disorganization")]
    Disorganization = 7,
    #[display("Action/Flow")]
    Action = 8,
    #[display("Rest/Equilibrium")]
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

    pub const fn all() -> [Self; 9]
    {
        [Self::Recovery, Self::Association, Self::Repression, Self::Rejection, Self::Catatonia, Self::Paranoia, Self::Disorganization, Self::Action, Self::Rest]
    }

    pub fn digit(&self) -> &'static [[i8; 2]]
    {
        match self
        {
            Enneatype::Recovery => &[[-1, 3], [0, 4], [0, -4], [-1, -4], [1, -4]],
            Enneatype::Association => &[[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 0], [-3, -4], [3, -4], [3, -3]],
            Enneatype::Repression => &[[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [1, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3]],
            Enneatype::Rejection => &[[3, 1], [-3, 1], [1, 4], [1, -4]],
            Enneatype::Catatonia => &[[3, 4], [-3, 4], [-3, 1], [2, 1], [3, 0], [3, -3], [2, -4], [-3, -4]],
            Enneatype::Paranoia => &[[3, 4], [0, 4], [-3, -1], [-2, 0], [2, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3], [-3, -1]],
            Enneatype::Disorganization => &[[-3, 4], [3, 4], [-3, -4]],
            Enneatype::Action => &[[1, 0], [3, 1], [3, 3], [2, 4], [-2, 4], [-3, 3], [-3, 1], [-1, 0], [-3, -1], [-3, -3], [-2, -4], [2, -4], [3, -3], [3, -1], [1, 0], [-1, 0]],
            Enneatype::Rest => &[[3, 1], [2, 0], [-2, 0], [-3, 1], [-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [-3, -4]],
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

    pub fn neighbours(&self) -> [Enneatype; 8]
    {
        self.triads()
            .into_iter()
            .filter_map(|triad| match triad.edges()
                {
                    [this, a, b] | [a, this, b] | [a, b, this] if this == self => Some([*a, *b]),
                    _ => None
                }
            ).flatten()
            .collect::<Vec<_>>()
            .try_into()
            .expect("There must be exactly 8 neighbouring edges!")
    }

    pub fn info(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result
    {
        Self::common_info(core::slice::from_ref(self), f)
    }

    pub fn common_info(edges: &[Enneatype], f: &mut core::fmt::Formatter) -> core::fmt::Result
    {
        for edge in edges
        {
            let number = edge.number();
            writeln!(f, "Enneagram {number} {edge}")?;
        }

        for triad in Self::common_triads(edges)
        {
            let numbers = triad.edges()
                .into_iter()
                .map(|edge| edge.number())
                .map(|number| format!("{number}"))
                .collect::<String>();
            write!(f, "\n{numbers} {triad}")?;
        }
        Ok(())
    }

    pub fn affirmation(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result
    {
        self.personality()
            .affirmation(f)
    }

    pub fn pivot(&self) -> Pivot
    {
        Pivot::new(*self)
    }
}

#[cfg(test)]
mod test
{
    use crate::enneatype::Enneatype;

    #[test]
    fn test_pos()
    {
        let pos = Enneatype::Repression.position();
        println!("{pos:?}")
    }

    #[test]
    fn test_path1()
    {
        for edge in Enneatype::Action.path()
        {
            println!("{edge}")
        }
    }

    #[test]
    fn test_path2()
    {
        for edge in Enneatype::Repression.path()
        {
            println!("{edge}")
        }
    }

    #[test]
    fn test_ordering()
    {
        let edges = Enneatype::all();
        assert!(edges.is_sorted())
    }

    #[test]
    fn test_neighbours()
    {
        for edge in Enneatype::all()
        {
            let neighbours = edge.neighbours();
            println!("{edge:?} -> {neighbours:?}")
        }
    }
}