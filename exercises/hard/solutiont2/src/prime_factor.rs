// prime_factor.rs

/// 使用快速模乘（避免溢出）
fn mod_mul(mut a: u128, mut b: u128, m: u128) -> u128 {
    let mut result = 0;
    a %= m;
    while b > 0 {
        if b & 1 == 1 {
            result = (result + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }
    result
}

/// 快速模幂，计算 base^exp mod m
fn mod_exp(mut base: u128, mut exp: u128, m: u128) -> u128 {
    let mut result = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, m);
        }
        base = mod_mul(base, base, m);
        exp >>= 1;
    }
    result
}

/// 欧几里得算法求最大公约数
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Miller-Rabin 素数测试
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    // 先用小素数测试
    let small_primes: [u128; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &p in small_primes.iter() {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    // 将 n-1 表示为 d * 2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    // 选择一组基，适用于 128 位数的测试
    let bases: [u128; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    'witness: for &a in bases.iter() {
        if a % n == 0 { continue; }
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'witness;
        }
        for _ in 0..(s - 1) {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

/// Pollard-Rho 算法寻找非平凡因子
fn pollard_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    // 初始设定
    let mut x: u128 = 2;
    let mut y: u128 = 2;
    let mut c: u128 = 1;
    let mut d: u128 = 1;
    while d == 1 {
        x = (mod_mul(x, x, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        d = gcd(if x > y { x - y } else { y - x }, n);
        if d == n {
            c += 1; // 更换常数 c 再试
            x = 2;
            y = 2;
            d = 1;
        }
    }
    d
}

/// 递归分解，将 n 的所有素因子收集到 factors 中
fn factorize(n: u128, factors: &mut Vec<u128>) {
    if n == 1 {
        return;
    }
    if is_prime(n) {
        factors.push(n);
    } else {
        let factor = pollard_rho(n);
        factorize(factor, factors);
        factorize(n / factor, factors);
    }
}

/// 找出正整数的最大素数因子
pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut factors = Vec::new();
    factorize(number, &mut factors);
    // 返回所有因子中最大的那个
    *factors.iter().max().unwrap_or(&number)
}
