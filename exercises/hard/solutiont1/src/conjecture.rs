pub fn goldbach_conjecture() -> String {
    fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let sqrt = (n as f64).sqrt() as u64;
        (3..=sqrt).step_by(2).all(|i| n % i != 0)
    }

    fn can_be_represented(n: u64) -> bool {
        let max_square = ((n / 2) as f64).sqrt() as u64;
        (0..=max_square).any(|i| {
            let twice_square = 2 * i * i;
            if twice_square >= n {
                return false;
            }
            is_prime(n - twice_square)
        })
    }

    let mut found = Vec::with_capacity(2);
    let mut n: u64 = 33; // 从一个较小的数开始

    while found.len() < 2 {
        if !can_be_represented(n) {
            found.push(n);
        }
        n += 2;
    }

    format!("{},{}", found[0], found[1])
}
