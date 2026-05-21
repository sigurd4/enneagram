use crate::{Corner, Point};

#[derive(Clone, Copy, Debug)]
pub struct Line<P>(pub P, pub P)
where
    P: Point;

impl<P> Line<P>
where
    P: Point
{
    pub fn corner(self, other: Line<P>) -> Option<Corner<P>>
    where
        P: PartialEq
    {
        Corner::between(self, other)
    }

    pub fn distance(self) -> P::Distance
    {
        let Self(a, b) = self;
        a.distance_to(b)
    }

    pub fn square_magnitude(self) -> <P::Distance as Point>::Magnitude
    {
        let Self(a, b) = self;
        a.distance_square_magnitude(b)
    }

    pub fn magnitude(self) -> <P::Distance as Point>::Magnitude
    {
        let Self(a, b) = self;
        a.distance_magnitude(b)
    }
}

impl<P> PartialEq for Line<P>
where
    P: Point + PartialEq
{
    fn eq(&self, other: &Self) -> bool
    {
        match (self, other)
        {
            (Line(a1, b1), Line(a2, b2) | Line(b2, a2)) if a1 == a2 && b1 == b2 => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test
{
    use core::{fmt::Debug, ops::{Add, Neg}};

use num::{One, Zero};

use crate::{Magnitude, Point};

    #[test]
    fn test_line()
    {
        fn test_line<P, D, M>()
        where
            P: One + Zero,
            D: One + Zero + Neg<Output = D> + PartialEq + Point<Distance = D, Dimension = D, Magnitude = M> + Default,
            M: One + Add<Output = M> + Debug + PartialEq + Magnitude,
            [P; 2]: Point<Distance = [D; 2], Magnitude = M>
        {
            let line = [P::one(), P::zero()].line([P::zero(), P::one()]);

            assert_eq!(line.distance(), [-D::one(), D::one()]);
            assert_eq!(line.square_magnitude(), M::one() + M::one());
            assert_eq!(line.magnitude(), (M::one() + M::one()).approx_sqrt())
        }

        test_line::<u8, _, _>();
        test_line::<i8, _, _>();
        test_line::<u16, _, _>();
        test_line::<i16, _, _>();
        test_line::<u32, _, _>();
        test_line::<i32, _, _>();
        test_line::<u64, _, _>();
        test_line::<i64, _, _>();
        test_line::<u128, _, _>();
        test_line::<i128, _, _>();
    }
}