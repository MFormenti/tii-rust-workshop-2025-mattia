use p22::figures::*;
use std::f64::consts::PI as pi;

#[test]
fn test_shapes_combinations() {
    // Create a point
    let p = point_new(0.0, 0.0);

    // Create shapes based on the point
    let c = circle_new(p, 5.0);
    let t = triangle_new(p, point_new(3.0, 0.0), point_new(0.0, 4.0));
    let r = rectangle_new(p, point_new(3.0, 4.0));

    // Verify correct calculations
    assert_eq!(circle_area(&c), 25.0 * pi);
    assert_eq!(circle_perimeter(&c), 10.0 * pi);

    // Triangle should be a 3-4-5 triangle with area 6
    assert!((triangle_area(&t) - 6.0).abs() < 1e-10);
    assert!((triangle_perimeter(&t) - 12.0).abs() < 1e-10);

    assert_eq!(rectangle_area(&r), 12.0);
    assert_eq!(rectangle_perimeter(&r), 14.0);
}

#[test]
fn test_shape_enum_composite() {
    // Create different shapes
    let p = point_new(0.0, 0.0);
    let c = circle_new(p, 2.0);
    let t = triangle_new(p, point_new(3.0, 0.0), point_new(0.0, 4.0));
    let r = rectangle_new(p, point_new(2.0, 3.0));

    // Create Shape enums
    let shape_p = shape_new(Shape::Point(p));
    let shape_c = shape_new(Shape::Circle(c));
    let shape_t = shape_new(Shape::Triangle(t));
    let shape_r = shape_new(Shape::Rectangle(r));

    // Calculate total area and perimeter
    let total_area = shape_area(&shape_p) + shape_area(&shape_c) +
        shape_area(&shape_t) + shape_area(&shape_r);

    let total_perimeter = shape_perimeter(&shape_p) + shape_perimeter(&shape_c) +
        shape_perimeter(&shape_t) + shape_perimeter(&shape_r);

    // Expected values
    let expected_area = 0.0 + (4.0 * pi) + 6.0 + 6.0;
    let expected_perimeter = 0.0 + (4.0 * pi) + 12.0 + 10.0;

    assert!((total_area - expected_area).abs() < 1e-10);
    assert!((total_perimeter - expected_perimeter).abs() < 1e-10);
}

#[test]
fn test_nested_shapes() {
    // Create a scenario where shapes are nested
    let center = point_new(0.0, 0.0);

    // A circle with radius 10
    let outer_circle = circle_new(center, 10.0);

    // A rectangle inside the circle
    let rect_width = 10.0;
    let rect_height = 6.0;
    let rect_top_left = point_new(-rect_width/2.0, rect_height/2.0);
    let rect_bottom_right = point_new(rect_width/2.0, -rect_height/2.0);
    let inner_rectangle = rectangle_new(rect_top_left, rect_bottom_right);

    // A triangle inside the rectangle
    let tri_a = point_new(-2.0, 2.0);
    let tri_b = point_new(2.0, 2.0);
    let tri_c = point_new(0.0, -2.0);
    let inner_triangle = triangle_new(tri_a, tri_b, tri_c);

    // A point at the center
    let inner_point = center;

    // Calculate areas and verify relationships
    let circle_a = circle_area(&outer_circle);
    let rect_a = rectangle_area(&inner_rectangle);
    let tri_a = triangle_area(&inner_triangle);
    let point_a = point_area(&inner_point);

    // The areas should be decreasing
    assert!(circle_a > rect_a);
    assert!(rect_a > tri_a);
    assert!(tri_a > point_a);

    // Convert to shapes and verify same results
    let shape_circle = Shape::Circle(outer_circle);
    let shape_rectangle = Shape::Rectangle(inner_rectangle);
    let shape_triangle = Shape::Triangle(inner_triangle);
    let shape_point = Shape::Point(inner_point);

    assert_eq!(shape_area(&shape_circle), circle_a);
    assert_eq!(shape_area(&shape_rectangle), rect_a);
    assert_eq!(shape_area(&shape_triangle), tri_a);
    assert_eq!(shape_area(&shape_point), point_a);
}

#[test]
fn test_point_distance_properties() {
    // Create several points
    let p1 = point_new(0.0, 0.0);
    let p2 = point_new(3.0, 4.0);
    let p3 = point_new(6.0, 8.0);

    // Test distance properties

    // Distance to self is zero
    assert_eq!(point_distance(&p1, &p1), 0.0);

    // Distance is symmetric
    assert_eq!(point_distance(&p1, &p2), point_distance(&p2, &p1));

    // Triangle inequality
    let d12 = point_distance(&p1, &p2);
    let d23 = point_distance(&p2, &p3);
    let d13 = point_distance(&p1, &p3);
    assert!(d12 + d23 >= d13);
}

#[test]
fn test_complex_shape_combinations() {
    // Create a few shapes
    let origin = point_new(0.0, 0.0);
    let c1 = circle_new(origin, 1.0);
    let c2 = circle_new(point_new(3.0, 0.0), 2.0);

    // Calculate distance between circle centers
    let distance = point_distance(&c1.center, &c2.center);
    assert_eq!(distance, 3.0);

    // Check if circles overlap (distance between centers < sum of radii)
    let sum_of_radii = c1.radius + c2.radius;
    let circles_overlap = distance < sum_of_radii;
    assert!(circles_overlap);

    // Calculate total area covered by both circles
    // Note: This is a simplification that doesn't account for overlap
    let total_area = circle_area(&c1) + circle_area(&c2);
    let expected_area = pi * (1.0*1.0 + 2.0*2.0);
    assert_eq!(total_area, expected_area);
}