fn main() {
    let mut a = 5;
    {
        let y = &mut a;
    }
    a = 10;
    a = 20;
    println!("Hello, world! {} {}", a, y);
}
