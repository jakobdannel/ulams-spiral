use image::{Rgb, RgbImage};
use std::io::*;

struct Point {
    x: usize,
    y: usize,
}

struct RgbColor {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Upwards,
    Downwards,
}
impl Direction {
    fn turn_counterclockwise(&self) -> Self {
        match self {
            Direction::Left => Direction::Downwards,
            Direction::Right => Direction::Upwards,
            Direction::Upwards => Direction::Left,
            Direction::Downwards => Direction::Right,
        }
    }
}

fn main() {
    print!("Please enter a width and height:");
    let mut input: String = String::new();
    let mut inverted: bool = false;
    let mut rainbow: bool = true;

    let mut width: usize = 100;
    stdout().flush().expect("flush");
    match stdin().read_line(&mut input) {
        Ok(_n) => width = input.trim().parse().expect("No number"),
        Err(e) => println!("error: {e}"),
    }
    input.clear();
    print!("Background color black or white? (Default black) (b/w):");
    stdout().flush().expect("flush");
    match stdin().read_line(&mut input) {
        Ok(_n) => {
            if input.trim() == "b" {
                inverted = false
            } else if input.trim() == "w" {
                inverted = true
            } else if input == "\n" {
                inverted = false;
            } else {
                println!("Invalid input, black selected.");
            }
        }
        Err(e) => println!("error: {e}"),
    }
    input.clear();

    print!("Rainbow? (y/N):");
    stdout().flush().expect("flush");
    match stdin().read_line(&mut input) {
        Ok(_n) => {
            if input.trim() == "n" {
                rainbow = false
            } else if input.trim() == "y" {
                rainbow = true;
            } else if input == "\n" {
                rainbow = false;
            } else {
                println!("Invalid input, no selected.");
            }
        }
        Err(e) => println!("error: {e}"),
    }

    let mut height = width;

    if (width % 2) == 0 {
        width += 1;
    }
    if (height % 2) == 0 {
        height += 1;
    }

    println!("The generated image will be {width} pixels wide and {height} pixels tall.");

    generate_image(width, height, inverted, rainbow);
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
    true
}

fn generate_image(width: usize, height: usize, inverted: bool, rainbow: bool) {
    let pixel_count = width * height;
    let start: Point = Point {
        x: width / 2,
        y: height / 2,
    };
    let mut img = RgbImage::new(width as u32, height as u32);

    let mut current_position: Point = start;
    let mut direction: Direction = Direction::Right;
    let mut steps: usize = 0;
    let mut sidelength: usize = 1;
    let mut counter = 0;
    let mut percentage: f32;
    let mut rgb: RgbColor = RgbColor {
        red: 255,
        green: 255,
        blue: 255,
    };
    for i in 0..pixel_count {
        percentage = i as f32 / pixel_count as f32;
        if rainbow {
            rgb = hsl_to_rgb(percentage, 1.0, 0.5);
        }

        if is_prime(i + 1) ^ inverted {
            img.put_pixel(
                current_position.x as u32,
                current_position.y as u32,
                Rgb([rgb.red, rgb.green, rgb.blue]),
            );
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
            direction = direction.turn_counterclockwise();
            counter += 1;
            if counter % 2 == 0 {
                counter = 0;
                sidelength += 1;
            }
        }
    }
    img.save("./output/output.png").expect("write img");
}

fn hsl_to_rgb(hue: f32, saturation: f32, luminance: f32) -> RgbColor {
    let mut rgb: RgbColor = RgbColor {
        red: 0,
        green: 0,
        blue: 0,
    };

    let c = (1.0 - (2.0 * luminance - 1.0).abs()) * saturation;
    let h = hue * 6.0;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let m = luminance - (c / 2.0);
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    if (0.0..=1.0).contains(&h) {
        r = c;
        g = x;
        b = 0.0;
    } else if (1.0..=2.0).contains(&h) {
        r = x;
        g = c;
        b = 0.0;
    } else if (2.0..=3.0).contains(&h) {
        r = 0.0;
        g = c;
        b = x;
    } else if (3.0..=4.0).contains(&h) {
        r = 0.0;
        g = x;
        b = c;
    } else if (4.0..=5.0).contains(&h) {
        r = x;
        g = 0.0;
        b = c;
    } else if (5.0..=6.0).contains(&h) {
        r = c;
        g = 0.0;
        b = x;
    }
    rgb.red = ((r + m) * 255.0) as u8;
    rgb.green = ((g + m) * 255.0) as u8;
    rgb.blue = ((b + m) * 255.0) as u8;
    rgb
}
