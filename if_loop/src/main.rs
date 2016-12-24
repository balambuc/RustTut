fn main() {
    let x=5;
    if x == 5 {
        println!("x öt");
    }
    else if x == 6 {
        println!("x hat");
    }
    else {
        println!("x se nem öt se nem hat");
    }

    let y = if x == 5 {10} else {15}; // ternary-like
    println!("{:?}",y );

    let mut x=10;
    loop {
        println!("infinite loop");
        if x == 0 {
            break;
        }
        else {
            x=x-1;
        }
    }
    let mut x=5;
    let mut done=false;
    while !done {
        x+=x-3;
        print!("{:?} ",x );
        if x%5 == 0 {done=true};
    }

    for i in 0..10 {
        println!("{:?}",i );
    }

    for (index, value) in (5..10).enumerate() {
        println!("index = {:?}, value = {:?}",index, value );
    }

    'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // continues the loop over x
        if y % 2 == 0 { continue 'inner; } // continues the loop over y
        println!("x: {}, y: {}", x, y);
    }
}
}
