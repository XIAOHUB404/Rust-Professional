pub fn convert_base(input: &str, target_base: u32) -> String {
    // 解析输入字符串，格式为 "number(base)"
    let parts: Vec<&str> = input.trim_end_matches(')').split('(').collect();
    let number = parts[0];
    let source_base = parts[1].parse::<u32>().unwrap();

    // 首先将输入数字转换为十进制
    let decimal = i64::from_str_radix(number, source_base).unwrap();

    // 如果目标进制是 10，直接返回十进制字符串
    if target_base == 10 {
        return decimal.to_string();
    }

    // 将十进制转换为目标进制
    let mut result = String::new();
    let mut n = decimal;

    while n > 0 {
        let digit = n % target_base as i64;
        let digit_char = match digit {
            0..=9 => (b'0' + digit as u8) as char,
            10..=15 => (b'a' + (digit - 10) as u8) as char,
            _ => unreachable!(),
        };
        result.insert(0, digit_char);
        n /= target_base as i64;
    }

    // 处理输入为 0 的情况
    if result.is_empty() {
        result.push('0');
    }

    result
}
