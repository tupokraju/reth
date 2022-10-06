use crate::{Address, Bytes, H256};

/// Ethereum Log
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Log {
    /// Contract that emitted this log.
    pub address: Address,
    /// Topics of the log. The number of logs depend on what `LOG` opcode is used.
    pub topics: Vec<H256>,
    /// Arbitrary length data.
    pub data: Bytes,
}