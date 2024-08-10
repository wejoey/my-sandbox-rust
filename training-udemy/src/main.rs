// fn stringvsstr() {

//   let str1 = "hello";             // &str
//   //let str2 = str1.to_string();  // String
//   //let str3 = str2;              // String
//   let mut str4 = str1;

//   println!("{}", str1);
//   println!("{}", str4);
//   str4 = "testrrrrrrrrrrrrrrrrrrr";

//   println!("{}", str1);
//   println!("{}", str4);
// }

// fn assignment2_1() {
//   let val1 = 5;
//   let val2 = 2;
//   let ans = val1 % val2;
//   println!("{}", ans);
// }

// fn assignment2_2() {
//   let mut vec1 = vec![2,4,6,8,10];
//   //let vec2 = Vec::new()["2,4,6,8,10"];

//   println!("{:?}", vec1);

//   vec1.remove(4);
//   vec1.push(12);

//   println!("{:?}", vec1);
// }

// fn concat_string(word : String) -> String{
//   let fixed_string = String::from("Hello");
//   let ret:String = fixed_string + &word;
//   ret
// }

// fn assignment2_3() {
//   let s = String::from("value");
//   //println!("{}", concat_string(" ---world".to_string()));
//   println!("{}", concat_string(s));
// }

// fn control_flow(i: i8){
//   print!("{} : ", i);
//   if i == 1 {
//     println!("equals 1");
//   }
//   else if i > 50 {
//     println!("gt 50");
//   }
//   else if i < 25 {
//     println!("lt 25");
//   }
//   else{
//     println!("between 25 and 50");
//   }
// }

// fn assignment2_4() {
//   control_flow(0);
//   control_flow(1);
//   control_flow(22);
//   control_flow(33);
//   control_flow(55);
//   control_flow(99);
// }

fn assignment3_1() {
  let mut my_vec = vec![1,3,5,7];
  let ret = assignment3_1_1(&my_vec);
  println!("{}", ret);
  my_vec.push(15);
  println!("{:?}", my_vec);

  let result = add_two(&4);
  println!("{}", result);
  println!("{}", add_two(&7));
}

fn assignment3_1_1(val: &Vec<i8>) -> bool {
  if val[0] == 1 {
    return true
  }
  else {
    return false
  }
}

fn add_two(val: &i8) -> i8 {
  val + 2
}

fn main() {
    //stringvsstr();
    //assignment2_1();
    //assignment2_2();
    //assignment2_3();
    //assignment2_4();
    assignment3_1();

  }
