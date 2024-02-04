use std::io;

struct QuadraticCoEff {
    a: i32,
    b: i32,
    c: i32,
}
fn main() {
    println!("Hello, world!");

    let mut quad1 = QuadraticCoEff{
        a: 0,
        b: 0,
        c: 0,
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


    let quadradic_discriminant = (quad1.b * quad1.b) - 4 * quad1.a * quad1.c;


    if quadradic_discriminant < 0 {
        println!("Solution is imaginary. ");
    }

}
