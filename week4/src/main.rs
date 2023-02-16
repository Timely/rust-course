use chrono::{Datelike, NaiveDate};
use serde::Deserialize;
use std::collections::HashMap;

fn main() {
    let countries = vec!["SE", "NO", "IT", "CA"];
    let mut holidays_for_country = HashMap::new();
    for country in countries {
        let holidays = get_holidays_for_country_and_year(country, 2023);
        holidays_for_country.insert(country, holidays);
    }

    for entry in &holidays_for_country {
        print!("Country code: {} ", entry.0);
        let holidays = entry.1;
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

    let dates: Vec<NaiveDate> = holidays_for_country
        .into_values()
        .flatten()
        .map(|h| h.date)
        .collect();

    let mut all_holy = HashMap::new();
    for date in dates {
        let count = all_holy.entry(date).or_insert(0);
        *count += 1
    }
    let at_least_twice: Vec<(&NaiveDate, &i32)> = all_holy.iter().filter(|e| e.1 >= &2).collect();
    println!("Days that are holiday in at least 2 countries: ");
    for i in at_least_twice {
        print!("{} is holiday {} times | ", i.0, i.1);
    }
    println!();
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
