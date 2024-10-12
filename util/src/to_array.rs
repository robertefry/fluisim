
pub trait ToArray<T>
{
    unsafe fn to_array<const N: usize>(self) -> [T; N];
}

impl<I, T> ToArray<T> for I where I: Iterator<Item = T>
{
    /// Collect an iterator into an array of size `N`.
    ///
    /// ## Safety
    ///
    /// Calling this method on an iterator of size not equal to `N` is
    /// *[undefined behaviour](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)*.
    ///
    unsafe fn to_array<const N: usize>(mut self) -> [T; N]
    {
        std::array::from_fn(|_|
        {
            self.next().unwrap_unchecked()
        })
    }
}
