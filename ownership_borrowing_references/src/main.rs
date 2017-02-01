#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    /// ownership

    let v = vec![1, 2, 3];

    // v2 takes ownership of v
    let v2 = v;

    // this would give an error of moved value
    /* println!("v[0] is {}", v[0]); */

    fn take(v: Vec<i32>) {
        println!("{:?}", v);
    }

    take(v2);

    // same problem
    /*println!("v[0] is {}", v2[0]);*/


    /// borrowing and references

    fn sum_vector(v: &Vec<i32>) -> i32 {
        //& = immutable borrow(const reference to)
        v.iter().fold(0, |a, b| a + b)
    }

    fn sum(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        sum_vector(v1) + sum_vector(v2)
    }

    let v1 = vec![4, 5, 6];
    let mut v2 = vec![1, 2, 3];
    println!("{:?}", sum(&v1, &v2));

    fn mutate(v: &mut Vec<i32>) {
        //mutable borrow
        println!("{:?}", v);
        v.push(7);
    }

    mutate(&mut v2);
    println!("{:?}", v2);

    /* rules of borrowing:
     * one or more references (&T) to a resource
     * exactly one mutable reference (&mut T)
     * one and only one of the above two!!!! */



    /// lifetimes
    /*let r;
    {
        let i = 1;
        r = &i;
    } // i is out of scope before r is used
    println!("{:?}", r);*/

    fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
        "Hello World!"
    }

    let (line, lang) = ("lang:en=Hello World!", "en");

    let v;
    {
        let p = format!("lang:{}", lang);
        v = skip_prefix(line, p.as_str());
    }
    println!("{:?}", v);

    struct FooLT<'a> {
        x: &'a i32,
    }

    impl<'a> FooLT<'a> {
        fn x(&self) -> &'a i32 {
            self.x
        }
    }

    let y = &5;
    let f = FooLT { x: y };
    println!("{:?}, {:?}", f.x, f.x());

    //lifetime elision
    fn print(s: &str) {} // elided
    fn ex_print<'a>(s: &'a str) {} // expanded

    fn debug(lvl: u32, s: &str) {} // elided
    fn ex_debug<'a>(lvl: u32, s: &'a str) {} // expanded


}
