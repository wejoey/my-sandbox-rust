#![allow(unused_variables, dead_code)]

// Compile-time constants
const MY_CONST: usize = 9;

// Static variables
static MY_STATIC: &str = "My static banner";

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn takes_u32(x: u32) {
  println!("u32: {x}");
}

fn takes_i8(y: i8) {
  println!("i8: {y}");
}

fn say_hello(name: String) {
  println!("Hello {name}")
}

fn main() {
    // conversion
    let x: i8 = 15;
    let y: i16 = 1000;
    //println!("{x} * {y} = {}", multiply(x.into(), y));

    // array
    let array = [10, 20, 30];
    println!("array: {array:?}");

    for x in array {
        //println!("{x}-");
        print!("{x}-");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();

    // auto type inference
    let auto1 = 10;
    let auto2 = 20;

    takes_u32(auto1);
    takes_i8(auto2);
    //takes_u32(auto2);

    // global const et static
    println!("global const et static:");
    println!("MY_CONST: {MY_CONST}");
    println!("MY_STATIC: {MY_STATIC}");

    // le ptr est passé (move)
    let name = String::from("Alice");
    say_hello(name);
    //say_hello(name);  // --> le ptr de name à été moved à la fn

    // While move semantics are the default, certain types are copied by default:
    // ex: i32
    // mon propre type doit use the copy semantics si je veux que ça fasse une copie
    // au lieu du move par défaut.
    // #[derive(Copy, Clone, Debug)]

    // Borrowinf

}
