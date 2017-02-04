#![allow(unused_variables)]
#![allow(dead_code)]


fn main() {
    //literal matching
    let x = 1;

    match x {
        1 => println!("one"),
        _ => println!("any"),
    }

    //binding match
    let c = 'c';

    match c {
        x => println!("x: {} c: {}", x, c), //shadows x
    }

    println!("x: {:?}", x);

    //multiple patterns
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("any"),
    }

    //desctructuring inside match
    struct Pnt {
        x: i32,
        y: i32,
    }

    let point = Pnt { x: 5, y: 6 };

    match point {
        Pnt { x, y } => println!("({}, {})", x, y),
    }

    match point {
        Pnt { x, .. } => println!("x is {}", x), //omit field
    }

    match point {
        Pnt { y, .. } => println!("y is {}", y), //omit field
    }

    //binding ignore
    let res_ok: Result<i32, i32> = Ok(10);
    let res_err: Result<i32, i32> = Err(0);

    match res_ok {
        Ok(val) => println!("got val of: {}", val),
        Err(_) => println!("an error occured"),
    }

    match res_err {
        Ok(val) => println!("got val of: {}", val),
        Err(_) => println!("an error occured"),
    }

    fn coord() -> (i32, i32, i32) {
        (0, 1, 2)
    }
    let (x, _, z) = coord();
    println!("x: {}, z: {}", x, z);

    let tuple: (u32, String) = (5, String::from("five"));

    // Here, tuple is moved, because the String moved:
    let (x, _s) = tuple;

    // The next line would give "error: use of partially moved value: `tuple`"
    // println!("Tuple is: {:?}", tuple);

    // However,

    let tuple = (5, String::from("five"));

    // Here, tuple is _not_ moved, as the String was never moved, and u32 is Copy:
    let (x, _) = tuple;

    // That means this works:
    println!("Tuple is: {:?}", tuple);
    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }

    let x = OptionalTuple::Value(5, -2, 3);

    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"), //disregard multiple fields
        OptionalTuple::Missing => println!("No such luck."),
    }

    //references
    let x = 5;
    match x {
        ref r => println!("Got a ref to {}", r),
    }

    let mut x = 5;
    match x {
        ref mut r => println!("Got a mut ref to {}", r),
    }

    //ranges
    let x = '💅';

    match x {
        'a'...'z' => println!("lowercase letter"),
        _ => println!("sg else"),
    }

    //val binding
    let x = 5;

    match x {
        e @ 1...5 | e @ 8...10 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    //match guards
    enum OptInt {
        Value(i32),
        Missing,
    }

    let x = OptInt::Value(10);

    match x {
        OptInt::Value(i) if i % 2 == 0 => println!("Got an even int"),
        OptInt::Value(_) => println!("Got an odd int"),
        OptInt::Missing => println!("NaI"),
    }
}
