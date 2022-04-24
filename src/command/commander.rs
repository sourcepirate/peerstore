use super::network::NetworkCommand;
use super::storage::StorageCommand;

pub enum CommandType {
    StorageCommand,
    NetworkCommand,
    UnknownCommand,
}

pub fn resolve(command: String) -> CommandType {
    let network_command: NetworkCommand = command.clone().into();
    match network_command {
        NetworkCommand::UnknownCommand => {
            let storage_command: StorageCommand = command.into();
            match storage_command {
                StorageCommand::UnknownCommand => CommandType::UnknownCommand,
                _ => CommandType::StorageCommand,
            }
        }
        _ => CommandType::NetworkCommand,
    }
}
