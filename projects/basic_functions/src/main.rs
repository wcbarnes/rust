fn main() {
    print_number("five", 5);
    print_sum(10, 15);
    print_number("add_one", add_one(10));
    fizzbuzz(15);
    print_number("nth_fib", nth_fib(7));
    print_number("mean", mean(&[1,2,3,4,27]));
    print_number("mode", mode(&[1,2,2,2,2,1,1,1,1,1]));
    println!("mode_mean is {}", mode_mean(&[1,2,3,3,3,3,4,5]));
    println!("mode_mean is {}", mode_mean(&[1,2,3,3,3,3,4,5,1000]));
    draw_stairs(10);
}

fn print_number(y: &str, x: i32) {
    println!("{} is: {}", y, x);
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

fn mode(arr: &[i32]) -> i32 {
    use std::collections::HashMap;
    let mut occurrences = HashMap::new();
    let mut greatest_count: i32 = -2000000000;
    let mut greatest_number: i32 = 0;
    for i in arr {
        let count = occurrences.entry(i.to_string()).or_insert(0);
        *count += 1;
    }
    for (number, count) in &occurrences {
        if *count as i32 > greatest_count {
            greatest_count = *count as i32;
            greatest_number = number.parse().unwrap();
        }
    }
    return greatest_number;
}

fn mode_mean(arr: &[i32]) -> bool {
    if mode(arr) == mean(arr) {
        return true;
    }
    false
}

fn draw_stairs(n: i32) {
    for i in 0..n+1 {
        let mut cur_stair: String = "".to_owned();
        for j in 0..n-i {
            let space: &str = " ";
            cur_stair.push_str(space);
        }
        for j in n-i..n {
            let star: &str = "*";
            cur_stair.push_str(star);
        }
        println!("{}", cur_stair);
    }
}


