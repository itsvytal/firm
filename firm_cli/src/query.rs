use clap::ValueEnum;
use firm_core::graph::Direction;

/// Wraps the underlying graph direction enum, allowing it to be used by clap.
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
