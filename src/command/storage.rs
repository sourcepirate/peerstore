pub enum StorageCommand {
    Get(String),
    Put(String, String),
    Delete(String),
    UnknownCommand,
}

impl From<String> for StorageCommand {
    fn from(cmd: String) -> StorageCommand {
        let tokens: Vec<&str> = (&cmd).split(char::is_whitespace).collect();
        if tokens.len() <= 0 {
            StorageCommand::UnknownCommand
        } else {
            match tokens[0].to_lowercase().as_str() {
                "get" => StorageCommand::Get(tokens[1].to_owned()),
                "put" => StorageCommand::Put(tokens[1].to_owned(), tokens[2].to_owned()),
                "delete" => StorageCommand::Delete(tokens[1].to_owned()),
                _ => StorageCommand::UnknownCommand,
            }
        }
    }
}
