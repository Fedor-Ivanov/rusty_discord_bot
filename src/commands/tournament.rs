use crate::tournament_state;

use serenity::all::User;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

pub fn run_start(options: &[ResolvedOption]) -> String {
    let title = &options[0];
    let bo = &options[1];

    if let (
        ResolvedOption {
            value: ResolvedValue::String(title_data),
            ..
        },
        ResolvedOption {
            value: ResolvedValue::Number(bo_data),
            ..
        },
    ) = (title, bo)
    {
        let title_string: String = title_data.to_string();
        let bo_qwe: u8 = bo_data.clone() as u8;

        tournament_state::init_tournament(title_string, bo_qwe);
        format!(
            "We are announcing a new tournament - {}! Tournament format: best of {}. Registration in open!",
            title_data, bo_data
        )
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
            CreateCommandOption::new(CommandOptionType::Number, "bo", "Best of...")
                .required(true)
                .add_int_choice("1", 1)
                .add_int_choice("3", 3)
                .add_int_choice("5", 5),
        )
}

pub fn run_info() -> String {
    let status = tournament_state::get_tournament_status();
    let tournament_title: String = tournament_state::get_tournament_title();

    match status {
        Ok(res) => format!("Tournament '{}' is {}", tournament_title, res),
        Err(err) => err.to_string(),
    }
}

pub fn register_info() -> CreateCommand {
    CreateCommand::new("info").description("Get info about current tournament")
}

pub fn run_join(user: &User) -> String {
    // here we need to save user to the tournament registration, in field 'participants'

    format!(
        "{:?} are joined a tournament",
        user.global_name.clone().unwrap()
    )
}

pub fn register_join() -> CreateCommand {
    CreateCommand::new("join").description("Join a tournament")
}
