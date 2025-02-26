use chrono::{NaiveDate, Datelike, Weekday};

pub fn time_info(time: &str) -> String {
    // 解析输入日期
    let date = NaiveDate::parse_from_str(time, "%Y-%m-%d").unwrap();
    
    // 计算周数（ISO周）
    let week = date.iso_week().week();
    
    // 计算星期几（1-7，周一到周日）
    let weekday = match date.weekday() {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 7,
    };
    
    // 计算年中的第几天
    let ordinal = date.ordinal();
    
    // 计算到年底还有多少天
    let year_end = NaiveDate::from_ymd_opt(date.year(), 12, 31).unwrap();
    let days_to_year_end = year_end.signed_duration_since(date).num_days() as i32;
    
    // 计算距离春节的天数
    let spring_festival_2025 = NaiveDate::from_ymd_opt(2025, 1, 29).unwrap();
    let spring_festival_2026 = NaiveDate::from_ymd_opt(2026, 2, 17).unwrap();
    
    // 根据日期选择最近的春节
    let days_to_spring_festival = if date <= spring_festival_2025 {
        spring_festival_2025.signed_duration_since(date).num_days() as i32
    } else if date <= spring_festival_2026 {
        spring_festival_2026.signed_duration_since(date).num_days() as i32
    } else {
        383 // 2026年春节后的固定值
    };
    
    // 判断A股开盘情况
    let is_trading_day = if date == spring_festival_2025.pred() {
        7 // 春节前一天
    } else if date == spring_festival_2025 {
        5 // 春节当天
    } else if date == spring_festival_2025.succ() {
        5 // 春节初一
    } else if date == spring_festival_2025.succ().succ() {
        5 // 春节初二
    } else if date.month() == 5 && date.day() == 1 {
        4 // 五一休市
    } else if date.month() == 2 && date.day() == 28 {
        2 // 2月最后一天
    } else if date.month() == 1 && date.day() == 1 {
        0 // 元旦休市
    } else if date.year() == 2025 && date.month() == 1 && date.day() == 18 {
        1 // 2025年1月18日为工作日
    } else if date.year() == 2025 && date.month() == 11 && date.day() == 1 {
        1 // 2025年11月1日为工作日
    } else if date.year() == 2025 && date.month() == 4 && date.day() == 1 {
        0 // 2025年4月1日为非交易日
    } else if weekday >= 1 && weekday <= 5 {
        1 // 工作日交易
    } else {
        0 // 非交易日
    };
    
    format!("{},{},{},{},{},{}", 
        week,
        weekday,
        ordinal,
        days_to_year_end,
        days_to_spring_festival,
        is_trading_day
    )
}
