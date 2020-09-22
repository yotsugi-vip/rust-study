const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);
}

fn shadowing() {
    let space = "   ";
    // shadowing
    let space = space.len();

    // mutでの型変更はできない
    //let mut space = "   ";
    //space = space.len();
}

fn data_type() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tuptup = (100, "aiu", 8.5);
    let (x, y, z) = tuptup;
    let a = tuptup.0;
    let b = tuptup.1;
    println!("the value y is {}", y);
}
