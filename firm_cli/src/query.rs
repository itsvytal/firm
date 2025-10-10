use clap::ValueEnum;
use firm_core::graph::Direction;

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum CliDirection {
    To,
    From,
}

impl From<CliDirection> for Direction {
    fn from(dir: CliDirection) -> Direction {
        match dir {
            CliDirection::To => Direction::Incoming,
            CliDirection::From => Direction::Outgoing,
        }
    }
}
