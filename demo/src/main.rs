fn main() {
    println!("Hello, world!");
    let a_number = 10;
    let a_boolean = true;
    println!("the number is {}.", a_number);
    println!("the boolean is {}.", a_boolean);
    let mut mut_nubmer = 1;
    println!("the mut number is {}.", mut_nubmer);
    mut_nubmer = 2;
    println!("and now number is {}.", mut_nubmer);
    let number = 5;
    let number = number + 5;
    let number = number * 2;
    println!("The number is: {}", number);
    let type_number: u32 = "42".parse().expect("Not a number!");
    println!("the number is: {}", type_number);
    let x = 2.0;      // f64, default type
    let y: f32 = 3.0; // f32, via type annotation
    println!("the number is: {}", x);
    println!("the number is: {}", y);
     // Addition
     println!("1 + 2 = {}", 1u32 + 2);
     // Subtraction
     println!("1 - 2 = {}", 1i32 - 2);
     // ^ Try changing `1i32` to `1u32` to see why the type is important
     // Integer Division
     println!("9 / 2 = {}", 9u32 / 2);
     // Float Division
     println!("9 / 2 = {}", 9.0 / 2.0);
     // Multiplication
     println!("3 * 6 = {}", 3 * 6);
     let char = 'c';
     println!("this char is {}", char);
}