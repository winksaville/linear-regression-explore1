// Try 7 Worked after a tweak. We had initially a runtime error
// which the bot identified. The problem was had only 3 points
// and the regression needs at least one more point than the
// number of variables.  So we added a point and it compiles
// and runs and returns the execpted result, z = 1.0 for
// x = 0.0 and y = 0.5.
//
// Here is the link to the conversation:
//    https://chat.openai.com/share/c2200401-7e7d-4469-a7e1-c59e8c6512ec
extern crate linregress;

use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

fn main() {
    // Given data points as (x, y, z) tuples
    let points = vec![
        (1.0, 0.0, 1.0),
        (-1.0, 0.0, 1.0),
        (0.0, 1.0, 1.0),
        (0.5, 0.5, 1.0),
    ];

    let x: Vec<_> = points.iter().map(|p| p.0).collect();
    let y: Vec<_> = points.iter().map(|p| p.1).collect();
    let z: Vec<_> = points.iter().map(|p| p.2).collect();

    // Constructing data using a vector of tuples
    let data_tuples = vec![
        ("x", x),
        ("y", y),
        ("z", z),
    ];

    // Setup regression data
    let data = RegressionDataBuilder::new()
        .build_from(data_tuples)
        .unwrap();

    // Define the regression formula
    let formula = "z ~ x + y";
    let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula(formula)
        .fit()
        .unwrap();

    // Predict z for the test point (0, 0.5)
    let test_data = vec![
        ("x", vec![0.0]),
        ("y", vec![0.5]),
    ];
    let predictions: Vec<f64> = model.predict(test_data).unwrap();

    if let Some(z_predicted) = predictions.get(0) {
        println!("Predicted z value for (0, 0.5): {}", z_predicted);
    }
}

// Try 1
//extern crate linregress;

//use linregress::{FormulaRegressionBuilder, RegressionDataBuilder, RegressionModel};

//fn main() {
    //// Given data points as (x, y, z) tuples
    //let points = vec![
        //(1.0, 0.0, 1.0),
        //(-1.0, 0.0, 1.0),
        //(0.0, 1.0, 1.0),
    //];

    //let x: Vec<_> = points.iter().map(|p| p.0).collect();
    //let y: Vec<_> = points.iter().map(|p| p.1).collect();
    //let z: Vec<_> = points.iter().map(|p| p.2).collect();

    //// Setup and perform the regression
    //let data = RegressionDataBuilder::new()
        //.set_response(z)
        //.add_input(x)
        //.add_input(y)
        //.build()
        //.unwrap();

    //let formula = FormulaRegressionBuilder::new()
        //.response("z")
        //.input("x")
        //.input("y")
        //.build()
        //.unwrap();

    //let model = RegressionModel::fit(&formula, &data).unwrap();

    //// Predict z for the test point (0, 0.5)
    //let x_test = 0.0;
    //let y_test = 0.5;
    //let z_predicted = model.predict(&[x_test, y_test]).unwrap();

    //println!("Predicted z value for ({}, {}): {}", x_test, y_test, z_predicted);
//}

//// Try 2
//extern crate linregress;
//
//use linregress::{RegressionBuilder, RegressionData};
//
//fn main() {
//    // Given data points as (x, y, z) tuples
//    let points = vec![
//        (1.0, 0.0, 1.0),
//        (-1.0, 0.0, 1.0),
//        (0.0, 1.0, 1.0),
//    ];
//
//    let x: Vec<_> = points.iter().map(|p| p.0).collect();
//    let y: Vec<_> = points.iter().map(|p| p.1).collect();
//    let z: Vec<_> = points.iter().map(|p| p.2).collect();
//
//    // Construct regression data
//    let data = RegressionData::new()
//        .set_x(x)
//        .set_y(z) // z is our "y" in this context, as we're doing regression to predict it
//        .set_exog(y) // Use y-coordinate as an exogenous variable
//        .build().unwrap();
//
//    // Perform the regression
//    let regression = RegressionBuilder::new()
//        .data(&data)
//        .build().unwrap();
//
//    // Predict z for the test point (0, 0.5)
//    let x_test = 0.0;
//    let y_test = 0.5;
//    let z_predicted = regression.predict(&[x_test, y_test]);
//
//    println!("Predicted z value for ({}, {}): {}", x_test, y_test, z_predicted);
//}

//// Try 3
//extern crate linregress;
//
//use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
//
//fn main() {
//    // Given data points as (x, y, z) tuples
//    let points = vec![
//        (1.0, 0.0, 1.0),
//        (-1.0, 0.0, 1.0),
//        (0.0, 1.0, 1.0),
//    ];
//
//    let x: Vec<_> = points.iter().map(|p| p.0).collect();
//    let y: Vec<_> = points.iter().map(|p| p.1).collect();
//    let z: Vec<_> = points.iter().map(|p| p.2).collect();
//
//    // Setup regression data
//    let data = RegressionDataBuilder::new()
//        .add_input("x", x)
//        .add_input("y", y)
//        .add_response("z", z)
//        .build().unwrap();
//
//    // Define the regression formula
//    let formula = FormulaRegressionBuilder::new()
//        .input("x")
//        .input("y")
//        .response("z")
//        .build().unwrap();
//
//    // Perform the regression
//    let regression = formula.fit(&data).unwrap();
//
//    // Predict z for the test point (0, 0.5)
//    let x_test = 0.0;
//    let y_test = 0.5;
//    let z_predicted = regression.predict(&[x_test, y_test]).unwrap();
//
//    println!("Predicted z value for ({}, {}): {}", x_test, y_test, z_predicted);
//}

//// Try 4
//extern crate linregress;
//
//use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
//use std::collections::HashMap;
//
//fn main() {
//    // Given data points as (x, y, z) tuples
//    let points = vec![
//        (1.0, 0.0, 1.0),
//        (-1.0, 0.0, 1.0),
//        (0.0, 1.0, 1.0),
//    ];
//
//    let x: Vec<_> = points.iter().map(|p| p.0).collect();
//    let y: Vec<_> = points.iter().map(|p| p.1).collect();
//    let z: Vec<_> = points.iter().map(|p| p.2).collect();
//
//    // Constructing the HashMap for input data
//    let mut data_map: HashMap<&str, Vec<f64>> = HashMap::new();
//    data_map.insert("x", x);
//    data_map.insert("y", y);
//
//    // Setup regression data
//    let data = RegressionDataBuilder::new()
//        .build_from(&data_map)
//        .unwrap();
//
//    // Define the regression formula
//    let formula = FormulaRegressionBuilder::new()
//        .input("x")
//        .input("y")
//        .response("z")
//        .build().unwrap();
//
//    // Perform the regression
//    let regression = formula.fit(&data).unwrap();
//
//    // Predict z for the test point (0, 0.5)
//    let x_test = 0.0;
//    let y_test = 0.5;
//    let z_predicted = regression.predict(&[x_test, y_test]).unwrap();
//
//    println!("Predicted z value for ({}, {}): {}", x_test, y_test, z_predicted);
//}
//

//// Try 5
//extern crate linregress;
//
//use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
//
//fn main() {
//    // Given data points as (x, y, z) tuples
//    let points = vec![
//        (1.0, 0.0, 1.0),
//        (-1.0, 0.0, 1.0),
//        (0.0, 1.0, 1.0),
//    ];
//
//    let x: Vec<_> = points.iter().map(|p| p.0).collect();
//    let y: Vec<_> = points.iter().map(|p| p.1).collect();
//    let z: Vec<_> = points.iter().map(|p| p.2).collect();
//
//    // Constructing data using a vector of tuples
//    let data_tuples = vec![
//        ("x", x),
//        ("y", y),
//        ("z", z),
//    ];
//
//    // Setup regression data
//    let data = RegressionDataBuilder::new()
//        .build_from(data_tuples)
//        .unwrap();
//
//    // Define the regression formula
//    let formula = FormulaRegressionBuilder::new()
//        .input("x")
//        .input("y")
//        .response("z")
//        .build().unwrap();
//
//    // Perform the regression
//    let regression = formula.fit(&data).unwrap();
//
//    // Predict z for the test point (0, 0.5)
//    let x_test = 0.0;
//    let y_test = 0.5;
//    let z_predicted = regression.predict(&[x_test, y_test]).unwrap();
//
//    println!("Predicted z value for ({}, {}): {}", x_test, y_test, z_predicted);
//}
//

//// Try 6
//extern crate linregress;
//
//use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
//
//fn main() {
//    // Given data points as (x, y, z) tuples
//    let points = vec![
//        (1.0, 0.0, 1.0),
//        (-1.0, 0.0, 1.0),
//        (0.0, 1.0, 1.0),
//    ];
//
//    let x: Vec<_> = points.iter().map(|p| p.0).collect();
//    let y: Vec<_> = points.iter().map(|p| p.1).collect();
//    let z: Vec<_> = points.iter().map(|p| p.2).collect();
//
//    // Constructing data using a vector of tuples
//    let data_tuples = vec![
//        ("x", x),
//        ("y", y),
//        ("z", z),
//    ];
//
//    // Setup regression data
//    let data = RegressionDataBuilder::new()
//        .build_from(data_tuples)
//        .unwrap();
//
//    // Define the regression formula
//    let formula = "z ~ x + y";
//    let model = FormulaRegressionBuilder::new()
//        .data(&data)
//        .formula(formula)
//        .fit()
//        .unwrap();
//
//    // Predict z for the test point (0, 0.5)
//    let x_test = 0.0;
//    let y_test = 0.5;
//    let z_predicted = model.predict(&[x_test, y_test]).unwrap();
//
//    println!("Predicted z value for ({}, {}): {}", x_test, y_test, z_predicted);
//}
//