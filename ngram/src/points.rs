use crate::{Line, Point};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Points<P>(pub P)
where
    P: IntoIterator<Item: Point>;

impl<P> Points<P>
where
    P: IntoIterator<Item: Point>
{
    pub fn open_path(self) -> impl Iterator<Item = Line<P::Item>>
    {
        let mut iter = self.into_iter();

        let first = iter.next();

        first.map(move |mut point| {
                iter.map(move |next_point| Line(
                    core::mem::replace(&mut point, next_point.clone()),
                    next_point
                ))
            }).into_flat_iter()
    }

    pub fn enclosed_path(self) -> impl Iterator<Item = Line<P::Item>>
    {
        let mut iter = self.into_iter();

        let first = iter.next();

        first.map(move |first| {
                let mut point = first.clone();

                iter.chain(core::iter::once(first))
                    .map(move |next_point| Line(
                        core::mem::replace(&mut point, next_point.clone()),
                        next_point
                    ))
            }).into_flat_iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> <&'a mut P as IntoIterator>::IntoIter
    where
        &'a mut P: IntoIterator
    {
        (&mut self.0).into_iter()
    }

    pub fn iter<'a>(&'a self) -> <&'a P as IntoIterator>::IntoIter
    where
        &'a P: IntoIterator
    {
        (&self.0).into_iter()
    }
}

impl<'a, P> IntoIterator for &'a Points<P>
where
    P: IntoIterator<Item: Point>,
    &'a P: IntoIterator<Item: Point>
{
    type Item = <&'a P as IntoIterator>::Item;
    type IntoIter = <&'a P as IntoIterator>::IntoIter;

    fn into_iter(self) -> <&'a P as IntoIterator>::IntoIter
    {
        self.iter()
    }
}
impl<'a, P> IntoIterator for &'a mut Points<P>
where
    P: IntoIterator<Item: Point>,
    &'a mut P: IntoIterator<Item: Point>
{
    type Item = <&'a mut P as IntoIterator>::Item;
    type IntoIter = <&'a mut P as IntoIterator>::IntoIter;

    fn into_iter(self) -> <&'a mut P as IntoIterator>::IntoIter
    {
        self.iter_mut()
    }
}

impl<P> From<P> for Points<P>
where
    P: IntoIterator<Item: Point>
{
    fn from(points: P) -> Self
    {
        Self(points)
    }
}

impl<P> IntoIterator for Points<P>
where
    P: IntoIterator<Item: Point>
{
    type Item = P::Item;
    type IntoIter = P::IntoIter;

    fn into_iter(self) -> P::IntoIter
    {
        self.0.into_iter()
    }
}