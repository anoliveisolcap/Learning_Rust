use std::io;

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

    //array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a o is {}", a[0]);
    let a = [3; 5];
    let mut i = 0;
    while i < 5  {
        println!("a {} is {}", i, a[i]);
        i+=1;
    }
    let a = [0, 1, 2, 3, 4 ,5];
    println!("Please choose an index to print.");
    let mut index =  String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line.");
    let index: usize = index.trim().parse().expect("Index is not a number.");
    let element = a[index];
    println!("The value od the element at index {} is {}", index, element);
}
