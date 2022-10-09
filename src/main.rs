use std::io::*;

struct Point {
    x: usize,
    y: usize
}
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

    let start: Point = Point { x: width/2, y: height/2 };

    println!("The center is at {},{}.", start.x, start.y);

    let pixel_count: usize = width * height;
    
    for i in 0..pixel_count {
        println!("{} is a prime: {}",i+1,is_prime(i+1));
    }
}


fn is_prime(n: usize) -> bool {
    if n == 1 {
        return false;
    }

    let root = f32::sqrt(n as f32).round();
    for i in 2..=root as usize {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}