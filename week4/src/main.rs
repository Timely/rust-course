use chrono::{Datelike, NaiveDate};
use serde::Deserialize;

fn main() {
    let countries = vec!["SE", "NO", "IT", "CA"];

    for country in countries {
        print!("Country code: {} ", country);
        let holidays = get_holidays_for_country_and_year(country, 2023);
        print!("Number of holidays: {} - ", holidays.len());

        for q in 1..=4 {
            let quarter: Vec<u32> = holidays
                .iter()
                .map(|h| h.date.month())
                .filter(|m| {
                    let min_bound = (q - 1) * 3;
                    let max_bound = min_bound + 3;
                    if m > &min_bound && m <= &max_bound {
                        true
                    } else {
                        false
                    }
                })
                .collect();
            print!("Q{}: {}; ", q, quarter.len());
        }
        println!()
    }
}

fn get_holidays_for_country_and_year(country_code: &str, year: i32) -> Vec<Holiday> {
    reqwest::blocking::get(format!(
        "https://date.nager.at/api/v3/publicholidays/{}/{}",
        year, country_code
    ))
    .unwrap()
    .json()
    .unwrap()
}

#[derive(Debug, Deserialize)]
struct Holiday {
    date: NaiveDate,
    name: String,
    localName: String,
}
