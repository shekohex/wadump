use super::{
    decoder::{read_list_size, read_node_content},
    Jid,
};
use crate::{constants::*, Result};
use byteorder::{BigEndian, ReadBytesExt};
use core::{
    fmt::{self, Debug},
    ops::Deref,
};
use std::{
    borrow::Cow,
    collections::HashMap,
    io::{Cursor, Read},
};

#[derive(Clone)]
pub enum NodeContent {
    None,
    List(Vec<Node>),
    String(Cow<'static, str>),
    Binary(Vec<u8>),
    Jid(Jid),
    Token(&'static str),
    Nibble(Cow<'static, str>),
}

impl Debug for NodeContent {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NodeContent::*;
        match self {
            None => write!(f, "None"),
            String(v) => write!(f, "{}", v),
            Jid(v) => write!(f, "{}", v),
            Token(s) => write!(f, "{}", s),
            Nibble(n) => write!(f, "{}", n),
            List(nodes) => nodes.fmt(&mut f),
            Binary(_) => write!(f, "[...]"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Nibble(pub(super) char);

impl From<char> for Nibble {
    fn from(c: char) -> Self { Nibble(c) }
}

impl From<u8> for Nibble {
    fn from(b: u8) -> Self {
        let r = match b {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => '-',
            11 => '.',
            15 => '\0',
            _ => unreachable!("Unknown byte for nibble {}", b),
        };
        Nibble(r)
    }
}

impl From<Nibble> for u8 {
    fn from(nibble: Nibble) -> Self {
        match *nibble {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            '-' => 10,
            '.' => 11,
            '\0' => 15,
            _ => unreachable!("Unknown char for nibble {}", nibble.0),
        }
    }
}

impl Deref for Nibble {
    type Target = char;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Default for NodeContent {
    fn default() -> Self { NodeContent::None }
}

impl NodeContent {
    pub(crate) fn into_cow(self) -> Cow<'static, str> {
        match self {
            NodeContent::None => "".into(),
            NodeContent::List(_) => {
                unimplemented!("We Need to handle the list")
            },
            NodeContent::String(string) => string,
            NodeContent::Nibble(string) => string,
            NodeContent::Binary(_) => {
                unimplemented!("We need to handle the Binary data")
            },
            NodeContent::Jid(jid) => Cow::Owned(jid.to_string()),
            NodeContent::Token(ref token) => Cow::Borrowed(token),
        }
    }

    pub(crate) fn into_string(self) -> String {
        match self {
            NodeContent::None => "".to_string(),
            NodeContent::List(n) => {
                n.into_iter().map(|n| n.content.into_string()).collect()
            },
            NodeContent::String(string) => string.into(),
            NodeContent::Nibble(string) => string.into(),
            NodeContent::Binary(_) => String::new(),
            NodeContent::Jid(jid) => jid.to_string(),
            NodeContent::Token(ref token) => (*token).to_string(),
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        match *self {
            NodeContent::None => "",
            NodeContent::List(_) => "",
            NodeContent::String(ref string) => string.deref(),
            NodeContent::Nibble(ref string) => string.deref(),
            NodeContent::Binary(_) => {
                unimplemented!("unhandled binary to string")
            },
            NodeContent::Jid(ref jid) => jid.id(),
            NodeContent::Token(ref token) => token,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Node {
    pub desc: Cow<'static, str>,
    pub attributes: HashMap<Cow<'static, str>, NodeContent>,
    pub content: NodeContent,
}

impl Node {
    #[inline(always)]
    pub fn deserialize(data: &[u8]) -> Result<Node> {
        Node::deserialize_stream(&mut Cursor::new(data))
    }

    pub(crate) fn deserialize_stream(stream: &mut impl Read) -> Result<Node> {
        let list_size = read_list_size(stream.read_u8()?, stream)?;
        let desc = read_node_content(stream.read_u8()?, stream)?.into_cow();

        let mut attributes = HashMap::new();

        for _ in 0..((list_size - 1) >> 1) {
            let attribute_name =
                read_node_content(stream.read_u8()?, stream)?.into_cow();
            let attribute_content =
                read_node_content(stream.read_u8()?, stream)?;

            attributes.insert(attribute_name, attribute_content);
        }

        let content = if list_size % 2 == 1 {
            NodeContent::None
        } else {
            let tag = stream.read_u8()?;
            match tag {
                BINARY_8 => {
                    let mut buffer = vec![0u8; stream.read_u8()? as usize];
                    stream.read_exact(&mut buffer)?;
                    NodeContent::Binary(buffer)
                },
                BINARY_20 => {
                    let len: usize = ((stream.read_u8()? as usize & 0x0F)
                        << 16)
                        | (stream.read_u8()? as usize) << 8
                        | stream.read_u8()? as usize;

                    let mut buffer = vec![0u8; len];
                    stream.read_exact(&mut buffer)?;
                    NodeContent::Binary(buffer)
                },
                BINARY_32 => {
                    let mut buffer =
                        vec![0u8; stream.read_u32::<BigEndian>()? as usize];
                    stream.read_exact(&mut buffer)?;
                    NodeContent::Binary(buffer)
                },
                _ => read_node_content(tag, stream)?,
            }
        };

        Ok(Node {
            desc,
            attributes,
            content,
        })
    }
}

impl Jid {
    pub(crate) fn id(&self) -> &str { &self.id }

    pub(crate) fn from_node_pair(id: String, suffix: &str) -> Self {
        Self {
            id,
            is_remote: suffix == "@s.whatsapp.net",
            is_group: suffix == "@g.us",
            is_broadcast: suffix == "@broadcast",
        }
    }
}
