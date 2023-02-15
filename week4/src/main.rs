use reqwest::Body;
use serde::Deserialize;

fn main() {
    let countries = vec!["SE", "NO", "IT", "CA"];

    for country in countries {
        let holidays = get_holidays_for_country_and_year(country, 2023);
        println!("Length {}", holidays.len())
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
    date: String,
    name: String,
    localName: String,
}
