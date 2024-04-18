fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}

fn stringvsstr() {

  let str1 = "hello";             // &str
  //let str2 = str1.to_string();  // String
  //let str3 = str2;              // String
  let mut str4 = str1;

  println!("{}", str1);
  println!("{}", str4);

  print_type_of(&str1);
  print_type_of(&str4);

  str4 = "testrrrrrrrrrrrrrrrrrrr";

  println!("{}", str1);
  println!("{}", str4);

  print_type_of(&str1);
  print_type_of(&str4);
}

fn assignment2() {
  let val1 = 5;
  let val2 = 2;
  let ans = val1 % val2;

  println!("{}", ans);

}

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    stringvsstr();
    assignment2();


  }
