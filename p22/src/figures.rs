use std::f64::consts::PI as pi;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// A point in 2D space
/// This struct represents a point in 2D space with x and y coordinates.
/// ```
/// use p22::figures::Point;
/// let p = Point::new(1.0, 2.0);
/// assert_eq!(p.x, 1.0);
/// assert_eq!(p.y, 2.0);
/// assert_eq!(p.area(), 0.0);
/// assert_eq!(p.perimeter(), 0.0);
/// ```
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn area(&self) -> f64 {
        0.0
    }

    pub fn perimeter(&self) -> f64 {
        0.0
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();
        (x_dist * x_dist + y_dist * y_dist).sqrt()
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: f64,
}
/// A circle in 2D space
/// This struct represents a circle in 2D space with center and radius.
/// ```
/// use p22::figures::{Circle, Point};
/// use std::f64::consts::PI as pi;
/// let p = Point::new(0.0, 0.0);
/// let c = Circle::new(p, 1.0);
/// assert_eq!(c.center, p);
/// assert_eq!(c.radius, 1.0);
/// assert_eq!(c.area(), pi);
/// assert_eq!(c.perimeter(), 2.0 * pi);
/// ```
impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }

    pub fn area(&self) -> f64 {
        pi*self.radius*self.radius
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * pi * self.radius
    }
}

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}
/// A triangle in 2D space
/// This struct represents a circle in 2D space with center and radius.
/// ```
/// use p22::figures::{Triangle, Point};
/// let a = Point::new(0.0, 0.0);
/// let b = Point::new(1.0, 0.0);
/// let c = Point::new(0.0, 1.0);
/// let t = Triangle::new(a,b,c);
/// assert_eq!(t.a, a);
/// assert_eq!(t.b, b);
/// assert_eq!(t.c, c);
/// assert!((t.area() -0.5).abs() < 1e-10);
/// assert!((t.perimeter() - 3.414213562373095).abs() < 1e-10);
/// ```
impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Triangle { a, b, c }
    }

    pub fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        let a = self.a.distance(&self.b);
        let b = self.b.distance(&self.c);
        let c = self.c.distance(&self.a);
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }

    pub fn perimeter(&self) -> f64 {
        self.a.distance(&self.b) + self.b.distance(&self.c) + self.c.distance(&self.a)
    }
}
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}
/// A rectangle in 2D space
/// This struct represents a circle in 2D space with center and radius.
/// ```
/// use p22::figures::{Rectangle, Point};
/// let a = Point::new(3.0, 2.0);
/// let b = Point::new(0.0, 0.0);
/// let r = Rectangle::new(a,b);
/// assert_eq!(r.top_left, a);
/// assert_eq!(r.bottom_right, b);
/// assert_eq!(r.area(), 6.0);
/// assert_eq!(r.perimeter(), 10.0);
/// ```
impl Rectangle {

    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Rectangle { top_left, bottom_right }
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * ((self.top_left.x - self.bottom_right.x).abs() + (self.top_left.y - self.bottom_right.y).abs())
    }

    pub fn area(&self) -> f64 {
        ((self.top_left.x - self.bottom_right.x) * (self.top_left.y - self.bottom_right.y)).abs()
    }
}
enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

impl Shape {
    pub fn new(shape: Shape) -> Self {
        match shape {
            Shape::Point(p) => Shape::Point(p),
            Shape::Circle(c) => Shape::Circle(c),
            Shape::Triangle(t) => Shape::Triangle(t),
            Shape::Rectangle(r) => Shape::Rectangle(r),
        }
    }
    pub fn area(&self) -> f64 {
        match self {
            Shape::Point(_) => 0.0,
            Shape::Circle(c) => c.area(),
            Shape::Triangle(t) => t.area(),
            Shape::Rectangle(r) => r.area(),
        }
    }

    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Point(_) => 0.0,
            Shape::Circle(c) => c.perimeter(),
            Shape::Triangle(t) => t.perimeter(),
            Shape::Rectangle(r) => r.perimeter(),
        }
    }
}

