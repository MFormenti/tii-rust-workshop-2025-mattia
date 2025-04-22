use std::f64::consts::PI as pi;

/// A point in 2D space
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Creates a new Point with the given coordinates
///
/// # Examples
///
/// ```
/// use p22::figures::{Point, point_new, point_area, point_perimeter};
/// let p = point_new(1.0, 2.0);
/// assert_eq!(p.x, 1.0);
/// assert_eq!(p.y, 2.0);
/// assert_eq!(point_area(&p), 0.0);
/// assert_eq!(point_perimeter(&p), 0.0);
/// ```
pub fn point_new(x: f64, y: f64) -> Point {
    Point { x, y }
}

/// Returns the area of a point (always 0.0)
pub fn point_area(_point: &Point) -> f64 {
    0.0
}

/// Returns the perimeter of a point (always 0.0)
pub fn point_perimeter(_point: &Point) -> f64 {
    0.0
}

/// Calculates the Euclidean distance between two points
pub fn point_distance(point1: &Point, point2: &Point) -> f64 {
    let x_dist = (point1.x - point2.x).abs();
    let y_dist = (point1.y - point2.y).abs();
    (x_dist * x_dist + y_dist * y_dist).sqrt()
}

/// A circle in 2D space defined by its center point and radius
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

/// Creates a new Circle with the given center and radius
///
/// # Examples
///
/// ```
/// use p22::figures::{Circle, Point, point_new, circle_new, circle_area, circle_perimeter};
/// use std::f64::consts::PI as pi;
/// let p = point_new(0.0, 0.0);
/// let c = circle_new(p, 1.0);
/// assert_eq!(c.center, p);
/// assert_eq!(c.radius, 1.0);
/// assert_eq!(circle_area(&c), pi);
/// assert_eq!(circle_perimeter(&c), 2.0 * pi);
/// ```
pub fn circle_new(center: Point, radius: f64) -> Circle {
    Circle { center, radius }
}

/// Calculates the area of the circle (π * r²)
pub fn circle_area(circle: &Circle) -> f64 {
    pi * circle.radius * circle.radius
}

/// Calculates the perimeter (circumference) of the circle (2 * π * r)
pub fn circle_perimeter(circle: &Circle) -> f64 {
    2.0 * pi * circle.radius
}

/// A triangle in 2D space defined by its three vertices
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

/// Creates a new Triangle with the given vertices
///
/// # Examples
///
/// ```
/// use p22::figures::{Triangle, Point, point_new, triangle_new, triangle_area, triangle_perimeter};
/// let a = point_new(0.0, 0.0);
/// let b = point_new(1.0, 0.0);
/// let c = point_new(0.0, 1.0);
/// let t = triangle_new(a,b,c);
/// assert_eq!(t.a, a);
/// assert_eq!(t.b, b);
/// assert_eq!(t.c, c);
/// assert!((triangle_area(&t) -0.5).abs() < 1e-10);
/// assert!((triangle_perimeter(&t) - 3.414213562373095).abs() < 1e-10);
/// ```
pub fn triangle_new(a: Point, b: Point, c: Point) -> Triangle {
    Triangle { a, b, c }
}

/// Calculates the area of the triangle using Heron's formula
pub fn triangle_area(triangle: &Triangle) -> f64 {
    let s = triangle_perimeter(triangle) / 2.0;
    let a = point_distance(&triangle.a, &triangle.b);
    let b = point_distance(&triangle.b, &triangle.c);
    let c = point_distance(&triangle.c, &triangle.a);
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

/// Calculates the perimeter of the triangle (sum of the three sides)
pub fn triangle_perimeter(triangle: &Triangle) -> f64 {
    point_distance(&triangle.a, &triangle.b)
        + point_distance(&triangle.b, &triangle.c)
        + point_distance(&triangle.c, &triangle.a)
}

/// A rectangle in 2D space defined by its top-left and bottom-right corners
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

/// Creates a new Rectangle with the given corners
///
/// # Examples
///
/// ```
/// use p22::figures::{Rectangle, Point, point_new, rectangle_new, rectangle_area, rectangle_perimeter};
/// let a = point_new(3.0, 2.0);
/// let b = point_new(0.0, 0.0);
/// let r = rectangle_new(a,b);
/// assert_eq!(r.top_left, a);
/// assert_eq!(r.bottom_right, b);
/// assert_eq!(rectangle_area(&r), 6.0);
/// assert_eq!(rectangle_perimeter(&r), 10.0);
/// ```
pub fn rectangle_new(top_left: Point, bottom_right: Point) -> Rectangle {
    Rectangle {
        top_left,
        bottom_right,
    }
}

/// Calculates the perimeter of the rectangle (2 * (width + height))
pub fn rectangle_perimeter(rectangle: &Rectangle) -> f64 {
    2.0 * ((rectangle.top_left.x - rectangle.bottom_right.x).abs()
        + (rectangle.top_left.y - rectangle.bottom_right.y).abs())
}

/// Calculates the area of the rectangle (width * height)
pub fn rectangle_area(rectangle: &Rectangle) -> f64 {
    ((rectangle.top_left.x - rectangle.bottom_right.x)
        * (rectangle.top_left.y - rectangle.bottom_right.y))
        .abs()
}

/// An enum representing different geometric shapes
pub enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

/// Creates a new Shape from the provided shape type
pub fn shape_new(shape: Shape) -> Shape {
    shape
}

/// Calculates the area of the shape
pub fn shape_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Point(p) => point_area(p),
        Shape::Circle(c) => circle_area(c),
        Shape::Triangle(t) => triangle_area(t),
        Shape::Rectangle(r) => rectangle_area(r),
    }
}

/// Calculates the perimeter of the shape
pub fn shape_perimeter(shape: &Shape) -> f64 {
    match shape {
        Shape::Point(p) => point_perimeter(p),
        Shape::Circle(c) => circle_perimeter(c),
        Shape::Triangle(t) => triangle_perimeter(t),
        Shape::Rectangle(r) => rectangle_perimeter(r),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = point_new(1.0, 2.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    fn test_point_area_and_perimeter() {
        let p = point_new(1.0, 2.0);
        assert_eq!(point_area(&p), 0.0);
        assert_eq!(point_perimeter(&p), 0.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = point_new(0.0, 0.0);
        let p2 = point_new(3.0, 4.0);
        assert_eq!(point_distance(&p1, &p2), 5.0);
    }

    #[test]
    fn test_circle_creation() {
        let center = point_new(1.0, 2.0);
        let c = circle_new(center, 3.0);
        assert_eq!(c.center, center);
        assert_eq!(c.radius, 3.0);
    }

    #[test]
    fn test_circle_area() {
        let center = point_new(0.0, 0.0);
        let c = circle_new(center, 1.0);
        assert_eq!(circle_area(&c), pi);
    }

    #[test]
    fn test_circle_perimeter() {
        let center = point_new(0.0, 0.0);
        let c = circle_new(center, 1.0);
        assert_eq!(circle_perimeter(&c), 2.0 * pi);
    }

    #[test]
    fn test_triangle_creation() {
        let a = point_new(0.0, 0.0);
        let b = point_new(1.0, 0.0);
        let c = point_new(0.0, 1.0);
        let t = triangle_new(a, b, c);
        assert_eq!(t.a, a);
        assert_eq!(t.b, b);
        assert_eq!(t.c, c);
    }

    #[test]
    fn test_triangle_area() {
        let a = point_new(0.0, 0.0);
        let b = point_new(1.0, 0.0);
        let c = point_new(0.0, 1.0);
        let t = triangle_new(a, b, c);
        assert!((triangle_area(&t) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_perimeter() {
        let a = point_new(0.0, 0.0);
        let b = point_new(1.0, 0.0);
        let c = point_new(0.0, 1.0);
        let t = triangle_new(a, b, c);
        let expected = 2.0 + 2.0_f64.sqrt();
        assert!((triangle_perimeter(&t) - expected).abs() < 1e-10);
    }

    #[test]
    fn test_rectangle_creation() {
        let top_left = point_new(3.0, 2.0);
        let bottom_right = point_new(0.0, 0.0);
        let r = rectangle_new(top_left, bottom_right);
        assert_eq!(r.top_left, top_left);
        assert_eq!(r.bottom_right, bottom_right);
    }

    #[test]
    fn test_rectangle_area() {
        let top_left = point_new(3.0, 2.0);
        let bottom_right = point_new(0.0, 0.0);
        let r = rectangle_new(top_left, bottom_right);
        assert_eq!(rectangle_area(&r), 6.0);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let top_left = point_new(3.0, 2.0);
        let bottom_right = point_new(0.0, 0.0);
        let r = rectangle_new(top_left, bottom_right);
        assert_eq!(rectangle_perimeter(&r), 10.0);
    }

    #[test]
    fn test_shape_enum() {
        let p = point_new(1.0, 1.0);
        let c = circle_new(p, 2.0);
        let p_shape = Shape::Point(p);
        let c_shape = Shape::Circle(c);

        assert_eq!(shape_area(&p_shape), 0.0);
        assert_eq!(shape_perimeter(&p_shape), 0.0);
        assert_eq!(shape_area(&c_shape), 4.0 * pi);
        assert_eq!(shape_perimeter(&c_shape), 4.0 * pi);
    }
}
