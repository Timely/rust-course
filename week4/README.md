# Week 4 - APIs, collections, traits, JSON

In this exercise, we will be making HTTP calls for JSON data, parse it to structs and extract some statistics from it.

## 1. HTTP request

We'll use `reqwest` to call the [date.nager.at Public Holiday API](https://date.nager.at/Api). This API returns a list of public holidays for a given country and year.
Add the following line to your `Cargo.toml`:

```toml
reqwest = { version = "0.11", features = ["blocking", "json"] }
```

Then, implement the `get_holidays_for_country_and_year` function in `src/main.rs` to call the API using `reqwest::blocking::get(...).unwrap().text().unwrap()`.
Print the result in `main` to see what it looks like.
[Docs](https://docs.rs/reqwest/latest/reqwest/blocking/index.html)

## 2. JSON parsing

The response is a JSON string.
Rust does not have runtime reflection, so how does it know how to parse a JSON string into a struct?
Serde is a framework for serializing and deserializing Rust data structures efficiently and generically. It allows us to annotate a struct (or enum) with `#[derive(Deserialize)]` to automatically generate the code to parse a JSON string (or CSV, or XML etc) into the struct.

Add `serde` to your `Cargo.toml`:

```
cargo add serde --features derive
```

Then, create a struct named `Holiday`. We are only interested in the `date`, `name` and `localName` fields, so let's only add those.

Next, add the `#[derive(Deserialize)]` attribute to the struct.

Now, we can change the `get_holidays_for_country_and_year` function to return a `Vec<Holiday>` instead of a `String`, and switch from .text() to .json() to parse the JSON string into a `Vec<Holiday>` (reqwest uses `serde_json` under the hood).

## 3. Printing stopped working!

We can't print the result anymore, because we have not defined what it is we want to happen when we print a `Vec<Holiday>`.
Since we do not want to define a text format for Holiday, and only print for debugging purposes, we can use the `Debug` trait.
Add `#[derive(Debug)]` (or add Debug to the existing derive line: `#[derive(Deserialize, Debug)]`) to the `Holiday` struct, and print the result again, but with `{:?}` instead of `{}`.

Try running it. If you want to see the result in a more readable format, you can use the pretty print format character #: `{:#?}`.

## 4. Statistics

Now, let's do some statistics on the data.
We want to run this function on a list of country codes.
Create a vec of country codes (e.g. `["SE", "NO", "IT", "CA"]` (I'm sorry to report "IN" is not included in the dataset. Oh well.)), and call the function for each country code.

Print the number of holidays for each country code.

## 5. Bonus

Print the number of holidays per quarter for each country code.

## 6. Bonus

Find dates which are holidays in the most datasets. Print every date that are holidays in at least 2 countries.
