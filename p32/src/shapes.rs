use std::f64::consts::PI as pi;
use std::fmt::Debug;

pub trait ShapeNamedDebug: Shape + Named + Debug {
    fn print_properties(&self) {
        println!("Shape: {}", self.name());
        println!("Area: {}",   self.area());
        println!("Perimeter: {}", self.perimeter());
    }
}
impl<T: Shape + Named + Debug> ShapeNamedDebug for T {}

pub trait Shape {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn scale(&mut self, factor:f32);

    fn area_to_perimeter(&self) -> f64 {
        let area = self.area();
        let perimeter = self.perimeter();
        if perimeter == 0.0 {
            0.0
        } else {
            area / perimeter
        }
    }

}

pub trait Named {
    fn name(&self) -> &'static str;
}


fn biggest_area<'a, S1, S2>(
    a: &'a S1,
    b: &'a S2
) -> &'a dyn ShapeNamedDebug
where
    S1: Shape + Named + Debug + 'a,
    S2: Shape + Named + Debug + 'a,
{
    if a.area() > b.area() {
        a as &(dyn ShapeNamedDebug)
    } else {
        b as &(dyn ShapeNamedDebug)
    }
}

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}
#[derive(Debug)]
pub struct Circle {
    radius: f64,
}

#[derive(Debug)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug)]
pub struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

#[derive(Debug)]
pub enum DynamicShape {
    PointShape(Point),
    CircleShape(Circle),
    RectangleShape(Rectangle),
    TriangleShape(Triangle),
}

impl Shape for Point {

    fn perimeter(&self) -> f64 {
        0.0
    }

    fn area(&self) -> f64 {
        0.0
    }

    fn scale(&mut self, _factor: f32) {}
}

impl Shape for Circle {

    fn perimeter(&self) -> f64 {
        2.0 * pi * self.radius
    }

    fn area(&self) -> f64 {
        pi * self.radius * self.radius
    }

    fn scale(&mut self, _factor: f32) {
        self.radius *= _factor as f64;
    }
}

impl Shape for Rectangle {

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, _factor: f32) {
        self.width *= _factor as f64;
        self.height *= _factor as f64;
    }
}

impl Shape for Triangle {

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }

    fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    fn scale(&mut self, _factor: f32) {
        self.a *= _factor as f64;
        self.b *= _factor as f64;
        self.c *= _factor as f64;
    }
}

impl Shape for DynamicShape {

    fn perimeter(&self) -> f64 {
        match self {
            DynamicShape::PointShape(point) => point.perimeter(),
            DynamicShape::CircleShape(circle) => circle.perimeter(),
            DynamicShape::RectangleShape(rectangle) => rectangle.perimeter(),
            DynamicShape::TriangleShape(triangle) => triangle.perimeter(),
        }
    }

    fn area(&self) -> f64 {
        match self {
            DynamicShape::PointShape(point) => point.area(),
            DynamicShape::CircleShape(circle) => circle.area(),
            DynamicShape::RectangleShape(rectangle) => rectangle.area(),
            DynamicShape::TriangleShape(triangle) => triangle.area(),
        }
    }

    fn scale(&mut self, _factor: f32) {
        match self {
            DynamicShape::PointShape(point) => point.scale(_factor),
            DynamicShape::CircleShape(circle) => circle.scale(_factor),
            DynamicShape::RectangleShape(rectangle) => rectangle.scale(_factor),
            DynamicShape::TriangleShape(triangle) => triangle.scale(_factor),
        }
    }
}


impl Named for Point {
    fn name(&self) -> &'static str { "Point" }
}

impl Named for Circle {
    fn name(&self) -> &'static str { "Circle" }
}

impl Named for Rectangle {
    fn name(&self) -> &'static str { "Rectangle" }
}

impl Named for Triangle {
    fn name(&self) -> &'static str { "Triangle" }
}
impl Named for DynamicShape {
    fn name(&self) -> &'static str { "DynamicShape" }
}

pub enum ShapeSource<'a> {
    FirstSlice(&'a dyn ShapeNamedDebug),
    SecondSlice(&'a dyn ShapeNamedDebug),
}

pub fn find_biggest_ratio<'a, T: Shape + Named + Debug, U: Shape + Named + Debug>(
    slice1: &'a [T],
    slice2: &'a [U],
) -> Option<ShapeSource<'a>> {
    let mut max_ratio = 0.0;
    let mut result = None;

    for shape in slice1 {
        let ratio = shape.perimeter() / shape.area();
        if ratio.is_finite() && ratio > max_ratio {
            max_ratio = ratio;
            // coerce into your object‑safe super‑trait
            result = Some(ShapeSource::FirstSlice(shape as &dyn ShapeNamedDebug));
        }
    }

    for shape in slice2 {
        let ratio = shape.perimeter() / shape.area();
        if ratio.is_finite() && ratio > max_ratio {
            max_ratio = ratio;
            result = Some(ShapeSource::SecondSlice(shape as &dyn ShapeNamedDebug));
        }
    }

    if let Some(src) = &result {
        match src {
            ShapeSource::FirstSlice(shape)  => println!("Found in first slice:  {:?}", shape),
            ShapeSource::SecondSlice(shape) => println!("Found in second slice: {:?}", shape),
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let mut point = Point { x: 1.0, y: 2.0 };
        assert_eq!(point.perimeter(), 0.0);
        assert_eq!(point.area(), 0.0);
        assert_eq!(point.area_to_perimeter(), 0.0);

        // Scale shouldn't affect a point
        point.scale(2.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
    }

    #[test]
    fn test_circle() {
        let mut circle = Circle { radius: 5.0 };
        assert_eq!(circle.perimeter(), 2.0 * pi * 5.0);
        assert_eq!(circle.area(), pi * 25.0);

        // Test area_to_perimeter ratio
        let expected_ratio = (pi * 25.0) / (2.0 * pi * 5.0);
        assert_eq!(circle.area_to_perimeter(), expected_ratio);

        // Test scaling
        circle.scale(2.0);
        assert_eq!(circle.radius, 10.0);
    }

    #[test]
    fn test_rectangle() {
        let mut rect = Rectangle { width: 4.0, height: 5.0 };
        assert_eq!(rect.perimeter(), 2.0 * (4.0 + 5.0));
        assert_eq!(rect.area(), 20.0);

        rect.scale(0.5);
        assert_eq!(rect.width, 2.0);
        assert_eq!(rect.height, 2.5);
    }

    #[test]
    fn test_triangle() {
        // Create a 3-4-5 right triangle
        let mut tri = Triangle { a: 3.0, b: 4.0, c: 5.0 };
        assert_eq!(tri.perimeter(), 12.0);
        assert_eq!(tri.area(), 6.0);

        tri.scale(3.0);
        assert_eq!(tri.a, 9.0);
        assert_eq!(tri.b, 12.0);
        assert_eq!(tri.c, 15.0);
        assert_eq!(tri.area(), 54.0);  // Area scales by factor²
    }

    #[test]
    fn test_dynamic_shape() {
        let mut dynamic = DynamicShape::CircleShape(Circle { radius: 5.0 });
        assert_eq!(dynamic.perimeter(), 2.0 * pi * 5.0);

        // Change to a rectangle
        dynamic = DynamicShape::RectangleShape(Rectangle { width: 3.0, height: 4.0 });
        assert_eq!(dynamic.area(), 12.0);
    }

    #[test]
    fn test_biggest_area() {
        let circle = Circle { radius: 5.0 };
        let rect = Rectangle { width: 10.0, height: 10.0 };

        let bigger = biggest_area(&circle, &rect);
        // Circle area = π·25 ≈ 78.5, Rectangle area = 100
        assert_eq!(bigger.area(), 100.0);
    }

    #[test]
    fn test_find_biggest_ratio() {
        // For perimeter/area ratio:
        // - Circle with radius 10: perimeter = 2π·10, area = π·100, ratio = 2π·10/(π·100) = 0.2
        // - Square with side 10: perimeter = 40, area = 100, ratio = 40/100 = 0.4
        // - Triangle (3,4,5): perimeter = 12, area = 6, ratio = 12/6 = 2.0

        let shapes1 = [
            DynamicShape::CircleShape(Circle { radius: 10.0 }),
            DynamicShape::CircleShape(Circle { radius:  5.0 }),
        ];
        let shapes2 = [
            DynamicShape::RectangleShape(Rectangle { width: 10.0, height: 10.0 }),
            DynamicShape::TriangleShape( Triangle   { a: 3.0,  b: 4.0, c: 5.0  }),
        ];

        let result = find_biggest_ratio(&shapes1, &shapes2);

        // The triangle should have the highest ratio
        match result {
            Some(ShapeSource::SecondSlice(shape)) => {
                assert_eq!(shape.perimeter(), 12.0);
                assert_eq!(shape.area(), 6.0);
            },
            _ => panic!("Expected to find the triangle with highest ratio")
        }
    }
}


