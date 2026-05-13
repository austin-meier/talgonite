use crate::TryFromBytes;
use anyhow::anyhow;
use byteorder::ReadBytesExt;
use encoding::all::WINDOWS_949;
use encoding::{DecoderTrap, Encoding};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum LoginControlsType {
    /// <summary>
    ///     Tells the client that the packet contains the homepage url
    /// </summary>
    Homepage = 3,
}

#[derive(Debug)]
pub enum ServerInfo {
    Homepage { url: String },
}

impl ServerInfo {
    fn parse_homepage(cursor: &mut std::io::Cursor<&[u8]>) -> anyhow::Result<Self> {
        let message = {
            let mut buf = vec![0; cursor.read_u8()? as usize];
            cursor.read_exact(&mut buf)?;
            WINDOWS_949
                .decode(&buf, DecoderTrap::Replace)
                .map_err(|e| anyhow!("Failed to decode message: {}", e))?
        };
        Ok(ServerInfo::Homepage { url: message })
    }
}

impl TryFromBytes for ServerInfo {
    fn try_from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut cursor = std::io::Cursor::new(bytes);
        let login_controls_type: LoginControlsType = cursor.read_u8()?.try_into()?;

        match login_controls_type {
            LoginControlsType::Homepage => Self::parse_homepage(&mut cursor),
        }
    }
}
