fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2 // return 생략 가능함
}

fn swap(num1: i32, num2: i32) -> (i32, i32) {
    (num2, num1)
}

fn main() {
    let x: f64 = 1.0;
    let y: i32 = 10;
    
    println!("x = {}, y = {}", x, y);

    let z: f64 = 1.2;
    let y: i32 = z as i32;
    println!("{} -> {}", z, y);

    println!("{}", add(1,2));

    let (n1, n2) = swap(1, 2);
    println!("{n1} {n2}");

    let my_func = |x| x + 1;
    println!("{}", my_func(3))
}

