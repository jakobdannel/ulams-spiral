# ulams-spiral

A tool that generates an image of Ulams Spiral.

## What is Ulam's Spiral?

Ulam's Spiral is a graphical depiction of prime numbers. Every number that is prime is colored. The spiral starts in the middle of the image.
```
(17)-  16 -  15 -  14 - (13)
 |                       |
 18   (5) -  4  - (3)    12
 |     |           |     |
(19)   6     1  - (2)   (11)
 |     |                 |
 20   (7) -  8  -  9  -  10
 |  
 21 -  22 - (23)-  24 -  25
```

## How to use this tool

* Clone this repository with `git clone git@github.com/jakobdannel/ulams-spiral.git`
* Install rust (https://www.rust-lang.org/learn/get-started)
* Build project with `cargo build --release`
* Execute file `/targets/release/ulams-spiral`
* Enter desired size
* The output is generated as a .png file under `/output`

## Example image (1001x1001)

![Example image](/output/output.png)
