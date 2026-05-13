use crate::ToBytes;

use super::Codes;

#[derive(Debug)]
pub struct HomepageRequest;

impl ToBytes for HomepageRequest {
    const OPCODE: u8 = Codes::HomepageRequest as _;

    fn write_payload(&self, bytes: &mut Vec<u8>) {
        bytes.push(1);
    }
}
