
use crate::SymmetricKernel;

/// TODO Document
///
pub struct MullerViscous
{
    support_radius: f64,
}

impl SymmetricKernel for MullerViscous
{
    fn support_radius(&self) -> f64
    {
        self.support_radius
    }
    fn kernel(&self, _r: f64) -> f64
    {
        todo!("Implement the MullerViscous kernel");
    }
}
