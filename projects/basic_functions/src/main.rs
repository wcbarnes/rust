fn main() {
    print_number(5);
    print_sum(10, 15);
    print_number(add_one(10));
    fizzbuzz(15);
    print_number(nth_fib(7));
    let arr: [i32; 5] = [1,2,3,4,5];
    print_number(mean(&arr));
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

fn fizzbuzz(x: i32) {
    for y in 1..x+1 {
        if y % 15 == 0 {
            println!("{}. FIZZBUZZ", y);
        } else if y % 5 == 0 {
            println!("{}. BUZZ", y);
        } else if y % 3 == 0 {
            println!("{}. FIZZ", y);
        } else {
            println!("{}", y);
        }
    }
}

fn nth_fib(num: i32) -> i32 {
    if num < 2 {
        return num;
    }
    nth_fib(num - 1) + nth_fib(num - 2)
}

fn mean(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for i in arr {
        sum = sum + i;
    }
    return sum / arr.len() as i32;
}


