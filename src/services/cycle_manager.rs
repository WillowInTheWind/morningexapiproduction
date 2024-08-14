use chrono::{DateTime, Datelike, Days, NaiveDate, NaiveDateTime, Utc, Weekday};

pub static OFF_DAYS: [[&str; 2]; 12]= [["September 2 2024", "September 2 2024"],
["10-11-2024", "10-11-2024"],
[ "10-14-2024", "10-14-2024"],
[ "11-27-2024", "11-29-2024"],
[ "12-20-2024", "1-3-2025"],
[ "1-6-2025", "1-6-2025"],
[ "1-17-2025", "1-17-2025"],
[ "1-20-2025", "1-21-2025"],
[ "2-17-2025", "2-21-2025"],
[ "4-7-2025", "4-11-2025"],
[ "4-14-2025", "4-14-2025"],
[ "5-26-2025", "5-26-2025"]];

pub fn day_to_cycle(day: NaiveDateTime ) -> i32 {
    let parse = NaiveDateTime::parse_from_str;
    let mut start: NaiveDateTime;
    if (NaiveDate::from_ymd_opt(2025, 1, 22).unwrap().and_hms_opt(23,59,59).unwrap() < day) {
        start = parse("1/22/25 23:59:00", "%Y/%m/%d %H:%M:%S").unwrap();
    } else {
        start = parse("9/4/2024 23:59:00", "%Y/%m/%d %H:%M:%S").unwrap();
    }
    let off_days_parsed = OFF_DAYS.map(|e|  {
        let start_date = parse(&(e[0].to_owned() + " 00:00:00"), "%Y-%m-%d %H:%M:%S").unwrap();
        let end_date =  parse(&(e[1].to_owned() + " 23:59:59"), "%Y-%m-%d %H:%M:%S").unwrap();
        if (start_date.month() > 7) {
            start_date.with_year(2024);
        } else {
            start_date.with_year(2025);
        }
        if (end_date.month() > 7) {
            end_date.with_year(2024);
        } else {
            end_date.with_year(2025);
        }
         [start_date, end_date]
    }
    );

    let mut days = 0;
    while (start.and_utc().timestamp_millis() < day.and_utc().timestamp_millis()) {
    let weekday = start.weekday();
    if weekday == Weekday::Sat || weekday == Weekday::Sun || off_days_parsed.iter().any(|e| { e[0].and_utc().timestamp_millis() <= start.and_utc().timestamp_millis() && e[1].and_utc().timestamp_millis() >= start.and_utc().timestamp_millis() }) {
        start =start + Days::new(1);
        continue
    }
        start =start + Days::new(1);
        days += 1;
    }
     days % 8
}
