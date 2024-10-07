
pub fn distance<const N: usize>(pos1: &[f32;N], pos2: &[f32;N]) -> f64
{
    itertools::izip!(pos1, pos2)
        .map(|(a,b)| (a-b).powi(2) as f64)
        .sum::<f64>()
        .sqrt()
}
