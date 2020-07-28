#![deny(unsafe_code)]

use argh::FromArgs;
use keyring::Keyring;
use prost::Message;
use std::io;
use wa::{Node, NodeContent};

const SERVICE_NAME: &str = "wadump";
const ENC_KEY_NAME: &str = "enc_key";
const MAC_KEY_NAME: &str = "mac_key";

type AnyError = Box<dyn std::error::Error>;

/// CLI Tool to dump WhatsApp Packets
#[derive(Debug, FromArgs)]
struct Args {
    /// reset the saved session and clear the keys
    #[argh(switch, short = 'r')]
    reset: bool,
    /// is this message outgoing? (i.e you are the sender)
    #[argh(switch)]
    outgoing: bool,
    /// a base64 message to be decrypted and dumped,
    /// if not provided it will try to read it from stdin
    #[argh(option, short = 'm')]
    message: Option<String>,
}

fn main() -> Result<(), AnyError> {
    let args: Args = argh::from_env();
    let enc_key_service = Keyring::new(SERVICE_NAME, ENC_KEY_NAME);
    let mac_key_service = Keyring::new(SERVICE_NAME, MAC_KEY_NAME);

    if args.reset {
        enc_key_service.delete_password()?;
        mac_key_service.delete_password()?;
        return Ok(());
    }

    let enc_key = match enc_key_service.get_password() {
        Ok(key) => key,
        Err(_) => {
            let key = rpassword::prompt_password_stdout(
                "Enter Encryption Key (base64):",
            )?;
            enc_key_service.set_password(&key)?;
            key
        },
    };
    let mac_key = match mac_key_service.get_password() {
        Ok(key) => key,
        Err(_) => {
            let key =
                rpassword::prompt_password_stdout("Enter MAC Key (base64):")?;
            mac_key_service.set_password(&key)?;
            key
        },
    };

    let enc = base64::decode(enc_key)?;
    let mac = base64::decode(mac_key)?;

    let pad = if args.outgoing {
        3 // these there bytes for the [metric, b',', flag]
    } else {
        1 // one for the , it self
    };

    let msg = if let Some(msg) = args.message {
        base64::decode(msg)?
    } else {
        // ok, let's try to read it from stdin
        let mut msg = String::new();
        io::stdin().read_line(&mut msg)?;
        // remove any lines brakes
        #[cfg(unix)]
        let msg = msg.replace('\n', "");
        #[cfg(windows)]
        let msg = msg.replace("\r\n", "");
        base64::decode(msg)?
    };

    let sep = msg
        .iter()
        .position(|x| x == &b',')
        .expect("message should have the `,` separator!")
        + pad;

    let message_tag = std::str::from_utf8(&msg[..(sep - pad)])?;
    println!("Message Tag: {}", message_tag);

    let msg = wa::verify_and_decrypt_message(&enc[..], &mac[..], &msg[sep..])?;
    let payload = Node::deserialize(&msg)?;

    if let NodeContent::List(nodes) = &payload.content {
        handle_node_list(nodes)?;
    } else {
        println!("{:#?}", payload);
    }

    Ok(())
}

fn handle_node_list(nodes: &[Node]) -> Result<(), AnyError> {
    for node in nodes {
        match node.desc.as_ref() {
            "message" => {
                handle_message(&node.content)?;
            },
            unknown => {
                eprintln!("got unhandled node of desc: {}", unknown);
                println!("{:#?}", node);
            },
        }
    }
    Ok(())
}

fn handle_message(content: &NodeContent) -> Result<(), AnyError> {
    if let NodeContent::Binary(ref content) = content {
        let msg = wa::WebMessageInfo::decode(content.as_slice())?;
        println!("{:#?}", msg);
    }
    Ok(())
}
