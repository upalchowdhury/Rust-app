use std::fmt;
use std::mem;


// fn testprimmut () {
//     let mut b = 1;
//     b += 3;
//     println!("the value of b is {}",b);
// }

// // Underscore can be used for readability in numeric   


// // Tuple is a collection of values of different types. Tuples are constructed using parenthesis ()
// // It can be used as function argument and return values

// // fn reverse(pair: (i32, bool)) -> (bool, i32) {
// //     let (integer, boolean) = pair;
// //     return (boolean,integer)
// // }

// #[derive(Debug)]
// struct Matrix (f32,f32,f32,f32);

// impl fmt::Display for Matrix {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,"({},{},{},{})", self.0,self.1,self.2,self.3)
//     }
// }

// // Enum
// // a common use case for enums is to create a linked-list:
// // use crate::List::*;

// // enum List {
// //     Cons(u32, Box<List>),
// //     Nil,
// // }


// // GENERICS
// struct  Point<T,U> {
//     x: T,
//     y: U,
// }

// impl <T,U> Point<T,U> {
//     fn mixup<V,W> (self, other: Point<V,W> ) -> Point<T,W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }


// Collections
let a: [i32; 3] = [1,2,3];
let mut v2: Vec<i32> = Vec::new();
v2.push(3)



fn main() {
    // testprimmut();
    // let pair = (234,true);
    // let mat = Matrix(1.3,1.2,1.4,1.5);
    // println!("Hello, world! {}", mat);
    // let arr : [i32 ; 4] = [0, 4, 5 ,6];
    // println!("the slice is {:?}", &arr[1..4]);

    // let p1: Point<i32, f64> = Point { x: 5, y: 10.4};
    // let p2: Point<&str, char> = Point {x: "hello", y: 'C'};
    // let p3: Point<i32, char> = p1.mixup(p2);
    // println! ("p3.x = {}, p3.y = {}",p3.x,p3.y);

}
