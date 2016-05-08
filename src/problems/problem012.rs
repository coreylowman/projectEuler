fn num_divisors(x: u64) -> u64 {
    let lim = ((x as f64).sqrt() as u64) + 1;
    let mut num = 0;
    for i in 1..lim {
        if x % i == 0 {
            num += 2;
        }
    }
    num
}

fn go() -> String {
    let mut n = 1;
    let mut next = 2;
    while num_divisors(n) < 500 {
        n += next;
        next += 1;
    }
    n.to_string()
}

problem!(go, 76576500);
