fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x

    *y = 10;
    *z = 15;
    println!("x = {}", x); //This will print 15, but not in a predictable way, it is undefined behavior
}