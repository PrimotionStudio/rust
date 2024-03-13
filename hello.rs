fn main() {
    let fname = "prime";
    let lname = "boss";
    println!("Hello {} {}", fname, lname);

    let result = 10; // i32 by default
    let age:u32 = 20;
    let sum:i32 = 5 - 15;
    let mark:isize = 10;
    let count:usize = 30;

    println!("result is {}", result);
    println!("age is {}", age);
    println!("sum is {}", sum);
    println!("mark is {}", mark);
    println!("count is {}", count);

    println!("=========Integer Overflow=======");
    let age:u8 = 255;
    /*
    let width:u8 = 256;
    let height:u8 = 257;
    let score:u8 = 258;
    */

    println!("Found it");
    println!("age is {}", age);
    /*
    println!("width is {}", width);
    println!("height is {}", height);
    println!("score is {}", score);
    */

    println!("=======floats=========");
    let result = 10.52; // f64 by default
    let interest:f32 = 8.35;
    let cost:f64 = 15000.600;
    println!("result is {}", result);
    println!("interest is {}", interest);
    println!("cost is {}", cost);

    println!("======automatic type casting is not allowed=======");
    /*
    let interest:f32 = 8; // integer is assigned to a float type
    println!("interest is {}", interest);
    println!("integer is assigned to a float type");
    */
}
