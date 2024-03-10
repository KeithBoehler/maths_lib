use std::io;
use std::f32;
use std::process;

struct QuadraticCoEff {
    a: f32,
    b: f32,
    c: f32,
}

struct AnsHolder {
    d: f32,
    r1: f32,
    r2: f32,
}

fn main() {
    println!("Hello, world!");

    let mut quad1 = QuadraticCoEff{
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };
    let mut line_input = String::new();
    println!("Welcome to Quadratic equation solver! Enter values for ax^2 + bx + c");


    println!("Enter value for a: ");
    io::stdin().read_line(&mut line_input).expect("Failed to read input. ");
    quad1.a = line_input.trim().parse().expect("Input not an integer!");
    line_input.clear();

    println!("Enter value for b: ");
    io::stdin().read_line(&mut line_input).expect("Failed to read input. ");
    quad1.b = line_input.trim().parse().expect("Input not an integer!");
    line_input.clear();

    println!("Enter value for c: ");
    io::stdin().read_line(&mut line_input).expect("Failed to read input. ");
    quad1.c = line_input.trim().parse().expect("Input not an integer!");
    line_input.clear();

    let equ_solustion = quad_calc(quad1);

    if equ_solustion.d == 0.0 {
        println!("There is one real solution: {}", equ_solustion.r1);
    } else {
        println!("There are two solutions. Positive root: {} and negative root: {}", equ_solustion.r1, equ_solustion.r2);
    }

    println!("Fin");


}

fn quad_calc(coeffs: QuadraticCoEff) -> AnsHolder{
    // 

    let quadradic_discriminant = (coeffs.b * coeffs.b) - 4.0 * coeffs.a * coeffs.c;

    if quadradic_discriminant < 0.0 {
        println!("Solution is imaginary. ");
        process::exit(0);
    }

    let pos_root = (coeffs.b + quadradic_discriminant.sqrt()) / (2.0 * coeffs.a);
    let neg_root = (coeffs.b - quadradic_discriminant.sqrt()) / (2.0 * coeffs.a);

    let sol = AnsHolder{
        d: quadradic_discriminant,
        r1: pos_root,
        r2: neg_root,
    };

    return sol;
}
