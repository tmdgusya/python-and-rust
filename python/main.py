"""
fn main() {
    let x: f64 = 1.0;
    let y: i32 = 10;
    
    println!("x = {}, y = {}", x, y);

    let z: f64 = 1.2;
    let y: i32 = z as i32;
    println!("{} -> {}", z, y)
}
"""

def main():
    x = 1.0;
    y = 10;
    
    print(f"x = {x}, y = {y}")
    
    z = 1.2;
    y = int(z)
    
    print(f"z = {z}, y = {y}")


if __name__ == "__main__":
    main()
