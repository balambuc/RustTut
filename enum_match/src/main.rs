#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    //enum (="struct collection"; sum type)
    enum Msg {
        Quit,
        ChgCol(i32, i32, i32),
        Mv { x: i32, y: i32 },
        Write(String),
    }

    let msg = Msg::ChgCol(0, 10, 100);
    //constr func
    let m = Msg::Write("Hello world".to_string()); //same as,
    fn foo(x: String) -> Msg {
        Msg::Write(x)
    }

    let x = foo("Hello world".to_string());

    //match, kinda like switch

    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("smg else"),
    }

    let y = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "smg else",
    };

    println!("{:?}", y);

    //enum matching
    fn quit() {
        println!("exiting app", );
    }
    fn change_color(r: i32, g: i32, b: i32) {
        println!("changing colour to: {},{},{}", r, g, b);
    }
    fn move_cursor(x: i32, y: i32) {
        println!("Moving to ({}, {})", x, y);
    }
    fn process(msg: Msg) {
        match msg {
            Msg::Quit => quit(),
            Msg::ChgCol(r, g, b) => change_color(r, g, b),
            Msg::Mv { x, y } => move_cursor(x, y),
            Msg::Write(s) => println!("{:?}", s),
        };
    }

    let msg = Msg::Quit;
    process(msg);
    let msg = Msg::ChgCol(255, 10, 0);
    process(msg);
    let msg = Msg::Mv { x: 10, y: 20 };
    process(msg);
    let msg = Msg::Write("Hi beauty".to_string());
    process(msg);


}
