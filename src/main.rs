use std::vec;

fn main() {
    let n: &i32 = &2_i32;
    let m: &i32 = &(*n +1);
    println!("{} and {}",* n, *m);
    let b: &'static str = "Hello, world."; //b: &'static str
//    b.whatisthis
    println!("{}", b);
    let mut a: vec::Vec<i32> = Vec::new() ;
    for i in 1..4 {(&mut a).push(i)};
//    a.whatisthis;
    let c = & a;
    println!("{:?} ex a", a);
    println!("{:?} ex a", a);
    println!("{:?} ex a", a);
    println!("{:?} ex &a", &a);

    println!("{:?} ex &c",& c);

}
