pub mod utils;

use lazy_static::lazy_static;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum TournamentStatus {
    Annonce,
    Started,
    Finished,
    InProgress,
    Closed,
}
#[derive(Debug, Clone)]
pub enum RegistrationStatus {
    Open,
    Closed,
}
#[derive(Debug, Clone)]
pub enum MatchFormat {
    BO1,
    BO3,
    BO5,
}
#[derive(Clone)]
pub struct Tournament {
    status: TournamentStatus,
    title: String,
    match_format: MatchFormat,
    teams: Vec<Team>,
    registrations: Registration,
}

#[derive(Clone)]

pub struct Player {
    id: String,
    name: String,
}
#[derive(Clone)]

pub struct TeamMember {
    player: Player,
    team_id: String,
}
#[derive(Clone)]

pub struct Team {
    id: String,
    members: Vec<TeamMember>,
}
#[derive(Clone)]

pub struct Registration {
    status: RegistrationStatus,
    participants: Vec<Player>,
}

lazy_static! {
    static ref TOURNAMENT: Arc<Mutex<Tournament>> = Arc::new(Mutex::new(Tournament {
        status: TournamentStatus::Closed,
        title: String::from("Default Tournament"),
        match_format: MatchFormat::BO1,
        teams: Vec::new(),
        registrations: Registration {
            status: RegistrationStatus::Closed,
            participants: Vec::new(),
        },
    }));
}

pub fn init_tournament(title: String, format_number: u8) -> Arc<Mutex<Tournament>> {
    let mut tournament = TOURNAMENT.lock().unwrap();
    tournament.status = TournamentStatus::Annonce;
    tournament.title = title;
    tournament.registrations.status = RegistrationStatus::Open;
    tournament.match_format = utils::parse_tournament_format(format_number);
    TOURNAMENT.clone()
}

pub fn get_tournament() -> Result<Tournament, Box<dyn Error>> {
    let current_tournament_status = get_tournament_status();

    let tournament_guard = TOURNAMENT.lock().unwrap();

    match current_tournament_status {
        Err(e) => Err(e.into()),
        _ => Ok(tournament_guard.clone()),
    }
}

pub fn get_tournament_status() -> Result<String, Box<dyn Error>> {
    let tournament = TOURNAMENT.lock().unwrap();
    match tournament.status {
        TournamentStatus::Annonce => Ok(String::from("is announsed")),
        TournamentStatus::Started => Ok(String::from("is about to start")),
        TournamentStatus::InProgress => Ok(String::from("in progress")),
        TournamentStatus::Finished => Ok(String::from("is finished")),
        TournamentStatus::Closed => Err("No Tournament at this time".into()),
    }
}

pub fn get_tournament_title() -> String {
    let tournament = TOURNAMENT.lock().unwrap();
    tournament.title.clone()
}

pub fn reset_tournament() {
    let mut tournament = TOURNAMENT.lock().unwrap(); // Получаем доступ к Mutex
    *tournament = Tournament {
        status: TournamentStatus::Closed,
        title: String::from("Default Tournament"),
        match_format: MatchFormat::BO1,
        teams: Vec::new(),
        registrations: Registration {
            status: RegistrationStatus::Closed,
            participants: Vec::new(),
        },
    };
}
