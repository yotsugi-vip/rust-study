//const MAX_POINTS: u32 = 100_000;

fn main() {
    _twelve_days_of_christmas();
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

fn _show_po() {
    let x = _plus_one(5);
    println!("the value is {}", x);
}

fn _variable_if() {
    let state = true;
    let a = if state { 10 } else { 15 };
    println!("the value is {}", a);
}

fn _collection_for() {
    let a = [10, 20, 30, 40, 50];
    for el in a.iter() {
        println!("the value is {}", el);
    }
}

fn _convert_c_to_f(c: f64) -> f64 {
    let ret = c * 1.8 + 32.0;
    return ret;
}

fn _fibo(index: u128) -> u128 {
    let mut f0: u128 = 0;
    let mut f1: u128 = 1;
    let mut _buff: u128 = 0;
    let mut i = 0;

    while i != index {
        _buff = f1;
        f1 = f0 + f1;
        f0 = _buff;
        i = i + 1;
        println!("f0:{} f1:{} i:{}", f0, f1, i);
    }
    return f0;
}

fn _twelve_days_of_christmas() {
    let mut val = [
        "and a partridge in a pear tree.",
        "two turtle doves,",
        "three French hens,",
        "four calling birds,",
        "five golden rings.",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    val.reverse();
    
    println!("12.");
    println!("On the twelfth day of Christmas,my true love sent to me");
        
    for el in val.iter() {
        println!("{}", el);
    }
}
