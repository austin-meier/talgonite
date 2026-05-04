use crate::{ToBytes, types::BodyAnimationKind};

use super::Codes;

#[derive(Debug)]
pub struct Emote {
    pub animation: BodyAnimationKind,
}

impl ToBytes for Emote {
    const OPCODE: u8 = Codes::Emote as _;

    fn write_payload(&self, bytes: &mut Vec<u8>) {
        bytes.push((self.animation as u8) - 9);
    }
}
