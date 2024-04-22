// SPDX-License-Identifier: MIT

use netlink_packet_utils::DecodeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddressError {
    #[error(
        "Invalid {kind}, got unexpected length of payload: {payload_length}"
    )]
    ParseAttributeInvalidPayload {
        kind: &'static str,
        payload_length: usize,
    },

    #[error("Invalid {kind} value")]
    ParseAttribute {
        kind: &'static str,
        err: DecodeError,
    },

    #[error("unknown NLA {kind}")]
    ParseUnknownNLA { kind: u16, err: DecodeError },

    #[error("Failed to aprse nla attribute: {0:?}")]
    NlaAttribute(DecodeError),

    #[error("Faield to parse address buffer: {0:?}")]
    FailedBufferInit(DecodeError),
}
