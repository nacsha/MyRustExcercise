fn main () {
    let mut x_left: f64 = 0.0;
    let mut x_right: f64 = 1.0;

    let mut x_mid: f64 = (x_left + x_right) * 0.5;

    println!("Hello, world!");
}

fn example_function (x: f64) -> f64 {
    return x.powf(3.0) + 1.0;
}
