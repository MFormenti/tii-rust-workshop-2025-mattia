use std::f64::consts::PI as pi;
use std::fmt::Debug;

pub trait Shape: Debug {
    const NAME: &'static str;
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
    fn biggest_area<'a, S>(&'a self, other: &'a S) -> Either<&'a Self, &'a S>
    where
        S: Shape + 'a,
    {
        if self.area() > other.area() {
            Either::Left(self)
        } else {
            Either::Right(other)
        }
    }
    fn print_properties(&self) {
        println!("Shape: {}", Self::NAME);
        println!("Area: {}", self.area());
        println!("Perimeter: {}", self.perimeter());
    }
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
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

impl Shape for Point {
    const NAME: &'static str = "Point";
    fn perimeter(&self) -> f64 {
        0.0
    }
    fn area(&self) -> f64 {
        0.0
    }
    fn scale(&mut self, _factor: f32) {}
}

impl Shape for Circle {
    const NAME: &'static str = "Circle";
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

impl Shape for Rectangle {
    const NAME: &'static str = "Rectangle";
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

impl Shape for Triangle {
    const NAME: &'static str = "Triangle";
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

impl Shape for DynamicShape {
    const NAME: &'static str = "DynamicShape";
    fn perimeter(&self) -> f64 {
        match self {
            DynamicShape::PointShape(p) => p.perimeter(),
            DynamicShape::CircleShape(c) => c.perimeter(),
            DynamicShape::RectangleShape(r) => r.perimeter(),
            DynamicShape::TriangleShape(t) => t.perimeter(),
        }
    }
    fn area(&self) -> f64 {
        match self {
            DynamicShape::PointShape(p) => p.area(),
            DynamicShape::CircleShape(c) => c.area(),
            DynamicShape::RectangleShape(r) => r.area(),
            DynamicShape::TriangleShape(t) => t.area(),
        }
    }
    fn scale(&mut self, factor: f32) {
        match self {
            DynamicShape::PointShape(p) => p.scale(factor),
            DynamicShape::CircleShape(c) => c.scale(factor),
            DynamicShape::RectangleShape(r) => r.scale(factor),
            DynamicShape::TriangleShape(t) => t.scale(factor),
        }
    }
}

pub fn find_biggest_ratio<'a, T: Shape, U: Shape>(
    slice1: &'a [T],
    slice2: &'a [U],
) -> Option<Either<&'a T, &'a U>> {
    let mut max_ratio = 0.0;
    let mut result = None;
    for s in slice1 {
        let ratio = s.perimeter() / s.area();
        if ratio.is_finite() && ratio > max_ratio {
            max_ratio = ratio;
            result = Some(Either::Left(s));
        }
    }
    for s in slice2 {
        let ratio = s.perimeter() / s.area();
        if ratio.is_finite() && ratio > max_ratio {
            max_ratio = ratio;
            result = Some(Either::Right(s));
        }
    }
    if let Some(ref either) = result {
        match either {
            Either::Left(s) => println!("Found in first slice:  {:?}", s),
            Either::Right(s) => println!("Found in second slice: {:?}", s),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point() {
        let mut p = Point { x: 1.0, y: 2.0 };
        assert_eq!(p.perimeter(), 0.0);
        assert_eq!(p.area(), 0.0);
        assert_eq!(p.area_to_perimeter(), 0.0);
        p.scale(5.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }
}
