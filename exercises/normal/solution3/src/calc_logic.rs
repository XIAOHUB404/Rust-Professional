pub fn new_birthday_probability(n: u32) -> f64 {
    let days = 365.0;
    let mut not_same = 1.0;
    
    // 计算所有人生日都不相同的概率
    for i in 0..n {
        not_same *= (days - i as f64) / days;
    }
    
    // 计算至少两个人生日相同的概率，并保留4位小数
    ((1.0 - not_same) * 10000.0).round() / 10000.0
}
