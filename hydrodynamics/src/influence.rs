
use peroxide::fuga;
use crate::FieldKernel;

pub struct FieldInfluence
{
    kernel_normalisation_coefficient: f64,
    kernel: Box<dyn FieldKernel>,
}

impl FieldInfluence
{
    /// Creates a new `SmoothingInfluence` instance with the specified parameters.
    ///
    /// * `n`       - The dimensionality of the space (e.g., 2 for 2D, 3 for 3D).
    /// * `steps`   - The number of discretization steps for the smoothing kernel.
    /// * `kernel`  - The smoothing kernel function to be used.
    ///
    pub fn new<K>(n: u8, steps: usize, kernel: K) -> Self
    where
        K: FieldKernel + 'static,
    {
        let integrand = |r: f64| kernel.kernel(r) * util::nball::volume(n,r);
        let bounds = (0.0,kernel.support_radius());
        let volume = fuga::integrate(integrand, bounds, fuga::GaussLegendre(steps));

        Self {
            kernel_normalisation_coefficient: 1.0 / volume,
            kernel: Box::new(kernel),
        }
    }

    /// Calculates the influence contribution to a property field by a particle
    /// at a distance `r`.
    ///
    /// `r` - The distance to the particle.
    ///
    pub fn influence(&self, r: f64) -> f64
    {
        self.kernel.kernel(r) * self.kernel_normalisation_coefficient
    }
}
