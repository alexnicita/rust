use std::io;

fn main() {

    // println!("fib(?) = ");

    loop {
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
    
        let index: u32 = index.trim().parse().expect("Please type a number!");
    
        let mut a: u32 = 0;
        let mut b: u32 = 1;
    
        if index <= 2 {
            let value: u32 = index - 1;
            println!("fib({index}) = {value}")
        } else {
            for _ in 2..index {
                let c: u32 = a + b;
                a = b;
                b = c;
            }
            println!("fib({index}) = {b}")
        }
    }
}
