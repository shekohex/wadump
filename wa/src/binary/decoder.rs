use super::{
    node::{Nibble, Node, NodeContent},
    Jid,
};
use crate::{constants::*, Result};
use byteorder::{BigEndian, ReadBytesExt};
use log::error;
use std::io::Read;

pub(super) fn read_list_size(tag: u8, stream: &mut impl Read) -> Result<u16> {
    match tag {
        LIST_EMPTY => Ok(0),
        LIST_8 => Ok(u16::from(stream.read_u8()?)),
        LIST_16 => Ok(stream.read_u16::<BigEndian>()?),
        _ => {
            error!("Invalid List Size, tag {}", tag);
            Ok(0)
        },
    }
}

pub(super) fn read_list(tag: u8, stream: &mut impl Read) -> Result<Vec<Node>> {
    let size = read_list_size(tag, stream)?;
    let mut list = Vec::<Node>::with_capacity(size as usize);
    for _ in 0..size {
        list.push(Node::deserialize_stream(stream)?);
    }
    Ok(list)
}

pub(super) fn read_node_content(
    tag: u8,
    stream: &mut impl Read,
) -> Result<NodeContent> {
    match tag {
        3..=TOKENS_LEN => Ok(NodeContent::Token(TOKENS[(tag - 3) as usize])),
        DICTIONARY_0 | DICTIONARY_1 | DICTIONARY_2 | DICTIONARY_3 => {
            stream.read_u8()?;
            Ok(NodeContent::List(Vec::new()))
        },
        LIST_EMPTY | LIST_8 | LIST_16 => {
            Ok(NodeContent::List(read_list(tag, stream)?))
        },
        BINARY_8 => {
            let mut buffer = vec![0u8; stream.read_u8()? as usize];
            stream.read_exact(&mut buffer)?;
            Ok(String::from_utf8(buffer)
                .map(|string| NodeContent::String(string.into()))
                .unwrap_or_else(|err| NodeContent::Binary(err.into_bytes())))
        },
        BINARY_20 => {
            let len = stream.read_uint::<BigEndian>(20)? as usize;
            let mut buffer = vec![0u8; len];
            stream.read_exact(&mut buffer)?;
            Ok(String::from_utf8(buffer)
                .map(|string| NodeContent::String(string.into()))
                .unwrap_or_else(|err| NodeContent::Binary(err.into_bytes())))
        },
        BINARY_32 => {
            let mut buffer =
                vec![0u8; stream.read_u32::<BigEndian>()? as usize];
            stream.read_exact(&mut buffer)?;
            Ok(String::from_utf8(buffer)
                .map(|string| NodeContent::String(string.into()))
                .unwrap_or_else(|err| NodeContent::Binary(err.into_bytes())))
        },
        JID_PAIR => Ok(NodeContent::Jid(Jid::from_node_pair(
            read_node_content(stream.read_u8()?, stream)?.into_string(),
            read_node_content(stream.read_u8()?, stream)?.as_str(),
        ))),
        NIBBLE_8 | HEX_8 => read_node_packed8(tag, stream),
        _ => {
            error!("Invalid Tag, tag {}", tag);
            Ok(NodeContent::None)
        },
    }
}

pub(super) fn read_node_packed8(
    tag: u8,
    stream: &mut dyn Read,
) -> Result<NodeContent> {
    let start_byte = stream.read_u8()?;
    let mut string = String::with_capacity((start_byte as usize & 127) * 2);

    for _ in 0..(start_byte & 127) {
        let byte = stream.read_u8()?;
        if tag == HEX_8 {
            string.push(
                std::char::from_digit(u32::from((byte >> 4) & 0x0F), 16)
                    .unwrap()
                    .to_ascii_uppercase(),
            );
            string.push(
                std::char::from_digit(u32::from(byte & 0x0F), 16)
                    .unwrap()
                    .to_ascii_uppercase(),
            );
        } else {
            let mut nibble = Nibble::from((byte >> 4) & 0x0F);
            if *nibble == '\0' {
                return Ok(NodeContent::Nibble(string.into()));
            }
            string.push(*nibble);

            nibble = Nibble::from(byte & 0x0F);
            if *nibble == '\0' {
                return Ok(NodeContent::Nibble(string.into()));
            }
            string.push(*nibble);
        }
    }
    // if start_byte >> 7 == 0 {
    // let len = string.len();
    // string.split_off(len - 1);
    // }
    Ok(NodeContent::String(string.into()))
}
