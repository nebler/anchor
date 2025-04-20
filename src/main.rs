use std::fmt;
use std::io::{self};

use anyhow::Ok;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    src: String,
    dest: String,
    body: BodyMessage,
}

// Manual Display implementation
impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Message from {} to {}: {:?}",
            self.src, self.dest, self.body
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BodyMessage {
    #[serde(flatten)]
    message_type: MessageType,
    msg_id: i128,
    in_reply_to: Option<i128>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InitMessage {
    src: String,
    dest: String,
    body: InitBody,
}

// Manual Display implementation
impl fmt::Display for InitMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Message from {} to {}: {:?}",
            self.src, self.dest, self.body
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct InitBody {
    #[serde(flatten)]
    message_type: MessageType,
    msg_id: i128,
    in_reply_to: Option<i128>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum MessageType {
    Init,
    InitOk,
    Echo { echo: String },
    EchoOk { echo: String },
}

// Manual Display implementation for MessageType
impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Init => write!(f, "Init Message"),
            MessageType::InitOk => write!(f, "Init Message"),
            MessageType::Echo { echo: String } => todo!(),
            MessageType::EchoOk { echo: String } => todo!(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut buffer = String::new();

    stdin.read_line(&mut buffer).unwrap();
    let init = buffer.trim_end();
    let value: InitMessage = serde_json::from_str(init)?;

    let response = InitMessage {
        body: InitBody {
            message_type: MessageType::InitOk,
            in_reply_to: Some(value.body.msg_id),
            ..value.body
        },
        dest: value.src,
        src: value.dest,
    };
    eprintln!("about to send {}", response);
    println!("{}", serde_json::to_string(&response).unwrap());
    eprintln!("Sent init ok!");
    buffer.clear();

    while stdin.read_line(&mut buffer).is_ok() {
        let trimmed = buffer.trim_end();
        eprintln!("Received: {}", trimmed);
        let value: Message = serde_json::from_str(trimmed)?;
        match value.body.message_type {
            MessageType::Init => {}
            MessageType::InitOk => (),
            MessageType::Echo { echo } => {
                eprintln!("Consturcting message");
                let response = Message {
                    body: BodyMessage {
                        message_type: MessageType::EchoOk { echo },
                        in_reply_to: Some(value.body.msg_id),
                        msg_id: value.body.msg_id,
                    },
                    dest: value.src,
                    src: value.dest,
                };
                println!("{}", serde_json::to_string(&response).unwrap());
                eprintln!("Sent echo ok!");
            }
            MessageType::EchoOk { echo } => (),
        }

        buffer.clear();
    }
    Ok(())
}
