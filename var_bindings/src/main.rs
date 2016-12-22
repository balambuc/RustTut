fn main() {
    let (x,y)=(1,2); //pattern match
    println!("{} + {} = {}",x,y, x+y );
    let mut x=10;
    println!("x = {:?}",x );
    x=2; //mutable reassign
    println!("x = {:?}",x );
    {
        let x=8; //shadowing x
        println!("x = {:?}",x );
    }
    println!("x = {:?}",x );
}
