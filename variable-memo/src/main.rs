const MAX_POINTS : u32 = 100_000;
fn main() {
    what_is_tuple();
    what_is_array();
    what_is_statement();
    what_is_if();
    println!("Hello, world!");
}




fn what_is_tuple()
{
    let tup = (1,"asahi", 1.0);
    let (x, y, z) = tup;
   
    let mut tup = (1,"asahi", 1.0);
    tup.0 = 4;
    //let (x, y) = tup; Error
}

fn what_is_array()
{
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let mut b: [i32; 5] = [3;5];

    let x: i32 = b[0];

    let &mut y = &mut b[0];
}

fn what_is_statement()
{
    let y: i32 = 
    {
        let x: i32 = 3;
        x + 3
    };

    println!("asahi")
}

fn what_is_if()
{

    let x = if (3 == 3)
    {
        println!("equal");
    }
    else
    {
        1;
    };
}