//const MAX_POINTS: u32 = 100_000;

fn main() {
    _show_po();
}

fn _mutable() {
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);
}

fn _shadowing() {
    let space = "   ";
    // shadowing
    let _space = space.len();

    // mutでの型変更はできない
    //let mut _space = "   ";
    //_space = _space.len();
}

fn _data_type() {
    // tuple
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let tuptup = (100, "aiu", 8.5);
    let (_x, y, _z) = tuptup;
    let _a = tuptup.0;
    let _b = tuptup.1;
    println!("the value y is {}", y);
}

fn _another_function(x: i32, y: i32) {
    println!("the value x is {}", x);
    println!("the value y is {}", y);
}

fn _code_val() {
    let _x = 5;
    let y = {
        let _x = 3;
        // 式の終端は;を含まない...含んだら文
        _x + 1
    };
    println!("the value of y is {}", y);
}

// returnでの指定もあるが、多くの関数は最後の式を暗黙的に返す
fn _five() -> i32 {
    5
}

fn _show_five() {
    let x = _five();
    println!("the value is {}", x);
}

fn _plus_one(x: i32) -> i32 {
    // 下記は文になるためエラー
    // x + 1;
    x + 1
}

fn _show_po(){
    let x = _plus_one(5);
    println!("the value is {}", x);
}
