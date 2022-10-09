use std::io::*;
use image::{RgbImage, Rgb};

struct Point {
    x: usize,
    y: usize
}

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Upwards,
    Downwards,
}
impl Direction {
    fn next(&self) -> Self{
        match self {
            Direction::Left => Direction::Downwards,
            Direction::Right => Direction::Upwards,
            Direction::Upwards => Direction::Left,
            Direction::Downwards => Direction::Right
        }
    }
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

    generate_image(width, height, start);
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

fn generate_image(width: usize, height: usize, start: Point) {
    let pixel_count = width * height;
    let mut img = RgbImage::new(width as u32, height as u32);

    let mut current_position: Point = start;
    let mut direction: Direction = Direction::Right;
    let mut steps: usize = 0;
    let mut sidelength: usize = 1;
    let mut k = 0;
    for i in 0..pixel_count {
        if is_prime(i+1) {
            img.put_pixel(current_position.x as u32, current_position.y as u32, Rgb([255,255,255]));
        }

        if direction == Direction::Left {
            current_position.x -= 1;
            steps += 1;
        } else if direction == Direction::Right {
            current_position.x += 1;
            steps += 1;
        } else if direction == Direction::Upwards {
            current_position.y -= 1;
            steps += 1;
        } else if direction == Direction::Downwards {
            current_position.y += 1;
            steps += 1;
        }
        
        if steps == sidelength {
            steps = 0;
            direction = direction.next();
            k += 1;
            if k % 2 == 0 {
                k = 0;
                sidelength += 1;
            }
        }
    }
    img.save("./output/output.png").expect("write img");
}