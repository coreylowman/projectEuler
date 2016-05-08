fn choose(n: u64, r: u64) -> u64 {
    if r == 0 || r == 1 {
        return 1u64;
    }
    let mut prod = 1u64;
    for i in 1..r + 1 {
        prod *= n + 1 - i;
    }
    for i in 1..r + 1 {
        prod /= i;
    }
    prod
}

fn go() -> String {
    let mut num = 0;
    for n in 1..101 {
        for r in 0..n / 2 {
            if choose(n, r) >= 1_000_000 {
                num += n - 2 * r + 1;
                break;
            }
        }
    }
    num.to_string()
}

problem!(go, 4075);
