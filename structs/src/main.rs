use std::process::exit;

struct Square {
    size: u64,
    // area: u128,
    // perimeter: u64,
}
struct Rectangle {
    height: u64,
    width: u64,
    // area: u128,
    // perimeter: u64,
}
impl Rectangle {
    fn area(&self) -> u128 {
        (self.height * self.width).into()
    }
    fn perimeter(&self) -> u64 {
        self.height + self.width
    }
    fn square(&self, height: u64, width: u64) -> Square {
        if height == width {
            return Square { size: height };
        } else {
            exit(-1);
        }
    }
}
impl Square {
    fn area(&self) -> u128 {
        (self.size^2).into()
    }
    fn perimeter(&self) -> u64 {
        self.size * 2
    }
}

fn main() {
    let my_rectangle = Rectangle {
        height: 50,
        width: 50,
        // area: {
        //     &self.height * &self.width
        // },
        // perimeter: {
        //     &self.height * &self.width
        // }
    };
    println!("My Rectangle::height : {}", my_rectangle.height);
    println!("My Rectangle::width : {}", my_rectangle.width);
    println!("My Rectangle::area : {}", my_rectangle.area());
    println!("My Rectangle::perimeter : {}", my_rectangle.perimeter());
    let my_square = my_rectangle.square(my_rectangle.height, my_rectangle.width);
    println!("My Square::size : {}", my_square.size);
    println!("My Square::area : {}", my_square.area());
    println!("My Square::perimeter : {}", my_square.perimeter());
}
