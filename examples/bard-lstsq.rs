// bard couldn't get this right.
use lstsq;

fn main() {
    // Define the set of points
    let points = [(0.0, 1.0), (1.0, 0.0), (2.0, 2.0)];

    // Fit a linear model to the points
    let (m, b) = lstsq::solve(&points);

    // Evaluate the model at an arbitrary x-value
    let x = 3.0;
    let y = m * x + b;

    // Print the y-value
    println!("y = {}", y);
}