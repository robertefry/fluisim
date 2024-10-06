
pub trait FieldKernel
{
    /// Defines the radius of support for the smoothing kernel for which a
    /// property field can be meaningfully calculated.
    ///
    fn support_radius(&self) -> f64;

    /// Defines the smoothing kernel used to calculate a property field for all
    /// distances `r` away from a particle.
    ///
    /// * `r`: The distance between particle and field property.
    ///
    /// To ensure compact support, the kernel should vanish as `r` tends to
    /// the radius of support.
    ///
    fn kernel(&self, r: f64) -> f64;
}
