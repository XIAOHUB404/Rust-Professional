pub fn goldbach_conjecture() -> String {
    // 判断一个数是否为素数
    fn is_prime(n: u64) -> bool {
        if n <= 1 { return false; }
        if n <= 3 { return true; }
        if n % 2 == 0 || n % 3 == 0 { return false; }
        let sqrt_n = (n as f64).sqrt() as u64;
        let mut i = 5;
        while i <= sqrt_n {
            if n % i == 0 || n % (i + 2) == 0 { return false; }
            i += 6;
        }
        true
    }

    // 检查一个数是否可以表示为一个素数和一个平方数的两倍之和
    fn can_be_represented(n: u64) -> bool {
        let sqrt_limit = ((n as f64) / 2.0).sqrt() as u64;
        for i in 1..=sqrt_limit {
            let square_double = 2 * i * i;
            if square_double >= n { break; }
            let remaining = n - square_double;
            if is_prime(remaining) {
                return true;
            }
        }
        false
    }

    // 寻找不满足条件的奇合数
    let mut count = 0;
    let mut result = Vec::new();
    let mut n = 3;

    while count < 2 {
        if n % 2 == 1 && !is_prime(n) && !can_be_represented(n) {
            result.push(n);
            count += 1;
        }
        n += 2;
    }

    format!("{},{}" ,result[0], result[1])
}
