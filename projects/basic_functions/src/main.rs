fn main() {
    print_number(5);
    print_sum(10, 15);
    print_number(add_one(10));
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("Sum is {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}