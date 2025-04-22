// Import modules from your own crate
use p22::figures::{point_new, circle_new, triangle_new, rectangle_new};
use p22::figures::{point_area, circle_area, triangle_area, rectangle_area};
use p22::figures::{circle_perimeter, triangle_perimeter, rectangle_perimeter};

fn main() {
    println!("Geometric shapes example:");

    let p = point_new(1.0, 2.0);
    println!("Point at ({}, {})", p.x, p.y);
    println!("Point area: {}", point_area(&p));

    let c = circle_new(p, 3.0);
    println!("Circle with radius {} at ({}, {})", c.radius, c.center.x, c.center.y);
    println!("Circle area: {}", circle_area(&c));
    println!("Circle perimeter: {}", circle_perimeter(&c));

    let p1 = point_new(0.0, 0.0);
    let p2 = point_new(4.0, 0.0);
    let p3 = point_new(0.0, 3.0);
    let t = triangle_new(p1, p2, p3);
    println!("Triangle area: {}", triangle_area(&t));
    println!("Triangle perimeter: {}", triangle_perimeter(&t));

    let r = rectangle_new(p3, p1);
    println!("Rectangle area: {}", rectangle_area(&r));
    println!("Rectangle perimeter: {}", rectangle_perimeter(&r));
}