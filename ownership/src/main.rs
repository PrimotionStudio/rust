fn main() {
    let mut s: String = String::from("Hello");
    let len: usize = get_str_len(&mut s);
    println!("The length of the string `{}` is {}", s, len);
}

fn get_str_len(my_str: &mut String) -> usize
{
    my_str.push_str(", World!");
    my_str.len()
}