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

    println!("=====CONDITIONAL=======");
    let age = 0;
    if age >= 18 {      //In rust, parentesis around the conditions is not necessary, the same is
                        //for elseif
        println!("You're an adult");
    } else {
        println!("You're stil a child");
    }
    println!("=======MATCH EXPR======");
    println!("There are 2 ways to do this");
    println!("way 1 =>");
    let ntime = "Afternoon";

    let daytime = match ntime {
        "Morning"   => "00:00 -> 11:59",
        "Afternoon" => {
            println!("Match Found");
            "12:00 -> 15:59"
        },
        "Evening"   => "16:00 -> 23:59",
        &_         => "Unknown day time format"     // now `&_` is used as a wildcard to match
                                                    // anything that wasnt found
    };

    println!("The time of the day is {} and it ranges from {}", ntime, daytime);

    println!("way 2 =>");
    let time = "Morning";

    match time {
        "Morning"   => {
            println!("Match Found");
            println!("00:00 -> 11:59");
        },
        "Afternoon" => { println!("12:00 -> 15:59"); },
        "Evening"   => { println!("16:00 -> 23:59"); },
        &_          => { println!("Unknown day time format"); }     // the difference btw this one
                                                                    // that last is that the last
                                                                    // one was storing a value
                                                                    // into a variable if it
                                                                    // matched a certain condition
                                                                    // while this one is just
                                                                    // executing a funtion if it
                                                                    // matches.
                                                                    // NOTE: you could also execute
                                                                    // a function and still store a
                                                                    // value as shown in the first
                                                                    // example
    };

}
