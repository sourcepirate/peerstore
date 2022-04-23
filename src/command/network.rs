
use std::net::SocketAddrV4;
use std::convert::From;

pub enum NetworkCommand {
    Join(SocketAddrV4),
    Remove(SocketAddrV4),
    UnknownCommand
}

impl From<String> for NetworkCommand {

    fn from(cmd: String) -> NetworkCommand {
        let tokens : Vec<&str> = (&cmd).split(char::is_whitespace).collect();
        if (tokens.len() <= 0) {
            NetworkCommand::UnknownCommand
        } else {
            match tokens[0].to_lowercase().as_str() {
                "join" => NetworkCommand::Join(tokens[1].parse().unwrap()),
                "remove" => NetworkCommand::Remove(tokens[1].parse().unwrap()),
                _ => NetworkCommand::UnknownCommand
            }
        }
    }
}