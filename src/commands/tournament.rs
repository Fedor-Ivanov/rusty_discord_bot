use crate::tournament_state;

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

pub fn run_start(options: &[ResolvedOption]) -> String {
    println!("Starting {:?}", options);

    if let Some(ResolvedOption {
        value: ResolvedValue::String(data),
        ..
    }) = options.first()
    {
        data.to_string()
    } else {
        "Please provide a valid title".to_string()
    }
}

pub fn register_start() -> CreateCommand {
    CreateCommand::new("start")
        .description("Start a new tournament")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "title",
                "Enter title of the new tournament",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "bo", "Best of...").required(true),
        )
}

pub fn run_info() -> String {
    let status = tournament_state::get_tournament_status();

    match status {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}

pub fn register_info() -> CreateCommand {
    CreateCommand::new("info").description("Get info about current tournament")
}
