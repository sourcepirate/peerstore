use storage::StorageCommand;
use network::NetworkCommand:

pub enum CommandType {
    StorageCommand,
    NetworkCommand,
    UnknownCommand
}

fn resolve(command_input: String) -> 