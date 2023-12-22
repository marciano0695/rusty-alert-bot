use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

pub fn register() -> CreateCommand {
    CreateCommand::new("alert_auth")
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
                CommandOptionType::String,
                "username",
                "If neeeded basic auth",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "password",
                "If neeeded basic auth",
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

fn get_alert_with_basic_auth(
    value: &str,
    username: &str,
    password: Option<&str>,
) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(value).basic_auth(username, password).send()?;
    Ok(resp.text()?)
}
