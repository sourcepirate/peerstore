
pub enum CommandError {
    UnknownCommand,
    InvalidArgument
}

pub type CommandResult<T> = Result<T, CommandError>;