
use crate::FieldKernel;

/// A uniform-mass field comprising a collection of objects in N-dimensional
/// space.
///
/// This struct implements a field where objects are distributed uniformly and
/// their influence is determined by a field kernel.
///
/// ## Type Parameters
///
/// * `N` - The number of dimensions in the space.
/// * `T` - The type of objects contributing to the field.
///
/// ## Fields
///
/// * `kernel`          - The field kernel.
/// * `contributors`    - A vector of objects, and their field data.
///
pub struct UniformField<const N: usize, T>
{
    kernel: FieldKernel<N>,
    contributors: Vec<([f32;N],T)>, // (position, object)
}

impl<const N: usize, T> UniformField<N,T>
{
    /// Create a new uniform-mass field.
    ///
    pub fn new(kernel: FieldKernel<N>) -> Self
    {
        Self {
            kernel,
            contributors: Vec::new(),
        }
    }

    /// Contribute an object to the field.
    ///
    pub fn contribute(&mut self, position: [f32;N], object: T)
    {
        self.contributors.push(( position, object ));
    }

    /// Interpolate and evaluate the density at a position based on the
    /// positions of nearby equal-mass particles mapped by the field kernel.
    ///
    pub fn density(&self, position: &[f32;N]) -> f64
    {
        self.contributors.iter()

            // Calculate the distance (radius) from the desired position.
            //
            .map(|(position_other, _object)|
            {
                let radius = util::euclidean::distance(&position, position_other);
                radius
            })

            // Filter only the objects who are within the kernel's support radius.
            //
            .filter(|radius|
            {
                *radius <= self.kernel.support_radius()
            })

            // Calculate the influence this object has on the density.
            //
            .map(|radius|
            {
                let influence = self.kernel.influence(radius);
                influence
            })

            // Return the sum of all contributing influences.
            .sum()
    }

    /// Interpolate a quantity of the field at a position based on the quantity
    /// of nearby particles mapped by the field kernel.
    ///
    pub fn quantity(&self, to_quantity: impl Fn(&T) -> f64) -> UniformQuantityField<N>
    {
        let contributors = self.contributors.iter()

            // Clone the position of each contributing object.
            //
            .map(|(position, object)|
            {
                (position.clone(), object)
            })

            // Map the objects to their desired quantities.
            //
            .map(|(position, object)|
            {
                let quantity = to_quantity(object);
                (position, quantity)
            })

            // Calculate the density field for each contributor position.
            //
            .map(|(position, quantity)|
            {
                let density = self.density(&position);
                (position, density, quantity)
            })

            // Collect the quantity field contributors into a vector.
            //
            .collect::<Vec<([f32;N],f64,f64)>>();

        UniformQuantityField {
            kernel: self.kernel.clone(),
            contributors,
        }
    }
}

/// Represents a uniform quantity field in N-dimensional space.
///
/// This struct holds information about a field where each point in space
/// has an associated quantity. The field is defined by a set of contributors.
///
/// ## Type Parameters
///
/// * `N`: The number of dimensions in the space (const generic parameter).
///
/// ## Fields
///
/// * `kernel`          - The field kernel.
/// * `contributors`    - The vector of contributors.
///
pub struct UniformQuantityField<const N: usize>
{
    kernel: FieldKernel<N>,
    contributors: Vec<([f32;N],f64,f64)>, // (position, density, quantity)
}

impl<const N: usize> UniformQuantityField<N>
{
    /// Evaluate the quantity of the field at the desired position based on the
    /// interpolated field quantities mapped by the field kernel.
    ///
    pub fn at(&self, position: [f32;N]) -> f64
    {
        self.contributors.iter()

            // Calculate the distance (radius) from the desired position.
            //
            .map(|(position_other, density, quantity)|
            {
                let radius = util::euclidean::distance(&position, position_other);
                (radius, density, quantity)
            })

            // Filter only the contributors who are within the kernel's support radius.
            //
            .filter(|(radius, _density, _quantity)|
            {
                *radius <= self.kernel.support_radius()
            })

            // Calculate the influence this contributor has on the quantity.
            //
            .map(|(radius, density, quantity)|
            {
                let influence = self.kernel.influence(radius);
                influence * quantity / density
            })

            // Return the sum of all contributing influences.
            .sum()
    }
}
