use serenity::all::ResolvedValue;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;
use serenity::model::application::ResolvedOption;

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(value),
        ..
    }) = options.first()
    {
        let alert = get_alert(value);
        match alert {
            Ok(value) => {
                format!("Alert: {:#?}", value)
            }
            Err(error) => {
                format!("Error: {:#?}", error)
            }
        }
    } else {
        "Please provide a valid string".to_string()
    }
}

fn get_alert(value: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::blocking::get(value)?.text()?;
    Ok(resp)
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
            .add_int_choice("Sunday", 1)
            .add_int_choice("Monday", 2)
            .add_int_choice("Tuesday", 3)
            .add_int_choice("Wednesday", 4)
            .add_int_choice("Thursday", 5)
            .add_int_choice("Friday", 6)
            .add_int_choice("Saturday", 7)
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
