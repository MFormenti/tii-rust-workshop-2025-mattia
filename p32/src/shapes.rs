use std::f64::consts::PI as pi;
use std::fmt::Debug;

pub trait ShapeCore {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn scale(&mut self, factor: f32);

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

pub trait ShapeName {
    const NAME: &'static str;
}

pub trait Shape: ShapeCore + ShapeName + Sized {
    fn biggest_area<'a, S: Shape>(&'a self, other: &'a S) -> ShapeBiggest<'a, Self, S> {
        if self.area() > other.area() {
            ShapeBiggest::First(self)
        } else {
            ShapeBiggest::Second(other)
        }
    }

    fn print_properties(&self)
    where
        Self: Debug,
    {
        println!("Shape: {}", Self::NAME);
        println!("Area: {}", self.area());
        println!("Perimeter: {}", self.perimeter());
    }
}

impl<T: ShapeCore + ShapeName> Shape for T {}

pub trait ShapeNamedDebug: Shape + Debug {}
impl<T: Shape + Debug> ShapeNamedDebug for T {}

pub enum ShapeBiggest<'a, T: Shape, U: Shape> {
    First(&'a T),
    Second(&'a U),
}

pub enum ShapeSource<'a, T, U>
where
    T: Shape + Debug,
    U: Shape + Debug,
{
    FirstSlice(&'a T),
    SecondSlice(&'a U),
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

#[derive(Debug)]
pub enum DynamicShape {
    PointShape(Point),
    CircleShape(Circle),
    RectangleShape(Rectangle),
    TriangleShape(Triangle),
}

impl ShapeCore for Point {
    fn perimeter(&self) -> f64 {
        0.0
    }

    fn area(&self) -> f64 {
        0.0
    }

    fn scale(&mut self, _factor: f32) {}
}

impl ShapeCore for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * pi * self.radius
    }

    fn area(&self) -> f64 {
        pi * self.radius * self.radius
    }

    fn scale(&mut self, factor: f32) {
        self.radius *= factor as f64;
    }
}

impl ShapeCore for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, factor: f32) {
        self.width *= factor as f64;
        self.height *= factor as f64;
    }
}

impl ShapeCore for Triangle {
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }

    fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    fn scale(&mut self, factor: f32) {
        self.a *= factor as f64;
        self.b *= factor as f64;
        self.c *= factor as f64;
    }
}

impl ShapeCore for DynamicShape {
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

    fn scale(&mut self, factor: f32) {
        match self {
            DynamicShape::PointShape(point) => point.scale(factor),
            DynamicShape::CircleShape(circle) => circle.scale(factor),
            DynamicShape::RectangleShape(rectangle) => rectangle.scale(factor),
            DynamicShape::TriangleShape(triangle) => triangle.scale(factor),
        }
    }
}

impl ShapeName for Point {
    const NAME: &'static str = "Point";
}

impl ShapeName for Circle {
    const NAME: &'static str = "Circle";
}

impl ShapeName for Rectangle {
    const NAME: &'static str = "Rectangle";
}

impl ShapeName for Triangle {
    const NAME: &'static str = "Triangle";
}

impl ShapeName for DynamicShape {
    const NAME: &'static str = "DynamicShape";
}

pub fn find_biggest_ratio<'a, T, U>(
    slice1: &'a [T],
    slice2: &'a [U],
) -> Option<ShapeSource<'a, T, U>>
where
    T: Shape + Debug,
    U: Shape + Debug,
{
    let mut max_ratio = 0.0;
    let mut result = None;

    for shape in slice1 {
        let ratio = shape.perimeter() / shape.area();
        if ratio.is_finite() && ratio > max_ratio {
            max_ratio = ratio;
            result = Some(ShapeSource::FirstSlice(shape));
        }
    }

    for shape in slice2 {
        let ratio = shape.perimeter() / shape.area();
        if ratio.is_finite() && ratio > max_ratio {
            max_ratio = ratio;
            result = Some(ShapeSource::SecondSlice(shape));
        }
    }

    if let Some(src) = &result {
        match src {
            ShapeSource::FirstSlice(shape) => println!("Found in first slice:  {:?}", shape),
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
        assert_eq!(Point::NAME, "Point");

        point.scale(2.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
    }

    #[test]
    fn test_circle() {
        let mut circle = Circle { radius: 5.0 };
        assert_eq!(circle.perimeter(), 2.0 * pi * 5.0);
        assert_eq!(circle.area(), pi * 25.0);
        assert_eq!(Circle::NAME, "Circle");

        let expected_ratio = (pi * 25.0) / (2.0 * pi * 5.0);
        assert_eq!(circle.area_to_perimeter(), expected_ratio);

        circle.scale(2.0);
        assert_eq!(circle.radius, 10.0);
    }

    #[test]
    fn test_rectangle() {
        let mut rect = Rectangle {
            width: 4.0,
            height: 5.0,
        };
        assert_eq!(rect.perimeter(), 2.0 * (4.0 + 5.0));
        assert_eq!(rect.area(), 20.0);
        assert_eq!(Rectangle::NAME, "Rectangle");

        rect.scale(0.5);
        assert_eq!(rect.width, 2.0);
        assert_eq!(rect.height, 2.5);
    }

    #[test]
    fn test_triangle() {
        let mut tri = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };
        assert_eq!(tri.perimeter(), 12.0);
        assert_eq!(tri.area(), 6.0);
        assert_eq!(Triangle::NAME, "Triangle");

        tri.scale(3.0);
        assert_eq!(tri.a, 9.0);
        assert_eq!(tri.b, 12.0);
        assert_eq!(tri.c, 15.0);
        assert_eq!(tri.area(), 54.0);
    }

    #[test]
    fn test_dynamic_shape() {
        let mut dynamic = DynamicShape::CircleShape(Circle { radius: 5.0 });
        assert_eq!(dynamic.perimeter(), 2.0 * pi * 5.0);
        assert_eq!(DynamicShape::NAME, "DynamicShape");

        dynamic = DynamicShape::RectangleShape(Rectangle {
            width: 3.0,
            height: 4.0,
        });
        assert_eq!(dynamic.area(), 12.0);
    }

    #[test]
    fn test_biggest_area() {
        let circle = Circle { radius: 5.0 };
        let rect = Rectangle {
            width: 10.0,
            height: 10.0,
        };

        let bigger = circle.biggest_area(&rect);

        match bigger {
            ShapeBiggest::First(_) => panic!("Expected rectangle to have bigger area"),
            ShapeBiggest::Second(_) => {}
        }
    }

    #[test]
    fn test_find_biggest_ratio() {
        let shapes1 = [Circle { radius: 10.0 }, Circle { radius: 5.0 }];

        let shapes2 = [
            Rectangle {
                width: 10.0,
                height: 10.0,
            },
            Rectangle {
                width: 3.0,
                height: 4.0,
            },
        ];

        let triangle = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };
        let triangle_array = [triangle];

        let result = find_biggest_ratio::<Circle, Rectangle>(&shapes1, &shapes2);

        let result_with_triangle =
            find_biggest_ratio::<Circle, Triangle>(&shapes1, &triangle_array);

        match result_with_triangle {
            Some(ShapeSource::SecondSlice(shape)) => {
                assert_eq!(shape.perimeter(), 12.0);
                assert_eq!(shape.area(), 6.0);
            }
            _ => {}
        }

        assert!(result.is_some() || result_with_triangle.is_some());
    }

    #[test]
    fn test_print_properties() {
        let circle = Circle { radius: 5.0 };
        circle.print_properties();

        let rect = Rectangle {
            width: 4.0,
            height: 5.0,
        };
        rect.print_properties();
    }
}
