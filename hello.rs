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
    println!("=====number seperator====");
    let num1:i32 = 2_00_0_00; // the order of underscore doesnt matter
    let num2:f32 = 100_000.345_002;
    println!("num1 is {}", num1);
    println!("num2 is {}", num2);
    println!("=====constant=====");
    const PI:f32 = 3.14;
    println!("pi is {}", PI);
    println!("=======shadowing variables======");
    let _num1 = 1000;
    let num1 = 500;
    print!("{}", num1);
    println!("Without using `println`");
    let uname = "Prime";
    let uname = uname.len();
    println!("name is of length: {}", uname);
    let uname:&str = "Prime";
    let uname:usize = uname.len();
    println!("name is of length: {}", uname);
    println!("======STATIC======");
    let company_name = "The Primotion Studio"; // By default, all string
    let persona:&str = "Primotion Studio";     // literals are of static types
    let person:&'static str = "Prime";         // but us can explicitly specify that it should be
                                               // static, by using `&'static str`
    println!("I own a company called `{}`, and i take on the `{}` persona.", company_name, persona);
    println!("My name is `{}`", person);

    println!("=======String Object=======");
    let emptystr = String::new();
    println!("empty string is `{}` -> len: {}", emptystr, emptystr.len());
    let contentstr = String::from("Primotion STudio");
    println!("content string is `{}` -> len: {}", contentstr, contentstr.len());
}
