fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let v_rep = vec![0;5];
    let mut i: usize = 0; //vektor index can only be usize
    while i < 5 {
        println!("{:?}, {:?}", v[i], v_rep[i]);
        i += 1;
    }

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

    /*for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }*/
    //Can't use after taking ownership
}
