fn main() {
    println!("Hello, world!");

    let mut x = 123;
    p(x);
    x = 3;
}

fn p(x : i32)
{
    println!("x = {}", x);
}