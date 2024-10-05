
pub trait SmoothingKernel
{
    /// Defines the kernel of influence for a particle at a normalised
    /// separation distance.
    ///
    /// * `r`: The normalised separation distance between particles.
    ///
    /// By normalising the separation distance over the radius of support before
    /// calculation, we remove the need for any dimensionality calculations
    /// when normalising the kernel in order to calculate its influence.
    ///
    fn kernel(&self, r: f32) -> f32;
}

pub mod kernels
{
    use super::*;

    /// TODO Document
    ///
    pub struct Poly6;

    impl SmoothingKernel for Poly6
    {
        fn kernel(&self, r: f32) -> f32
        {
            (1.0 - r.powi(2)).powi(3)
        }
    }

    /// TODO Document
    ///
    pub struct DebrunSpiky;

    impl SmoothingKernel for DebrunSpiky
    {
        fn kernel(&self, r: f32) -> f32
        {
            todo!("Implement the DebrunSpiky kernel");
        }
    }

    /// TODO Document
    ///
    pub struct MullerViscous;

    impl SmoothingKernel for MullerViscous
    {
        fn kernel(&self, r: f32) -> f32
        {
            todo!("Implement the MullerViscous kernel");
        }
    }
}

