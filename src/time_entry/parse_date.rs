use chrono::NaiveDate;
use regex::Regex;

fn convert_year(content: &str) -> i32 {
    return if content.len() == 4 {
        content.parse::<i32>().unwrap()
    } else {
        // TODO: config for this?
        content.parse::<i32>().unwrap() + 2000
    };
}

pub fn parse_date(text: &str) -> NaiveDate {
    let extract_date_regex =
        Regex::new(r"((?<first>\d+)[.\-/](?<second>\d+)[.\-/](?<third>\d+))").unwrap();
    let date_data = extract_date_regex.captures(text).unwrap();
    return if text.contains(".") {
        // german standard
        NaiveDate::from_ymd_opt(
            convert_year(date_data.name("third").unwrap().as_str()),
            date_data
                .name("second")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
            date_data
                .name("first")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
        )
    } else {
        NaiveDate::from_ymd_opt(
            convert_year(date_data.name("first").unwrap().as_str()),
            date_data
                .name("second")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
            date_data
                .name("third")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
        )
    }
    .unwrap();
}
