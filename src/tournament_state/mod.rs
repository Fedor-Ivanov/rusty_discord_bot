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

struct Tournament {
    status: TournamentStatus,
    title: String,
    match_format: MatchFormat,
    teams: Vec<Team>,
    registrations: Registration,
}

pub struct Player {
    id: String,
    name: String,
}

pub struct TeamMember {
    player: Player,
    team_id: String,
}

pub struct Team {
    id: String,
    members: Vec<TeamMember>,
}

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

pub fn init_tournament(title: String) -> Arc<Mutex<Tournament>> {
    let mut tournament = TOURNAMENT.lock().unwrap();
    tournament.status = TournamentStatus::Annonce;
    tournament.title = title;
    tournament.registrations.status = RegistrationStatus::Open;
    TOURNAMENT.clone()
}

pub fn get_tournament() -> Result<Arc<Mutex<Tournament>>, Box<dyn Error>> {
    let current_tournament_status = get_tournament_status();

    match current_tournament_status {
        Err(e) => Err(e.into()),
        _ => Ok(TOURNAMENT.clone()),
    }
}

pub fn get_tournament_status() -> Result<String, Box<dyn Error>> {
    let tournament = TOURNAMENT.lock().unwrap();
    match tournament.status {
        TournamentStatus::Annonce => Ok(String::from("Tournament in announsed")),
        TournamentStatus::Started => Ok(String::from("Tournament in about to start")),
        TournamentStatus::InProgress => Ok(String::from("Tournament in progress")),
        TournamentStatus::Finished => Ok(String::from("Tournament finished")),
        TournamentStatus::Closed => Err("No Tournament at this time".into()),
    }
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
