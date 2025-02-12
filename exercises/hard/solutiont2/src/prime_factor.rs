pub fn find_max_prime_factor(mut n: u128) -> u128 {
    if n <= 1 {
        return n;
    }

    let mut max_factor = 0;

    // 处理2因子
    while n & 1 == 0 {
        max_factor = 2;
        n >>= 1;
    }

    // 处理3因子
    while n % 3 == 0 {
        max_factor = 3;
        n /= 3;
    }

    // 处理5因子
    while n % 5 == 0 {
        max_factor = 5;
        n /= 5;
    }

    // 使用跳跃优化，跳过更多的合数
    let mut i = 7;
    let mut jump = 4;

    while i * i <= n {
        while n % i == 0 {
            max_factor = i;
            n /= i;
        }

        i += jump;
        jump = 6 - jump; // 在2和4之间交替，形成 7,11,13,17,19,23...序列

        // 快速检查是否可以提前退出
        if i > 1_000_000 {
            // 对于非常大的数，如果剩余的n是质数，直接返回
            if is_probably_prime(n) {
                return if n > max_factor { n } else { max_factor };
            }
        }
    }

    if n > 1 {
        max_factor = n.max(max_factor);
    }

    max_factor
}

// Miller-Rabin素性测试，用于快速检查大数是否为质数
fn is_probably_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // Miller-Rabin测试的基数
    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    let mut d = n - 1;
    let mut r = 0;

    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    'next_base: for &a in bases.iter() {
        if a >= n {
            break;
        }

        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }

        for _ in 0..r - 1 {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                continue 'next_base;
            }
        }

        return false;
    }

    true
}

// 模幂运算，避免溢出
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    base %= modulus;

    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exp >>= 1;
    }

    result
}

// 模乘法，避免溢出
fn mul_mod(a: u128, b: u128, m: u128) -> u128 {
    let mut result = 0;
    let mut a = a % m;
    let mut b = b;

    while b > 0 {
        if b & 1 == 1 {
            result = (result + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }

    result
}
