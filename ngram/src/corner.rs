use crate::{Line, Point, Points, error::LinesDoNotMeet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Corner<P>(Points<[P; 3]>)
where
    P: Point;

impl<P> Corner<P>
where
    P: Point
{
    pub fn new(left: P, center: P, right: P) -> Self
    {
        Self::from(Points::from([left, center, right]))
    }

    pub fn between(line1: Line<P>, line2: Line<P>) -> Option<Corner<P>>
    where
        P: PartialEq
    {
        [line1, line2]
            .try_into()
            .ok()
    }

    pub fn left(self) -> P
    {
        let Points([l, _, _]) = self.0;
        l
    }
    pub fn center(self) -> P
    {
        let Points([_, c, _]) = self.0;
        c
    }
    pub fn right(self) -> P
    {
        let Points([_, _, r]) = self.0;
        r
    }

    pub fn points(self) -> Points<[P; 3]>
    {
        self.0
    }

    pub fn lines(&self) -> [Line<P>; 2]
    {
        self.points()
            .open_path()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Lines of a corner are exactly two.")
    }
}

impl<P> From<Points<[P; 3]>> for Corner<P>
where
    P: Point
{
    fn from(points: Points<[P; 3]>) -> Self
    {
        Self(points)
    }
}

impl<P> TryFrom<[Line<P>; 2]> for Corner<P>
where
    P: Point
{
    type Error = LinesDoNotMeet<P>;

    fn try_from(lines: [Line<P>; 2]) -> Result<Self, Self::Error>
    {
        match lines
        {
            [Line(p, a) | Line(a, p), Line(p_eq, b) | Line(b, p_eq)] if p == p_eq && a != b => Ok(Corner([a, p, b].into())),
            [l1, l2] => Err(LinesDoNotMeet(l1, l2))
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{Corner, Line};

    #[test]
    fn test_corner()
    {
        let l1 = Line(1, 2);
        let l2 = Line(1, 3);
        let c = l1.corner(l2).expect("Lines did not meet!");

        assert_eq!(c, Corner::new(2, 1, 3));

        println!("{c:?}")
    }
}