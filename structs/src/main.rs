

fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 }; // origin: Point

    println!("The origin is at ({}, {})", origin.x, origin.y);

    let point = Point { x: 5, ..origin }; // update synt

    println!("The point is at ({}, {})", point.x, point.y);

    // tuple struct: struct w/o field names
    struct Colour(i32, i32, i32);
    let purple = Colour(127, 0, 255);
    println!("This coulour is {:.1}% red, {:.1}% green, and {:.1}% blue.",
             purple.0 as f64 / 255.0 * 100.0,
             purple.1 as f64 / 255.0 * 100.0,
             purple.2 as f64 / 255.0 * 100.0);
}
