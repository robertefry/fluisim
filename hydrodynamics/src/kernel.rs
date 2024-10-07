
use peroxide::fuga;
use std::rc::Rc;

/// Represents a symmetric smoothing kernel used in smoothed particle
/// hydrodynamic simulations.
///
pub trait SymmetricKernel
{
    /// Defines the radius of support for the smoothing kernel for which a
    /// property field can be meaningfully calculated.
    ///
    fn support_radius(&self) -> f64;

    /// Defines the smoothing kernel used to calculate a property field for all
    /// distances `r` away from a particle.
    ///
    /// * `r` - The distance between particle and field property.
    ///
    /// To ensure compact support, the kernel should vanish as `r` tends to
    /// the radius of support.
    ///
    fn kernel(&self, r: f64) -> f64;
}

/// Represents a normalised field kernel used in smoothed particle hydrodynamic
/// simulations.
///
/// This struct encapsulates a symmetric kernel and provides methods to
/// calculate the influence of particles on property fields.
///
#[derive(Clone)]
pub struct FieldKernel
{
    kernel_normalisation_coefficient: f64,
    kernel: Rc<dyn SymmetricKernel>,
}

impl FieldKernel
{
    /// Creates a new `FieldKernel` instance with the specified parameters.
    ///
    /// * `n`       - The dimensionality of the space (e.g., 2 for 2D, 3 for 3D).
    /// * `steps`   - The number of discretization steps for the smoothing kernel.
    /// * `kernel`  - The smoothing kernel function to be used.
    ///
    pub fn new<K>(n: u8, steps: usize, kernel: K) -> Self
    where
        K: SymmetricKernel + 'static,
    {
        let integrand = |r: f64| kernel.kernel(r) * util::nball::volume(n,r);
        let bounds = (0.0,kernel.support_radius());
        let volume = fuga::integrate(integrand, bounds, fuga::GaussLegendre(steps));

        Self {
            kernel_normalisation_coefficient: 1.0 / volume,
            kernel: Rc::new(kernel),
        }
    }

    /// Defines the radius of support for the smoothing kernel for which a
    /// property field can be meaningfully calculated.
    ///
    pub fn support_radius(&self) -> f64
    {
        self.kernel.support_radius()
    }

    /// Calculates the influence contribution to a property field by a particle
    /// at a distance `r`.
    ///
    /// `r` - The distance to the particle.
    ///
    pub fn influence(&self, r: f64) -> f64
    {
        if r > self.kernel.support_radius() { return 0.0 };
        self.kernel.kernel(r) * self.kernel_normalisation_coefficient
    }
}
