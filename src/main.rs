fn main() {
    println!("F = {}", fibonacci_recursive(187));
    println!("");
    println!("F = {}", fibonacci(187));
}

fn fibonacci(n: u8) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut n_1: u128 = 0;
    let mut n_2 = 1;
    for i in 2..n + 1 {
        let (r, overflow) = n_1.overflowing_add(n_2);
        if overflow {
            println!("Overflow happend, only {} number computed", i - 1);
            break;
        }
        n_1 = n_2;
        n_2 = r;
    }
    n_2
}

fn fibonacci_recursive(num: u8) -> u128 {
    fibonacci_recursion(1, 1, num)
}

fn fibonacci_recursion(n1: u128, n2: u128, count: u8) -> u128 {
    match (n1, n2, count) {
        (1, 1, 1) => 1,
        (1, 1, 2) => 1,
        (1, 1, 0) => 0,
        _ => {
            let cnt = count - 1;
            let (r, overflow) = n1.overflowing_add(n2);
            if overflow {
                println!("{} number(s) left, but overflow happend", cnt - 1);
                n2
            } else if cnt == 2 {
                r
            } else {
                fibonacci_recursion(n2, r, cnt)
            }
        }
    }
}
