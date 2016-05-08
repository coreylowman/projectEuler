fn go() -> String {
    let factors = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut n = 1;
    for i in 0..factors.len() {
        n *= factors[i];
    }
    for i in 2..21 {
        if n % i != 0 {
            n *= n % i;
        }
    }

    (n / 2).to_string()
}

problem!(go, 232792560);
