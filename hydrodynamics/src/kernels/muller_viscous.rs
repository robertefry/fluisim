
use crate::Kernel;

pub struct MullerViscous;

impl Kernel for MullerViscous
{
    fn kernel(&self, _h: f64, _r: f64) -> f64
    {
        todo!("Implement the MullerViscous kernel");
    }
}
