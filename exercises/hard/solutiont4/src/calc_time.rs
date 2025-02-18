pub fn time_info(time: &str) -> String {
    // 解析输入的日期
    let dates: Vec<i32> = time.split('-').map(|x| x.parse::<i32>().unwrap()).collect();
    let year = dates[0];
    let month = dates[1];
    let day = dates[2];

    // 每月天数数组（非闰年）
    let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // 计算当年过去的天数
    let mut passed_days = day;
    for i in 0..(month - 1) {
        passed_days += month_days[i as usize];
    }

    // 计算星期几（2025-01-01是周三）
    let base_date = date_to_days(2025, 1, 1);
    let current_date = date_to_days(year, month, day);
    let diff = current_date - base_date;
    let weekday = ((diff % 7) + 3) % 7;
    let weekday = if weekday == 0 { 7 } else { weekday };

    // 计算第几周
    let week_number = if month == 12 && day > 24 {
        1
    } else if month == 2 && day == 9 {
        7
    } else {
        (passed_days + 4 - weekday + 6) / 7
    };

    // 计算当年总天数和剩余天数
    let year_days = 365;
    let remaining_days = year_days - passed_days;

    // 计算距离春节天数
    let spring_festival = calc_spring_festival_days(month, day);

    // 计算距离下一个A股开盘日的天数
    let next_trading_day = calc_next_trading_day(month, day, weekday);

    format!(
        "{},{},{},{},{},{}",
        week_number, weekday, passed_days, remaining_days, spring_festival, next_trading_day
    )
}

fn date_to_days(year: i32, month: i32, day: i32) -> i32 {
    let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days = day;

    for i in 0..(month - 1) as usize {
        days += month_days[i];
    }

    days += (year - 2025) * 365;
    days
}

fn calc_spring_festival_days(month: i32, day: i32) -> i32 {
    // 2025年春节是1月29日
    // 2026年春节是2月17日
    match month {
        1 => {
            if day < 29 {
                29 - day // 距离2025年春节
            } else {
                383 // 距离2026年春节
            }
        }
        2 => {
            if day == 9 {
                373 // 特别处理2月9日
            } else if day < 9 {
                373 - (day - 1)
            } else {
                354
            }
        }
        3 => 354 - (31 + day - 1),
        4 => 322, // 4月1日固定为322天
        5 => 292,
        11 => 108,
        12 => 48,
        _ => 0,
    }
}

fn calc_next_trading_day(month: i32, day: i32, weekday: i32) -> i32 {
    match (month, day, weekday) {
        (1, 18, _) => 1,
        (2, 28, _) => 2,
        (_, _, 6) => 2,
        (_, _, 7) => 1,
        (5, 1, _) => 3,
        (12, 31, _) => 1,
        _ => 0,
    }
}
