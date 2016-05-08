fn p(n: u64) -> u64 {
    (3 * n * n - n) / 2
}

fn go() -> String {
    let mut ps = Vec::new();
    ps.push(0);

    for i in 1..5_000 {
        ps.push(p(i));
    }

    for i in 1..ps.len() {
        let d = ps[i];
        for j in 1..ps.len() {
            let pj = ps[j];
            let pk = d + pj;
            if ps.binary_search(&pk).is_ok() && ps.binary_search(&(pk + pj)).is_ok() {
                return d.to_string();
            }
        }
    }
    0.to_string()
}

problem!(go, 5482660);
