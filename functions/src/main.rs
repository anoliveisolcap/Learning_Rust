fn main() {
    println!("Hello, world!");
    let sum: i32 = another_function(5, 8);
    println!("The sum is: {}", sum);
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    //let sum: i32 = x + y;
    //return sum;
    x + y
}
