
use crate::Kernel;

pub struct Poly6;

impl Kernel for Poly6
{
    fn kernel(&self, h: f64, r: f64) -> f64
    {
        (h.powi(2) - r.powi(2)).powi(3)
    }
}
