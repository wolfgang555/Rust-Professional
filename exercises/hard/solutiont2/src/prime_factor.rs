pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut n = number;
    if n <= 1 {
            return n;
        }

        let mut max_prime = 1;

        // 处理2
        if n & 1 == 0 {
            max_prime = 2;
            n >>= 1;
            while n & 1 == 0 {
                n >>= 1;
            }
        }

        // 主要搜索循环
        let mut i = 3;
        while i <= n / i {  // 避免sqrt操作，使用除法
            if n % i == 0 {
                max_prime = i;
                n /= i;
                while n % i == 0 {
                    n /= i;
                }
            }
            i += 2;
        }

        if n > 1 {
            max_prime = n;
        }

        max_prime
}
