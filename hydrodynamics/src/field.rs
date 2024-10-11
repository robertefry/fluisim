
use crate::FieldKernel;

type FieldPos<const N: usize> = nalgebra::SVector<f32,N>;

/// Represents a uniform-mass field of particles in N-dimensional space.
///
/// ## Type Parameters
///
/// * `N` - The number of dimensions in the space.
/// * `T` - The type of particles contributing to the field.
///
/// ## Fields
///
/// * `kernel`    - The field kernel.
/// * `particles` - A vector of particles, and their field data.
///
pub struct UniformField<const N: usize, T>
{
    kernel: FieldKernel<N>,
    particles: Vec<(FieldPos<N>,T)>, // (position, particle)
}

impl<const N: usize, T> UniformField<N,T>
{
    /// Create a new uniform-mass field.
    ///
    pub fn new(kernel: FieldKernel<N>) -> Self
    {
        Self {
            kernel,
            particles: Vec::new(),
        }
    }

    /// Contribute a particle to the field.
    ///
    pub fn contribute(&mut self, position: FieldPos<N>, particle: T)
    {
        self.particles.push(( position, particle ));
    }

    /// Interpolate and evaluate the density at a position based on the
    /// positions of nearby equal-mass particles.
    ///
    pub fn density(&self, position: &FieldPos<N>) -> f64
    {
        self.particles.iter()

            // Calculate the euclidean distance from the desired position.
            //
            .map(|(position_other, _particle)|
            {
                let radius = (position - position_other).map(f64::from).norm();
                radius
            })

            // Filter only the particles who are within the kernel's support
            // radius.
            //
            .filter(|radius|
            {
                *radius <= self.kernel.support_radius()
            })

            // Calculate the influence this particle has on the density.
            //
            .map(|radius|
            {
                let influence = self.kernel.influence(radius);
                influence
            })

            // Return the sum of all contributing influences.
            .sum()
    }

    /// Interpolate a quantity field based on the quantity from all nearby
    /// particles.
    ///
    pub fn sample(&self, to_quantity: impl Fn(&T) -> f64) -> UniformQuantityField<N>
    {
        let quantities = self.particles.iter()

            // Sample the quantity from the particle.
            //
            .map(|(position, particle)|
            {
                let quantity = to_quantity(particle);
                (position, quantity)
            })

            // Sample the density at the position of the particle.
            //
            .map(|(position, quantity)|
            {
                let density = self.density(&position);
                (position, density, quantity)
            })

            // Collect the samples into a vector.
            //
            .map(|(position, density, quantity)|
            {
                (position.clone(), density, quantity)
            })
            .collect::<Vec<(FieldPos<N>,f64,f64)>>();

        UniformQuantityField {
            kernel: self.kernel.clone(),
            quantities,
        }
    }
}

/// Represents a uniform-mass field of quantities in N-dimensional space.
///
/// ## Type Parameters
///
/// * `N`: The number of dimensions in the space (const generic parameter).
///
/// ## Fields
///
/// * `kernel`     - The field kernel.
/// * `quantities` - A vector of quantities, and their field data.
///
pub struct UniformQuantityField<const N: usize>
{
    kernel: FieldKernel<N>,
    quantities: Vec<(FieldPos<N>,f64,f64)>, // (position, density, quantity)
}

impl<const N: usize> UniformQuantityField<N>
{
    /// Interpolate the quantity of the field at the desired position based on
    /// the quantities from all nearby samples.
    ///
    pub fn at(&self, position: FieldPos<N>) -> f64
    {
        self.quantities.iter()

            // Calculate the euclidean distance from the desired position.
            //
            .map(|(position_other, density, quantity)|
            {
                let radius = (position - position_other).map(f64::from).norm();
                (radius, density, quantity)
            })

            // Filter only the quantities who are within the kernel's support
            // radius.
            //
            .filter(|(radius, _density, _quantity)|
            {
                *radius <= self.kernel.support_radius()
            })

            // Calculate the influence this sample quantity has on the final
            // quantity.
            //
            .map(|(radius, density, quantity)|
            {
                let influence = self.kernel.influence(radius);
                influence * quantity / density
            })

            // Return the sum of all contributing quantity influences.
            .sum()
    }
}
