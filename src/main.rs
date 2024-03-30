use core::f32;
use std::env;
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
    println!("Welcome to Quadratic equation solver! Enter values for ax^2 + bx + c");

    let args: Vec<_> = env::args().collect();

    let quad1 = QuadraticCoEff{
        a: args[1].trim().parse::<f32>().unwrap(),
        b: args[2].trim().parse::<f32>().unwrap(),
        c: args[3].trim().parse::<f32>().unwrap(),
   };

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
