use core::f64::consts::TAU;

use crate::{personality::Personality, pivot::Pivot, triad::Triad};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(enum_display::EnumDisplay)]
pub enum Edge
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

impl Edge
{
    pub fn new(number: u8) -> Self
    {
        assert_ne!(number, 0, "There is no enneagram edge with the number 0.");
        assert!((1..=9).contains(&number), "Enneagram numbers must be within the range of 1-9.");
        number.checked_sub(1)
            .and_then(|i| Self::all()
                .get(i as usize)
                .copied()
            ).expect("Enneagram numbers must be within the range of 1-9.")
    }

    pub fn all() -> [Self; 9]
    {
        [Self::Recovery, Self::Association, Self::Repression, Self::Rejection, Self::Catatonia, Self::Paranoia, Self::Disorganization, Self::Action, Self::Rest]
    }

    pub fn number(&self) -> u8
    {
        let number = *self as u8;
        assert!((1..=9).contains(&number), "Enneagram numbers must be within the range of 1-9.");
        number
    }

    pub fn common_triads(edges: &[Edge]) -> Vec<Box<dyn Triad>>
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
            .expect("Each personality must consist of exactly 4 triads.")
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
        number as f64/10.0*TAU
    }

    pub fn position(&self) -> [f64; 2]
    {
        let angle = self.angle();
        let (sine, cosine) = angle.sin_cos();
        [sine, cosine]
    }

    pub fn neighbours(&self) -> [Edge; 8]
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

    pub fn common_info(edges: &[Edge], f: &mut core::fmt::Formatter) -> core::fmt::Result
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
    use crate::edge::Edge;

    #[test]
    fn test_ordering()
    {
        let edges = Edge::all();
        assert!(edges.is_sorted())
    }

    #[test]
    fn test_neighbours()
    {
        for edge in Edge::all()
        {
            let neighbours = edge.neighbours();
            println!("{edge:?} -> {neighbours:?}")
        }
    }
}