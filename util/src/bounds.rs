
use std::ops::*;

pub trait Bounds<T>
where
    T: PartialOrd,
{
    fn lower_bound(&self) -> Bound<&T>;
    fn upper_bound(&self) -> Bound<&T>;

    fn lower_value(&self) -> Option<&T>
    {
        match self.lower_bound() {
            Bound::Included(bound) => Some(bound),
            Bound::Excluded(bound) => Some(bound),
            Bound::Unbounded => None,
        }
    }

    fn upper_value(&self) -> Option<&T>
    {
        match self.upper_bound() {
            Bound::Included(bound) => Some(bound),
            Bound::Excluded(bound) => Some(bound),
            Bound::Unbounded => None,
        }
    }

    fn in_range(&self, value: &T) -> bool
    {
        match self.lower_bound() {
            Bound::Included(bound) => if value < bound { return false; },
            Bound::Excluded(bound) => if value <= bound { return false; },
            Bound::Unbounded => {},
        }
        match self.upper_bound() {
            Bound::Included(bound) => if value > bound { return false; },
            Bound::Excluded(bound) => if value >= bound { return false; },
            Bound::Unbounded => {},
        }
        true
    }

    fn some_in_range(&self, value: T) -> Option<T>
    {
        if self.in_range(&value) { Some(value) } else { None }
    }

    fn clamp(&self, value: T) -> T
    where
        T: Clone,
    {
        match self.lower_bound() {
            Bound::Included(bound) => if value < *bound { return bound.clone(); },
            Bound::Excluded(bound) => if value <= *bound { return bound.clone(); },
            Bound::Unbounded => {},
        }
        match self.upper_bound() {
            Bound::Included(bound) => if value > *bound { return bound.clone(); },
            Bound::Excluded(bound) => if value >= *bound { return bound.clone(); },
            Bound::Unbounded => {},
        }
        value
    }

    fn range(&self) -> Option<T>
    where
        T: Clone + Sub<Output = T>,
    {
        let lower = self.lower_value()?.clone();
        let upper = self.upper_value()?.clone();
        Some(upper - lower)
    }

    fn normalise(&self, value: T) -> Option<T>
    where
        T: Clone + Add<Output = T> + Sub<Output = T> + Div<Output = T>,
    {
        let range = self.range()?;
        let lower = self.lower_value()?.clone();
        Some((value - lower) / range)
    }

    fn denormalise(&self, value: T) -> Option<T>
    where
        T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let range = self.range()?;
        let lower = self.lower_value()?.clone();
        Some(value * range + lower)
    }
}

impl<T, RangeBoundsT: RangeBounds<T>> Bounds<T> for RangeBoundsT
where
    T: PartialOrd,
{
    fn lower_bound(&self) -> Bound<&T>
    {
        self.start_bound()
    }

    fn upper_bound(&self) -> Bound<&T>
    {
        self.end_bound()
    }
}
