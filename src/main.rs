use std::io::*;

fn main() {
    print!("Please enter a width:");
    let mut input: String = String::new();

    let mut width: usize = 100;
    stdout().flush().expect("flush");
    match stdin().read_line(&mut input) {
        Ok(_n) => width = input.trim().parse().expect("No number"),
        Err(e) => println!("error: {}", e),
    }
    input.clear();
    print!("Please enter a height:");
    
    let mut height: usize = 100;
    stdout().flush().expect("flush");
    match stdin().read_line(&mut input) {
        Ok(_n) => height = input.trim().parse().expect("No number"),
        Err(e) => println!("error: {}", e),
    }
    input.clear();
    
    if (width%2) == 0 {
        width += 1;
    }
    if (height%2) == 0 {
        height += 1;
    }

    println!("The generated image will be {} pixels wide and {} pixels tall.", width, height);   
}