#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    enum Opt<T> {
        Some(T),
        None,
    }

    let x: Opt<i32> = Opt::Some(3);
    let y: Opt<f64> = Opt::Some(5.0);

    enum Res<T, E> {
        Ok(T),
        Err(E),
    }

    let x: Res<i32, &'static str> = Res::Ok(3);
    let y: Res<i32, &'static str> = Res::Err("404");

    match x {
        Res::Ok(x) => println!("Success {}", x),
        Res::Err(e) => println!("Error {}", e),
    }

    match y {
        Res::Ok(x) => println!("Success {}", x),
        Res::Err(e) => println!("Error {}", e),
    }


    fn swap<T>(x: &mut T, y: &mut T) {
        std::mem::swap(x, y);
    }

    let (mut x, mut y) = (3, 6);
    println!("x: {} y: {}", x, y);
    swap(&mut x, &mut y);
    println!("x: {} y: {}", x, y);

    let (mut x, mut y) = ("Hello ", "World! ");
    println!("{}{}", x, y);
    swap(&mut x, &mut y);
    println!("{}{}", x, y);

    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn swap(&mut self) {
            std::mem::swap(&mut self.x, &mut self.y);
        }
    }

    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };

    let mut pnt = Point { x: 1, y: 2 };
    println!("({}, {})", pnt.x, pnt.y);
    pnt.swap();
    println!("({}, {})", pnt.x, pnt.y);
}
