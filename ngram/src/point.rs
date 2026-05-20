use core::{fmt::Debug, iter::Sum, ops::{Add, Sub}};

use crate::Line;

pub trait Magnitude: Point + Add<Output = Self> + Default + Sum
{
    fn approx_sqrt(self) -> Self;
}

pub trait Point: Sized + Clone + Copy + Debug + PartialEq + 'static
{
    type Magnitude: Magnitude<Magnitude = Self::Magnitude>;
    type Distance: Point<Distance = Self::Distance, Magnitude = Self::Magnitude, Dimension: Default>;
    type Dimension: Point<Distance = <Self::Distance as Point>::Dimension, Magnitude = Self::Magnitude, Dimension = Self::Dimension>;
    type Dimensions: IntoIterator<Item = Self::Dimension>;

    fn line(self, other: Self) -> Line<Self>
    {
        Line(self, other)
    }

    fn distance(self, other: Self) -> Self::Distance;
    fn distance_square_magnitude(self, other: Self) -> Self::Magnitude
    {
        self.dimensions()
            .into_iter()
            .zip(other.dimensions())
            .map(|(a, b)| a.distance_square_magnitude(b))
            .sum()
    }
    fn distance_magnitude(self, other: Self) -> Self::Magnitude
    {
        let mut iter = self.dimensions()
            .into_iter()
            .zip(other.dimensions())
            .map(|(a, b)| a.line(b));
        match iter.next()
        {
            None => Default::default(),
            Some(first) => match iter.next()
                .map(|line| line.square_magnitude())
            {
                None => first.magnitude(),
                Some(second) => [first.square_magnitude(), second].into_iter()
                    .chain(iter.map(|line| line.square_magnitude()))
                    .sum::<Self::Magnitude>()
                    .approx_sqrt()
            }
        }
    }
    fn dimensions(self) -> Self::Dimensions;
}

macro_rules! impl_magnitude {
    ($({$m:ty : $($t:ty),+}),+
    {
        fn sqrt($self_sqrt:ident) -> _
        $fn_sqrt:block

        fn distance_magnitude($self_distance_magnitude:ident, $rhs_distance_magnitude:ident) -> _
        $fn_distance_magnitude:block
    }
    ) => {
        $(
            impl Magnitude for $m
            {
                fn approx_sqrt(self) -> Self
                {
                    fn sqrt($self_sqrt: $m) -> $m
                    $fn_sqrt

                    sqrt(self)
                }
            }
            $(
                impl Point for $t
                {
                    type Magnitude = $m;
                    type Distance = Self;
                    type Dimension = Self;
                    type Dimensions = [Self; 1];

                    fn distance(self, other: Self) -> Self::Distance
                    {
                        self - other
                    }
                    fn distance_square_magnitude(self, other: Self) -> $m
                    {
                        let d = self.distance_magnitude(other);
                        d*d
                    }
                    fn distance_magnitude(self, other: Self) -> $m
                    {
                        fn distance_magnitude($self_distance_magnitude: $t, $rhs_distance_magnitude: $t) -> $m
                        $fn_distance_magnitude

                        distance_magnitude(self, other)
                    }
                    fn dimensions(self) -> Self::Dimensions
                    {
                        [self]
                    }
                }
            )+
        )+
    };
}

impl_magnitude!(
    {f16 : f16},
    {f32 : f32},
    {f64 : f64},
    {f128 : f128}
    {
        fn sqrt(x) -> _
        {
            x.sqrt()
        }

        fn distance_magnitude(lhs, rhs) -> _
        {
            (lhs - rhs).abs()
        }
    }
);
impl_magnitude!(
    {u8 : u8, i8},
    {u16 : u16, i16},
    {u32 : u32, i32},
    {usize : usize, isize},
    {u64 : u64, i64},
    {u128 : u128, i128}
    {
        fn sqrt(this) -> _
        {
            this.isqrt()
        }

        fn distance_magnitude(lhs, rhs) -> _
        {
            lhs.abs_diff(rhs)
        }
    }
);

impl<D, const N: usize> Point for [D; N]
where
    D: Point<Distance = D, Dimension = D> + Sub<Output = D::Distance> + Default
{
    type Magnitude = D::Magnitude;
    type Distance = [D::Distance; N];
    type Dimension = D;
    type Dimensions = [D; N];

    fn distance(self, other: Self) -> Self::Distance
    {
        let mut other = other.into_iter();
        self.map(|d| d.distance(other.next().expect("Arrays must be same length")))
    }
    fn dimensions(self) -> Self::Dimensions
    {
        self
    }
}