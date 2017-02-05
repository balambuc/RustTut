use std::f64::consts::PI as pi;

trait RoundTo {
    fn round_to(&self, n: i32) -> f64;
}

impl RoundTo for f64 {
    fn round_to(&self, n: i32) -> f64 {
        let mul = 10.0_f64.powi(n);
        (self * mul).round() / mul
    }
}

fn main() {
    struct Circle {
        x: f64,
        y: f64,
        r: f64,
    }
    impl Circle {
        fn area(&self) -> f64 {
            self.r * self.r * pi
        }
        fn grow(&self, inc: f64) -> Circle {
            Circle {
                x: self.x,
                y: self.y,
                r: self.r + inc,
            }
        }
        //associated(static) method
        fn new(x: f64, y: f64, r: f64) -> Circle {
            Circle { x: x, y: y, r: r }
        }
    }

    let circle = Circle {
        x: 10.0,
        y: 10.0,
        r: 1.0 / pi.sqrt(),
    };

    match circle {
        Circle { x, y, .. } => {
            println!("Area of circle({}, {}) is {}",
                     x,
                     y,
                     circle.area().round_to(3))
        }
    }

    let grow_by = 2.0;

    match circle {
        Circle { x, y, .. } => {
            println!("Area of circle({}, {}) before growth is {}.\nAfter growth it is {}",
                     x,
                     y,
                     circle.area().round_to(3),
                     circle.grow(grow_by).area().round_to(3))
        }
    }

    let c = Circle::new(5.0, 2.0, pi);
    println!("Area of new circ is {:?}", c.area().round_to(3));

    struct CircBuild {
        x: f64,
        y: f64,
        r: f64,
    }

    impl CircBuild {
        fn new() -> CircBuild {
            CircBuild {
                x: 0.0,
                y: 0.0,
                r: 1.0,
            }
        }

        fn x(&mut self, n: f64) -> &mut CircBuild {
            self.x = n;
            self
        }

        fn y(&mut self, n: f64) -> &mut CircBuild {
            self.y = n;
            self
        }

        fn r(&mut self, n: f64) -> &mut CircBuild {
            self.r = n;
            self
        }

        fn fin(&self) -> Circle {
            Circle {
                x: self.x,
                y: self.y,
                r: self.r,
            }
        }
    }

    let b_c = CircBuild::new()
        .x(1.0)
        .y(2.0)
        .r(2.0)
        .fin();

    match b_c {
        Circle { x, y, .. } => {
            println!("Area of circle({}, {}) is {}", x, y, b_c.area().round_to(3))
        }
    }
}
