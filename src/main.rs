fn main () {
    let x_left_ini: f64 = 0.0;
    let x_right_ini: f64 = 1.0;

    let ans: Option<f64> = recur(x_left_ini, x_right_ini);

    println!("{}", ans.unwrap());
}

fn func (x: f64) -> f64 {
    return 3.0 * x - 1.0;
}

fn recur (mut x_left: f64, mut x_right: f64) -> Option<f64> {
    let x_mid: f64 = (x_left + x_right) * 0.5;
    if func(x_mid).abs() < 1.0e-5 {
        Some(x_mid) 
    } else if func(x_mid) * func(x_left) < 0.0 {
        x_right = x_mid;
        recur(x_left, x_right)
    } else if func(x_mid) * func(x_right) < 0.0 {
        x_left = x_mid;
        recur(x_left, x_right)
    } else {
        None
    }
}