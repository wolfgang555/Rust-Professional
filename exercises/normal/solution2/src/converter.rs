pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 分离数字和原进制
    let (num_str, base_str) = num_str.split_once('(').unwrap();
    let from_base = base_str.trim_end_matches(')').parse::<u32>().unwrap();

    // 先将输入转换为十进制值
    let mut decimal = 0u32;
    let mut power = 1;

    // 从右向左处理每一位
    for c in num_str.chars().rev() {
        let digit = c.to_digit(from_base).unwrap();
        decimal += digit * power;
        power *= from_base;
    }

    // 如果十进制为 0，直接返回
    if decimal == 0 {
        return "0".to_string();
    }

    // 将十进制转换为目标进制
    let mut result = Vec::new();
    let mut num = decimal;

    while num > 0 {
        let digit = num % to_base;
        // 将数字转换为对应字符（使用小写字母）
        let c = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'a' + (digit - 10) as u8) as char
        };
        result.push(c);
        num /= to_base;
    }

    // 反转并组合结果
    result.into_iter().rev().collect()
}
