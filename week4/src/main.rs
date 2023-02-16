use serde::Deserialize;

fn get_holidays_for_country_and_year(country_code: &str, year: i32) -> Vec<Holiday> {
    let url = format!(
        "https://date.nager.at/api/v3/publicholidays/{}/{}",
        year, country_code
    );

    reqwest::blocking::get(url).unwrap().json().unwrap()
}

fn main() {
    let country_codes = vec!["SE", "NO", "IT", "CA"];

    for code in country_codes {
        let holidays = get_holidays_for_country_and_year(code, 2023);
        println!(
            "country = {:#?}, holidays_number = {:#?}",
            code,
            holidays.len(),
        );
        for h in holidays {
            println!("{}, {}, {}", h.name, h.date, h.localName);
        }
    }
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Holiday {
    date: String,
    name: String,
    localName: String,
}
