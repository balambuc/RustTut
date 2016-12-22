fn main() {
    //bools
    let x: bool = true;
    println!("{}",x);

    //chars
    let x ='💕';
    println!("{}",x );

    //numeric
    let x: i8 = 100; // i/u {8,16,32,64,size}
    println!("{}",x);
    let x: f64 = 3.14;
    println!("{}",x);

    //arrays
    let a: [i32; 5] = [1,2,3,4,5];
    let l=a.len();
    println!("\na[2] is {} of an array len of {}",a[2],l );
    let a = ["lel"; 5];
    for i in 0..a.len() {
        println!("a[{}] is: {}",i, a[i] );
    }

    //slices
    let a = [2,3,5,7,11];
    println!("\n{:?}",a );
    let slice=&a[..]; // full slice
    println!("{:?}",slice );
    let slice=&slice[1..3]; // mid slice
    println!("{:?}",slice );

    //tuples
    let x = ("life", 42);
    println!("{:?}",x );
    let y: (&str, i32) = x;
    println!("{:?}\n {}", y, y.0);
    let (x, y) = y;
    println!("{} {}",x,y );
}
