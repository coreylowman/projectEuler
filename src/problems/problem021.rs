fn d(n: u64) -> u64 {
    let q = (n as f64).sqrt() as u64 + 1;
    let mut sum = 1;
    for i in 2..q {
        if n % i == 0 {
            sum += i;
            sum += n / i;
        }
    }
    sum
}

fn go() -> String {
    let mut sum = 0;
    for a in 2..10000 {
        let b = d(a);
        let s = d(b);
        if a != b && s == a {
            sum += a;
        }
    }
    sum.to_string()
}

problem!(go, 31626);
