// https://github.com/bitcoin/bitcoin/blob/master/src/primitives/transaction.h
//
// /** An outpoint - a combination of a transaction hash and an index n into its vout */
// class COutPoint
// {
// public:
//     uint256 hash;
//     uint32_t n;

#[derive(Debug)]
pub struct OutPoint {
    hash: [u8; 32],
    index: u32,
}