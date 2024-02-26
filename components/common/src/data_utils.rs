use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use chrono::prelude::*;
use log::error;

pub fn to_day_str() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn to_naive_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
}

pub fn to_previous_day(date_str: &str) -> Option<NaiveDate> {
    match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(date) => date.pred_opt(),
        Err(err) => {
            error!("parse date error: {}", err);
            Local::now().date_naive().pred_opt()
        }
    }
}

pub fn diff_days(before: &NaiveDate, after: &NaiveDate) -> i64 {
    after.signed_duration_since(*before).num_days()
}

#[test]
fn compare_date() {
    let date_str = "2024-02-21";
    let date = to_naive_date(date_str).unwrap();
    let today = Local::now().date_naive();
    println!("days: {}", diff_days(&date, &today));
    // assert_eq!(diff_days(&today, &date), 3);
}

#[test]
fn main2() {
    // 假设我们有一个日期字符串
    let date_str = "2024-02-22"; // 日期格式为 YYYY-MM-DD
    // 解析日期字符串为 NaiveDate 类型
    match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(parsed_date) => {
            // 获取当前日期
            let current_date = Local::now().date_naive();

            // 比较日期
            if parsed_date > current_date {
                println!("给定的日期 '{}' 在当前日期之后。", date_str);
            } else if parsed_date < current_date {
                println!("给定的日期 '{}' 在当前日期之前。", date_str);
            } else {
                println!("给定的日期 '{}' 是当前日期。", date_str);
            }
        }
        Err(e) => {
            // 解析失败的错误处理
            println!("日期解析错误: {}", e);
        }
    }
}

#[test]
fn main1() {
    // 获取当前的UTC时间
    let utc_now: DateTime<Utc> = Utc::now();
    println!("当前的UTC时间：{}", utc_now.format("%Y-%m-%d %H:%M:%S"));

    // 转换为当地时间
    let local_now: DateTime<Local> = Local::now();
    println!("当前的当地时间：{}", local_now.format("%Y-%m-%d %H:%M:%S"));

    // 使用自定义格式来格式化日期和时间
    let custom_format = local_now.format("%Y年%m月%d日 %H时%M分%S秒");
    println!("格式化后的日期和时间：{}", custom_format);

    // 从字符串解析NaiveDateTime
    match NaiveDateTime::parse_from_str("2024-02-22 08:13:07", "%Y-%m-%d %H:%M:%S") {
        Ok(naive_datetime) => {
            // 如果你知道时区，可以转换为具体的TimeZone，比如使用UTC或本地时区（Local）
            let utc_datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive_datetime);
            println!("转换为UTC DateTime：{}", utc_datetime);

            let local_datetime: DateTime<Local> = Local.from_utc_datetime(&naive_datetime);
            println!("转换为本地DateTime：{}", local_datetime);
        }
        Err(e) => println!("解析错误：{}", e),
    }
}