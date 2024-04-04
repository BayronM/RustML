use ndarray::Array1;

fn main() {
    let x = Array1::from(vec![1.0, 2.0, 3.0]);
    let y = Array1::from(vec![4.0, 5.0, 6.0]);
    let (m, b) = linear_regression(&x, &y);

    println!("m: {}, b: {}", m, b);
    // prediction
    let x_pred = 8.0;
    // y = mx + b
    let y_pred = m * x_pred + b;
    println!("y_pred: {}", y_pred);
}

fn linear_regression(x: &Array1<f64>, y: &Array1<f64>) -> (f64, f64) {
    let n = x.len() as f64;
    let sum_x = x.sum();
    let sum_y = y.sum();
    let sum_xx = x.mapv(|xi| xi * xi).sum();
    let sum_xy = (x * y).sum();
    // slope
    let m = (n * sum_xy - (sum_x * sum_y)) / (n * sum_xx - (sum_x * sum_x));
    // intercept
    let b = (sum_y - m * sum_x) / n;

    (m, b)
}
