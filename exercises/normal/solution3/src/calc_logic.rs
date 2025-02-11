pub fn new_birthday_probability(n: u32) -> f64 {
    let mut diff_probability = 1.0;
    for i in 0..n {
        diff_probability *= (365.0 - i as f64) / 365.0;
    }

    // 返回至少两个人生日相同的概率
    // 即 1 减去所有人生日都不同的概率
    let result = 1.0 - diff_probability;

    // 四舍五入到4位小数
    (result * 10000.0).round() / 10000.0
}
