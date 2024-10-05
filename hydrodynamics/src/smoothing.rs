
use crate::SmoothingKernel;
use std::sync::LazyLock;

pub struct SmoothingRadius
{
    dimensionality: u8,
    integration_steps: u32,
    kernel: Box<dyn SmoothingKernel>,
}

impl SmoothingRadius
{
    pub fn new<K>(d: u8, n: u32, kernel: K) -> Self
    where
        K: SmoothingKernel + 'static,
    {
        Self {
            dimensionality: d,
            integration_steps: n,
            kernel: Box::new(kernel)
        }
    }

    /// Calculates the influence of a particle at a normalised separation
    /// distance by normalising the kernel.
    ///
    /// `r`: The normalised separation distance between particles.
    ///
    pub fn influence(&self, r: f32) -> f32
    {
        static NORMALISATION_COEFFICIENT: LazyLock<f32> = LazyLock::new(||
        {
            todo!("Calculate normalisation coefficient by integrating over the kernel");
        });
        self.kernel.kernel(r) * *NORMALISATION_COEFFICIENT
    }
}
