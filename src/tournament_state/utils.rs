use crate::tournament_state::MatchFormat;

pub fn parse_tournament_format(format: u8) -> MatchFormat {
    match format {
        1 => MatchFormat::BO1,
        3 => MatchFormat::BO3,
        5 => MatchFormat::BO5,
        _ => panic!("Invalid match format"),
    }
}
