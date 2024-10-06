
use crate::FieldKernel;

/// TODO Document
///
pub struct DebrunSpiky
{
    support_radius: f64,
}

impl FieldKernel for DebrunSpiky
{
    fn support_radius(&self) -> f64
    {
        self.support_radius
    }
    fn kernel(&self, _r: f64) -> f64
    {
        todo!("Implement the DebrunSpiky kernel");
    }
}
