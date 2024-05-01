
use std::ops::*;

pub trait Interval<T>
{
    fn lower_bound(&self) -> T;
    fn upper_bound(&self) -> T;

    fn range(&self) -> T;

    fn normalise(&self, value: T) -> T;
    fn denormalise(&self, norm: T) -> T;
}

#[derive(Copy, Clone)]
pub struct ClosedInterval<T>
{
    lower_bound: T,
    upper_bound: T,
}

impl <T> ClosedInterval<T>
{
    pub const fn new(lower_bound: T, upper_bound: T) -> Self
    {
        ClosedInterval{ lower_bound, upper_bound }
    }
}

impl <T> Interval<T> for ClosedInterval<T>
where
    T: Clone,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
{
    fn upper_bound(&self) -> T
    {
        self.upper_bound.clone()
    }

    fn lower_bound(&self) -> T
    {
        self.lower_bound.clone()
    }

    fn range(&self) -> T
    {
        self.upper_bound.clone() - self.lower_bound.clone()
    }

    fn normalise(&self, value: T) -> T
    {
        (value - self.lower_bound.clone()) / self.range()
    }

    fn denormalise(&self, norm: T) -> T
    {
        self.lower_bound.clone() + norm * self.range()
    }
}

impl <T> From<ClosedInterval<T>> for RangeInclusive<T>
where
    T: Clone,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
{
    fn from(interval: ClosedInterval<T>) -> Self
    {
        interval.lower_bound() ..= interval.upper_bound()
    }
}
