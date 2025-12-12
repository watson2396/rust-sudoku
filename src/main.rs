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
    // columns x rows
    grid: [[i8; 3]; 3]
}

impl Block {
    pub fn new() -> Block {
        Block { grid: [[0i8; 3]; 3] }
    }

    pub fn print(&self) {
        // on my terminal looks to be ~3 '-' per number

        // just do multiple loops bro
        let mut spacers: String = String::from("");
        for _ in 0..self.grid.len() {
            spacers = spacers + "---";
        }
        let header_footer_str = "| ".to_owned() + &spacers + " |";

        println!("{}", header_footer_str);
        
        for i in 0..self.grid.len() {
            let mut s: String = String::from("| ");
            for j in 0..self.grid[i].len() {
               let a = self.grid[i][j].to_string();
               s = s.to_owned() + &a + " | "; 
            }
            println!("{}", s);
        }

        println!("{}", header_footer_str);
    }

    pub fn fill(&mut self) {
        // Get an RNG:
        let mut rng = rand::rng();

        // fill in Block randomly
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                self.grid[i][j] = rng.random_range(1..10);
            }
        }
    }
}

// struct Board {
//     grid: [[Block; 3]; 3]
// }


// here in the rust by example site, https://doc.rust-lang.org/rust-by-example/flow_control/while.html
fn main() {

    println!("Hello, World!");
    let mut block: Block = Block::new();

    block.print();

    println!();

    block.fill();

    block.print();
}
