// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // let mut x =5;
    // println!("The value of x is: {x}");
    // x=6;
    // println!("The value of x is: {x}");
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    let spaces = "   ";
    let spaces = spaces.len();
    print!("The length of spaces is: {spaces}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let flag: bool = true;
    let (x, y, z) = tup;
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
}
