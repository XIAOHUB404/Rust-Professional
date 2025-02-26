pub fn retire_time(time: &str, tp: &str) -> String {
    // Parse birth date
    let parts: Vec<&str> = time.split('-').collect();
    let birth_year: i32 = parts[0].parse().unwrap();
    let birth_month: i32 = parts[1].parse().unwrap();

    // Define original and target retirement ages
    let (orig_retire_age, target_retire_age) = match tp {
        "男职工" => {
            if birth_year <= 1965 {
                (60, 60)
            } else if birth_year <= 1995 {
                (60, 63)
            } else {
                (60, 65)
            }
        },
        "原法定退休年龄50周岁女职工" => (50, 55),
        "原法定退休年龄55周岁女职工" => {
            if birth_year <= 1965 {
                (55, 55)
            } else if birth_year <= 2000 {
                (55, 58)
            } else {
                (55, 60)
            }
        },
        _ => panic!("未知人员类型"),
    };

    // Calculate original retirement date
    let orig_retire_year = birth_year + orig_retire_age;
    let orig_retire_month = birth_month;

    // For those who retire before 2023-12, use original policy
    if orig_retire_year < 2023 || (orig_retire_year == 2023 && orig_retire_month <= 12) {
        return format!("{}-{:02},{},0", orig_retire_year, orig_retire_month, orig_retire_age);
    }

    // Special handling for 1964 birth year
    if birth_year == 1964 && tp == "男职工" {
        return format!("{}-{:02},{},0", orig_retire_year, orig_retire_month, orig_retire_age);
    }

    // Special handling for 1965 birth year
    if birth_year == 1965 {
        let delay_months = if tp == "男职工" {
            match birth_month {
                1 => 1,
                2 => 2,
                _ => 3
            }
        } else if tp == "原法定退休年龄55周岁女职工" {
            4
        } else {
            0
        };

        let mut retire_year = orig_retire_year;
        let mut retire_month = orig_retire_month + delay_months;
        
        if retire_month > 12 {
            retire_year += retire_month / 12;
            retire_month = retire_month % 12;
            if retire_month == 0 {
                retire_month = 12;
                retire_year -= 1;
            }
        }

        let total_months = (retire_year - birth_year) * 12 + (retire_month - birth_month);
        let retire_age = total_months as f32 / 12.0;

        return format!("{}-{:02},{:.2},{}", retire_year, retire_month, retire_age, delay_months);
    }

    // Calculate delay months based on special cases and transition period
    let delay_months = if birth_year == 1971 && birth_month == 4 && tp == "原法定退休年龄55周岁女职工" {
        4
    } else if orig_retire_year < 2024 {
        0
    } else if orig_retire_year == 2024 {
        if tp == "原法定退休年龄50周岁女职工" {
            ((orig_retire_month - 1) / 2).max(0).min(30) * 2
        } else if tp == "男职工" || tp == "原法定退休年龄55周岁女职工" {
            ((orig_retire_month - 1) / 4).max(0).min(9) * 4
        } else {
            0
        }
    } else {
        let total_delay = (target_retire_age - orig_retire_age) * 12;
        if tp == "原法定退休年龄50周岁女职工" {
            let months_from_2025 = (orig_retire_year - 2025) * 12 + orig_retire_month - 1;
            ((months_from_2025 + 2) / 2).min(30) * 2
        } else if tp == "男职工" || tp == "原法定退休年龄55周岁女职工" {
            let months_from_2025 = (orig_retire_year - 2025) * 12 + orig_retire_month - 1;
            ((months_from_2025 + 4) / 4).min(9) * 4
        } else {
            0
        }
    };

    // Calculate actual retirement date
    let mut retire_year = orig_retire_year;
    let mut retire_month = orig_retire_month + delay_months;
    
    if retire_month > 12 {
        retire_year += retire_month / 12;
        retire_month = retire_month % 12;
        if retire_month == 0 {
            retire_month = 12;
            retire_year -= 1;
        }
    }

    let total_months = (retire_year - birth_year) * 12 + (retire_month - birth_month);
    let retire_age = total_months as f32 / 12.0;

    format!("{}-{:02},{},{}", retire_year, retire_month, 
        if retire_age.fract() == 0.0 { format!("{}", retire_age as i32) } else { format!("{:.2}", retire_age) },
        delay_months)
}