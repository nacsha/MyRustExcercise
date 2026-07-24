const EPS: f64 = 1.0e-10;
const PI: f64 = 3.14159265359;

fn main () {
    let x_left_ini: f64 = - PI;
    let x_right_ini: f64 = PI;

    let ans: Option<f64> = recursive_bisection(x_left_ini, x_right_ini);

    if ans == None {
        println!("No solution bewteen {} and {}.", x_left_ini, x_right_ini);
        return;
    }

    println!("Answer: x = ");
    println!("{}", ans.unwrap());

    let err_val: f64 = function_for_solve(ans.unwrap());
    println!("error:");
    println!("{}", err_val);
}

fn function_for_solve (x: f64) -> f64 {
    return x * x - 1.0;
    //return 1.0 + x * x;
}

fn recursive_bisection (x_left: f64, x_right: f64) -> Option<f64> {
    let x_mid: f64 = (x_left + x_right) * 0.5;
    if function_for_solve(x_mid).abs() < EPS {
        Some(x_mid) 
    } else if function_for_solve(x_mid) * function_for_solve(x_left) < 0.0 {
        recursive_bisection(x_left, x_mid)
    } else if function_for_solve(x_mid) * function_for_solve(x_right) < 0.0 {
        recursive_bisection(x_mid, x_right)
    } else {
        None
    }
}