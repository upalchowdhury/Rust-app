use std::fmt;
use std::mem;


fn testprimmut () {
    let mut b = 1;
    b += 3;
    println!("the value of b is {}",b);
}

// Underscore can be used for readability in numeric   


// Tuple is a collection of values of different types. Tuples are constructed using parenthesis ()
// It can be used as function argument and return values

// fn reverse(pair: (i32, bool)) -> (bool, i32) {
//     let (integer, boolean) = pair;
//     return (boolean,integer)
// }

#[derive(Debug)]
struct Matrix (f32,f32,f32,f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"({},{},{},{})", self.0,self.1,self.2,self.3)
    }
}

fn main() {
    testprimmut();
    let pair = (234,true);
    let mat = Matrix(1.3,1.2,1.4,1.5);
    println!("Hello, world! {}", mat);
    let arr : [i32 ; 4] = [0, 4, 5 ,6];
    println!("the slice is {:?}", &arr[1..4])
}
