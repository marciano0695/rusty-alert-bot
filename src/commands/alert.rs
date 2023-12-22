use chrono::Utc;
use schedule::{Agenda, Job};
use serde_json::Value;
use serenity::all::ResolvedValue;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;
use serenity::model::application::ResolvedOption;
use std::thread;

pub fn run(options: &[ResolvedOption]) -> String {
    // let mut week_day: &i64 = &1;
    // let mut hours: &i64 = &12;
    let mut url: &str = "";

    // TODO: Create te schedule with values from resolve option
    /*
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(week_day_option),
        ..
    }) = options.get(1)
    {
        week_day = week_day_option;
    }

    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(hours_option),
        ..
    }) = options.get(1)
    {
        hours = hours_option;
    }
     */

    if let Some(ResolvedOption {
        value: ResolvedValue::String(url_option),
        ..
    }) = options.first()
    {
        url = url_option;
    }
    /*
    thread::spawn(|| {
        let mut a = Agenda::new();

        // Run every second
        a.add(Job::new(
            || {
                println!("at second     :: {}", Utc::now());
            },
            "0 0 * * * *".parse().unwrap(),
        ));

        loop {
            a.run_pending();

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });
    */

    let alert = get_alert(url);
    match alert {
        Ok(value) => {
            let v: Value = serde_json::from_str(&value).unwrap();
            format!("```json\n{:#?}\n```", v)
        }
        Err(error) => {
            format!("Error: {:#?}", error)
        }
    }
}

fn get_alert(value: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(value).send()?;
    Ok(resp.text()?)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("alert")
        .description("An alert command to warn frequently weekly and display the message")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "url",
                "Specify url to check frequently",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "day",
                "Select which day of the week to frequently check",
            )
            .add_int_choice("Sunday", 0)
            .add_int_choice("Monday", 1)
            .add_int_choice("Tuesday", 2)
            .add_int_choice("Wednesday", 3)
            .add_int_choice("Thursday", 4)
            .add_int_choice("Friday", 5)
            .add_int_choice("Saturday", 6)
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "hours", "24 h")
                .min_int_value(0)
                .max_int_value(24)
                .max_length(2)
                .required(true),
        )
}

#[cfg(test)]
mod tests {
    use crate::commands::alert::get_alert;

    #[test]
    fn it_get_alert() {
        let url: &str = "https://dummyjson.com/products/1";
        let result = get_alert(url);
        match result {
            Ok(value) => {
                println!("Alert: {:#?}", value);
            }
            Err(error) => {
                println!("Error: {:#?}", error)
            }
        }
    }
}
