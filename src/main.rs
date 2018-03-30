extern crate lib1;
extern crate lib2;
extern crate lib3;

fn main() {
    println!("Hello, world!");
    lib1::function();
    lib2::function();
    lib3::function();
}

#[test]
fn test1() {
    println!("test1");
}

#[test]
fn test2() {
    println!("test2");
}
