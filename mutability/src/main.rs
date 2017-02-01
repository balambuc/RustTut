#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::sync::Arc;
use std::cell::RefCell;
use std::cell::Cell;

fn main() {
    let x = 5; //immutable binding
    /* x = 6 // error! */

    let mut x = 5;
    let mut z = 7;
    let mut w = 8;
    x = 6; //much bettah' :D
    let y = &mut x; // immutable binding to mutable ref
    /*y = &mut 7 // won't work*/
    *y = 5; //dereferencing mutable ref;
    let mut y = &mut z; //mutable bind to mut ref
    y = &mut w; //this works now

    //interior vs exterior mutability
    let x = Arc::new(5);
    let y = x.clone();
    let z = y.clone(); //works, because Arc is internally mutable and clone hands &T

    let x = RefCell::new(5);
    let y = x.borrow_mut(); //works, because RefCell is internally mutable
    /*let z = x.borrow_mut(); // won't coz borrow_mut hands &mut T*/
    /*struct P {
        x: i32,
        mut y: i32, //nononononono
    }*/

    struct P {
        x: i32,
        y: i32,
    }
    let p = P { x: 5, y: 6 }; // all fields are immutable
    let mut p = P { x: 5, y: 6 };
    p.x = 6;
    p.y = 7;

    //for field-level mut: Cell<T>

    struct FLP {
        x: i32,
        y: Cell<i32>,
    }

    let p = FLP {
        //immutable bind
        x: 5,
        y: Cell::new(5), // internally mut
    };
    println!("{:?}, {:?}", p.x, p.y.get());

    p.y.set(7);

    println!("{:?}, {:?}", p.x, p.y.get());


}
