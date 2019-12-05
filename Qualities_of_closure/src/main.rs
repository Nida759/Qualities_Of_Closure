// Making a closure with no arguments
// fn main() {
//     let x = || println!("Hello Nida");
//     x();
// }

//capturing values from environment
// fn main() {
//     let mut y = 1;
//     let mut o = || {y = y + 2;println!("y = {}",y)};
//     o();
//     o();
//     o();
// }

// another example of capturing values from environment
// fn main() {
//     let mut y = 4;
//     let mut z = 4;
//     let mut x = || {y = y + 4; z = z + 4};
//     x();
//     println!("y = {},z = {}",y,z);
// }

// another example of capturing values from environment
// fn main() {
//     let mut v = 2;
//     let mut inc = || {v = v + 4;};
//     inc();
//     println!("v = {}",v);
// }


// passing a closure as an argument to a function
// fn Hey<T:Fn()>(y:T){
//     y();
// }
// fn main() {
//     let mut y = || println!("Hello this is nida");
//     Hey(y);
// }

// A closure expecting u32 argument and returns a u32 value is passed as an argument to a function
// fn Hey<T:Fn(u32)->u32>(y:T)->u32{
//     y(4)
// }
// fn main() {
//     let mut y = |x| x + 2;
//     // println!("{}",x(4));
//     println!("{}",Hey(y));
// }