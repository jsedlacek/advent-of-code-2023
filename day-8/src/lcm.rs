fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn lcm_of_vec(values: &[u64]) -> u64 {
    values.iter().fold(1, |acc, &x| lcm(acc, x))
}
