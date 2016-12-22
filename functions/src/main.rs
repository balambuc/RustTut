fn main() {
    print_num(5);
    println!("5 + 6 = {}", add(5,6));
    let sum: fn(i32, i32) -> i32 = add;
    println!("5 + 6 = {}", sum(5,6));
}

fn print_num(x: i32) {
    println!("x is: {}", x);
}

fn add(x:i32, y:i32) -> i32 {
    x+y
}
