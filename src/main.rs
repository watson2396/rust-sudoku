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

struct Block {
    grid: [[i8; 3]; 3]
}

impl Block {
    pub fn new() -> Block {
        Block { grid: [[0i8; 3]; 3] }
    }

    pub fn print(self) {
        println!("|------------------------------|");
        for i in 0..self.grid.len() {
            let mut s: String = String::from("| ");
            for j in 0..self.grid[i].len() {
               let a = self.grid[i][j].to_string();
               s = s.to_owned() + &a + " |"; 
            }
            println!("{}", s);
        }
        println!("|------------------------------|");
    }
}

// struct Board {
//     grid: [[Block; 3]; 3]
// }


// here in the rust by example site, https://doc.rust-lang.org/rust-by-example/flow_control/while.html
fn main() {

    println!("Hello, World!");

    let mut block: Block = Block::new();

    // Get an RNG:
    let mut rng = rand::rng();

    // fill in Block randomly
    for i in 0..block.grid.len() {
        for j in 0..block.grid[i].len() {
            block.grid[i][j] = rng.random_range(1..10);
            //println!("x-index: {}, y-index: {}, value: {}", i, j, block.grid[i][j]);
        }
    }

    block.print();
}
