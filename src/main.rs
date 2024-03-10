use std::io;
use std::f32;
struct QuadraticCoEff {
    a: f32,
    b: f32,
    c: f32,
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


    let quadradic_discriminant = (quad1.b * quad1.b) - 4.0 * quad1.a * quad1.c;


    if quadradic_discriminant < 0.0 {
        println!("Solution is imaginary. ");
    } else if quadradic_discriminant == 0.0 {
        let sol1 = (quad1.b + quadradic_discriminant.sqrt()) / (2.0 * quad1.a);
        println!("There is one real solution: {}", sol1);
    } else {
        let sol1 = (quad1.b + quadradic_discriminant.sqrt()) / (2.0 * quad1.a);
        let sol2 = (quad1.b - quadradic_discriminant.sqrt()) / (2.0 * quad1.a);
        println!("There are two solutions. Positive root: {} and negative root: {}", sol1, sol2);
    }

    println!("Fin");


}
