
use crate::FieldKernel;

/// TODO Document
///
pub struct Poly6
{
    support_radius: f64,
}

impl FieldKernel for Poly6
{
    fn support_radius(&self) -> f64
    {
        self.support_radius
    }
    fn kernel(&self, r: f64) -> f64
    {
        (1.0 - r.powi(2)).powi(3)
    }
}
