
use peroxide::fuga;

/// Calculate the volume of an n-ball of dimensionality `n` and radius `r`.
///
/// * `n`   - The number of dimensions the n-ball exists in.
/// * `r`   - The radius of the n-ball.
///
/// The volume of an n-ball in euclidean space is the Lebesgue measure of the
/// n-ball. We calculate it using the closed-form formula.
/// See the linked Wikipedia article for more information.
///
/// https://en.wikipedia.org/wiki/volume_of_an_n-ball
///
pub fn volume(n: u8, r: f64) -> f64
{
    const PI: f64 = std::f64::consts::PI;
    PI.powf(n as f64 / 2.0) * r.powi(n as i32) / fuga::gamma(1.0 + n as f64 / 2.0)
}
