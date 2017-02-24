fn main() {
    //string literal (&'static str)
    let greet = "Hello world";
    println!("{:?}", greet);
    //String
    let s_greet = greet.to_string();
    //indexing

    let n = 20;

    match s_greet.chars().nth(n - 1) {
        Some(x) => println!("char {} of greet is {}", n, x),
        None => println!("no {}th char", n),
    }

    //&String coerces to &str, thats why when concating only String+&str works

    let hello = "Hello ".to_string();
    let world = "world!";

    let hello_world = hello + world;
    println!("{:?}", hello_world);

    let hello = "Hello ".to_string();
    let world = "world!".to_string();

    let hello_world = hello + &world;
    println!("{:?}", hello_world);


}
