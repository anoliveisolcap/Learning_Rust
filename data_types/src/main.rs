fn main() {
    //integer
    let x = -5;//i32
    let x = x - 1;
    let x = x * 2;
    let x = x - 2147483636;
    println!("The value of x is: {}", x);

    //float
    let x = 2.1; // f64
    let y: f32 = 3.1; // f32
    println!("x={} y={}", x, y);

    //char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup 0 is {}", tup.0);
    let (x, y, z) = tup;
    println!("tup is {} {} {}", x, y, z);
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("tup is {} {} {}", x, y, z);

}
