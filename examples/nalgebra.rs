use nalgebra as na;
use rand::prelude::*;
use rand::rngs::StdRng;

#[derive(Clone, Copy, Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn polynomial_regression(points: &[Point3D]) -> na::DVector<f64> {
    let n = points.len();
    let matrix_size = 3; // for our third-degree polynomial

    let mut x_matrix = na::DMatrix::<f64>::zeros(n, matrix_size);
    let mut y_vector = na::DVector::<f64>::zeros(n);

    for (i, point) in points.iter().enumerate() {
        x_matrix[(i, 0)] = point.x;
        x_matrix[(i, 1)] = point.z;
        x_matrix[(i, 2)] = point.x * point.z; // interaction term
        y_vector[i] = point.y;
    }

    let pseudo_inverse = x_matrix.pseudo_inverse(1.0e-15).unwrap();
    pseudo_inverse * y_vector
}

fn predict_y(point: &Point3D, coeffs: &na::DVector<f64>) -> f64 {
    coeffs[0] * point.x + coeffs[1] * point.z + coeffs[2] * point.x * point.z
}

fn generate_random_points(num_points: usize, seed: u64, min: f64, max: f64) -> Vec<Point3D> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut points = Vec::new();

    let scale = |v: f64| (v * (max - min)) + min;

    for _ in 0..num_points {
        points.push(Point3D {
            x: scale(rng.gen::<f64>()),
            y: scale(rng.gen::<f64>()),
            z: scale(rng.gen::<f64>()),
        });
    }

    points
}

fn mean_squared_error(points: &[Point3D], coeffs: &na::DVector<f64>) -> f64 {
    let n = points.len() as f64;
    points.iter().map(|&point| {
        let error = point.y - predict_y(&point, coeffs);
        error * error
    }).sum::<f64>() / n
}

fn main() {
    let data = generate_random_points(100, 12345, -5.0, 5.0);
    let coeffs = polynomial_regression(&data);

    let mse_all = mean_squared_error(&data, &coeffs);
    println!("MSE for all data ({} points): {}", data.len(), mse_all);

    let mut rng = StdRng::seed_from_u64(67890);
    let mut random_indices: Vec<usize> = (0..data.len()).collect();

    for i in 0..5 {
        let swap_with = rng.gen_range(i..random_indices.len());
        random_indices.swap(i, swap_with);
    }

    let selected_points: Vec<_> = random_indices.iter().take(5).map(|&i| data[i].clone()).collect();
    let mse_5 = mean_squared_error(&selected_points, &coeffs);
    println!("MSE for selected 5 data points: {}", mse_5);

    for &i in random_indices.iter().take(5) {
        let point = &data[i];
        let predicted_y = predict_y(point, &coeffs);
        let error = point.y - predicted_y;

        println!("{:?}, y = {:.3}, predicted y = {:.3}, error = {:.3}", point, point.y, predicted_y, error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mse_all_points() {
        let data = generate_random_points(100, 12345, -5.0, 5.0);
        let coeffs = polynomial_regression(&data);
        let mse = mean_squared_error(&data, &coeffs);
        assert!(mse >= 0.0);  // MSE should always be non-negative
    }

    #[test]
    fn test_mse_first_5_points() {
        let data = generate_random_points(100, 12345, -5.0, 5.0);
        let coeffs = polynomial_regression(&data);
        let mse = mean_squared_error(&data[0..5], &coeffs);
        assert!(mse >= 0.0);  // MSE should always be non-negative
    }
}
