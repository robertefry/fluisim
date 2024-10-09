
use peroxide::fuga;
use std::rc::Rc;

/// Represents a symmetric smoothing kernel used in smoothed particle
/// hydrodynamic simulations.
///
pub trait Kernel
{
    /// Defines the smoothing kernel used to calculate a property field for all
    /// distances `r` away from a particle.
    ///
    /// * 'h' - The radius of support for the smoothing kernel.
    /// * `r` - The distance between particle and field property.
    ///
    /// To ensure compact support, the kernel should vanish as `r` tends to
    /// the 'h'.
    ///
    fn kernel(&self, h: f64, r: f64) -> f64;
}

/// Represents a normalised field kernel used in smoothed particle hydrodynamic
/// simulations.
///
/// This struct encapsulates a symmetric kernel and provides methods to
/// calculate the influence of particles on property fields.
///
/// ## Type Parameters
///
/// * `N` - The number of dimensions in the space.
///
#[derive(Clone)]
pub struct FieldKernel<const N: usize>
{
    kernel_support_radius: f64,
    kernel_normalisation_coefficient: f64,
    kernel: Rc<dyn Kernel>,
}

impl<const N: usize> FieldKernel<N>
{
    /// Creates a new `FieldKernel` instance with the specified parameters.
    ///
    /// * `kernel`  - The smoothing kernel function to be used.
    /// * 'support' - The radius of support for the smoothing kernel.
    /// * `steps`   - The number of discretization steps for the smoothing kernel.
    ///
    pub fn new<K>(kernel: K, support: f64, steps: usize) -> Self
    where
        K: Kernel + 'static,
    {
        let mut field_kernel = Self
        {
            kernel_support_radius: 0.0,
            kernel_normalisation_coefficient: 0.0,
            kernel: Rc::new(kernel),
        };

        field_kernel.integrate(support, steps);
        field_kernel
    }

    /// Calculates the influence contribution to a property field by a particle
    /// at a distance `r`.
    ///
    /// `r` - The distance to the particle.
    ///
    pub fn influence(&self, r: f64) -> f64
    {
        if r > self.kernel_support_radius { return 0.0 };

        let support = self.kernel_support_radius;
        let normal = self.kernel_normalisation_coefficient;
        self.kernel.kernel(support, r) * normal
    }

    /// Recalculate the field properties for a given radius of support, using
    /// the given number of discretization steps.
    ///
    /// * `support` - The radius of support for the smoothing kernel.
    /// * `steps`   - The number of discretization steps for any numerical methods.
    ///
    pub fn integrate(&mut self, support: f64, steps: usize)
    {
        self.kernel_support_radius = support;

        // Recalculate the normalisation coefficient by numerically integrating
        // the kernel over the support volume.

        let support = self.kernel_support_radius;

        let integrand = |r: f64| self.kernel.kernel(support,r) * util::nball::volume(N as u8,r);
        let bounds = (0.0, support);
        let integral = fuga::GaussLegendre(steps);
        let volume = fuga::integrate(integrand, bounds, integral);

        self.kernel_normalisation_coefficient = 1.0 / volume;
    }

    /// Return the radius of support for the smoothing kernel.
    ///
    pub fn support_radius(&self) -> f64
    {
        self.kernel_support_radius
    }
}
