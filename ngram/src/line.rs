use crate::{Corner, Point, Points};

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
        a.distance(b)
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

pub fn equals<P>(l1: &[P; 2], l2: &[P; 2]) -> bool
where
    P: PartialEq
{
    match (l1, l2)
    {
        ([a1, b1], [a2, b2] | [b2, a2]) if a1 == a2 && b1 == b2 => true,
        _ => false
    }
}

pub fn length(l: [[f64; 2]; 2]) -> f64
{
    let [[ax, ay], [bx, by]] = l;
    let [dx, dy] = [bx - ax, by - ay];
    (dx*dx + dy*dy).sqrt()
}