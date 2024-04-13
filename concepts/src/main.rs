fn main() {
    println!("Hello, world!");
    let mut x: i32 = 5;
    println!("Hey {}", x);
    x = 6;
    println!("Hey {}", x);
    const TITLE: &str = "Hello You, Yes You!";
    println!("{}", TITLE);
    // Tuples
    // let my_tup: (&str, i32) = ("Prime", 20);
    // let (_name: ?, _age: ?) = my_tup;
    // println!("=> Name: {} <-> Age: {}", name, age);
    // let name: &str = my_tup.0;
    // let age: i32 = my_tup.1;
    // println!("{:?} => Name: {} <-> Age: {}", my_tup, name, age);
    let return_value = my_func(55, 6);
    println!("My function returned the value {}", return_value);
}


fn my_func(x: i32, y: i32) -> i32
{
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    let mut sum = if x > y { x - y } else { y - x };
    println!("The sum of x and y is {}", sum);
    sum+=1;
    sum
}