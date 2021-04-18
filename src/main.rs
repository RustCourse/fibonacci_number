fn main() {
    println!("F = {}", fibonacci(200));
}

fn fibonacci(n: u8) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut n_1: u128 = 0;
    let mut n_2 = 1;
    for i in 2..n+1 {
        let (r, overflow) = n_1.overflowing_add(n_2);
        if overflow {
            println!("Computed {} number", i);
            break;
        }
        n_1 = n_2;
        n_2 = r;
    }
    n_2
}
