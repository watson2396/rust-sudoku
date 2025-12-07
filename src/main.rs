
use rand::prelude::*;

// lets try filling an array with any missing numbers from 1-9
/*
9 |
8 |
7 |
6 |
5 |
4 |
3 | ...
2 | a2 b2 c2 d2 e2 f2 h2 i2 j2
1 | a1 b1 c1 d1 e1 f1 h1 i1 j1
    --------------------------
    a  b  c  d  e  f  h  i  j
*/

// here in the rust by example site, https://doc.rust-lang.org/rust-by-example/flow_control/while.html
fn main() {

    println!("Hello, World!");

    // Fixed-size array (type signature is superfluous).
    let mut a: [i8; 9];

    // Get an RNG:
    let mut rng = rand::rng();

    println!("i8: '{}'", rng.random_range(1..10));
    let mut x = 0;
    while x < 10 {
        a[x] = rng.random_range(1..10);

        x += 1;
    }


}
