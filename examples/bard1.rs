// Bard couldn't get this right
use linregress::RegressionDataBuilder;//, RegressionModel};

fn main() {
    // Define the set of points
    let points = [(0.0, 1.0), (1.0, 0.0), (2.0, 2.0)];

    // Create a builder for the linear model
    let mut builder = RegressionDataBuilder::new(points);

    // Add the points to the builder
    for point in points {
        builder.add_point(point);
    }

    // Fit the linear model
    let model = builder.fit().unwrap();

    // Evaluate the model at an arbitrary x-value
    let x = 3.0;
    let y = model.predict(&[x]).unwrap();

    // Print the y-value
    println!("y = {}", y);
}
