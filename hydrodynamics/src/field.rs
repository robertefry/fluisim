use util::to_array::*;
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
                quantity * influence / density
            })

            // Return the sum of all contributing quantity influences.
            .sum()
    }

    /// Interpolate the gradient of the field based on quantity from all nearby
    /// samples.
    pub fn gradient(&self, delta: f64) -> UniformGradientField<N>
    {
        let to_gradient = |position: &FieldPos<N>, quantity: f64|
        {
            let to_partial = |i|
            {
                let position_offset = FieldPos::from_fn(|k,_|
                {
                    if i == k { delta as f32 } else { 0.0 }
                });
                let quantity_partial = self.at(position + position_offset);

                (quantity_partial - quantity) / delta
            };

            std::array::from_fn(to_partial)
        };

        let gradients = self.quantities.iter()

            // Map the quantity at each position to the gradient of that
            // quantity.
            //
            .map(|(position, density, quantity)|
            {
                let gradient = to_gradient(position, *quantity);
                (position, density, gradient)
            })

            // Collect the gradients into a vector.
            //
            .map(|(position, density, gradient)|
            {
                (position.clone(), density.clone(), gradient)
            })
            .collect();

        UniformGradientField {
            kernel: self.kernel.clone(),
            gradients,
        }
    }
}

pub struct UniformGradientField<const N: usize>
{
    kernel: FieldKernel<N>,
    gradients: Vec<(FieldPos<N>,f64,[f64;N])>, // (position, density, gradient)
}

impl<const N: usize> UniformGradientField<N>
{
    pub fn at(&self, position: FieldPos<N>) -> [f64;N]
    {
        self.gradients.iter()

            // Calculate the euclidean distance from the desired position.
            //
            .map(|(position_other, density, gradient)|
            {
                let radius = (position - position_other).map(f64::from).norm();
                (radius, density, gradient)
            })

            // Filter only the gradients who are within the kernel's support
            // radius.
            //
            .filter(|(radius, _density, _gradient)|
            {
                *radius <= self.kernel.support_radius()
            })

            // Calculate the influence this sample gradients has on the final
            // gradient.
            //
            .map(|(radius, density, gradient)|
            {
                let influence = self.kernel.influence(radius);
                gradient.map(|q| q * influence / density)
            })

            // Return the sum of all contributing gradients influences.
            .fold([0.0;N], |sum, grad| unsafe
            {
                itertools::izip!(sum, grad)
                    .map(|(e_sum,e_grad)| e_sum + e_grad)
                    .to_array()
            })
    }
}
