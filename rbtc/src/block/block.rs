use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::block::transaction::Transactions;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};

/// https://en.bitcoin.it/wiki/Block
/// 
/// # Block
/// 
/// Transaction data is permanently recorded in files called **blocks**. They can be thought of as the individual 
/// pages of a city recorder's recordbook (where changes to title to real estate are recorded) or a stock 
/// transaction ledger. Blocks are organized into a linear sequence over time (also known as the block chain).
/// New transactions are constantly being processed by miners into new blocks which are added to the end of the chain.
/// As blocks are buried deeper and deeper into the blockchain they become harder and harder to change or remove,
/// this gives rise of bitcoin's Irreversible Transactions.
/// 
/// ## Block structure
/// ```
/// +--------------+------------------------------------------+-------+
/// | Field        | Description                              | Bytes |
/// +--------------+------------------------------------------+-------+
/// | Magic no     | value always 0xD9B4BEF9                  | 4     |
/// +--------------+------------------------------------------+-------+
/// | Blocksize    | number of bytes following up to end      | 4     |
/// |              |  of block                                |       |
/// +--------------+------------------------------------------+-------+
/// | Blockheader  | consists of 6 items                      | 80    |
/// +--------------+------------------------------------------+-------+
/// | tx counter   | positive integer VI = VarInt             | 1 - 9 |
/// +--------------+------------------------------------------+-------+
/// | transactions | the (non empty) list of transactions     | var   |
/// +--------------+------------------------------------------+-------+
/// ```
/// 
/// ## Description
/// 
/// Each block contains, among other things, a record of some or all recent transactions, and a reference to
/// the block that came immediately before it. It also contains an answer to a difficult-to-solve mathematical
/// puzzle - the answer to which is unique to each block. New blocks cannot be submitted to the network without
/// the correct answer - the process of "mining" is essentially the process of competing to be the next to find
/// the answer that "solves" the current block. The mathematical problem in each block is extremely difficult
/// to solve, but once a valid solution is found, it is very easy for the rest of the network to confirm that
/// the solution is correct. There are multiple valid solutions for any given block - only one of the solutions 
/// needs to be found for the block to be solved.
/// 
/// Because there is a reward of brand new bitcoins for solving each block, every block also contains a record 
/// of which Bitcoin addresses or scripts are entitled to receive the reward. This record is known as a generation
/// transaction, or a coinbase transaction, and is always the first transaction appearing in every block. 
/// The number of Bitcoins generated per block starts at 50 and is halved every 210,000 blocks (about four years).
/// 
/// Bitcoin transactions are broadcast to the network by the sender, and all peers trying to solve blocks collect 
/// the transaction records and add them to the block they are working to solve. Miners get incentive to include 
/// transactions in their blocks because of attached transaction fees.
/// 
/// The difficulty of the mathematical problem is automatically adjusted by the network, such that it targets a 
/// goal of solving an average of 6 blocks per hour. Every 2016 blocks (solved in about two weeks), all Bitcoin 
/// clients compare the actual number created with this goal and modify the target by the percentage that it varied. 
/// The network comes to a consensus and automatically increases (or decreases) the difficulty of generating blocks.
/// 
/// Because each block contains a reference to the prior block, the collection of all blocks in existence can be 
/// said to form a chain. However, it's possible for the chain to have temporary splits - for example, if two miners 
/// arrive at two different valid solutions for the same block at the same time, unbeknownst to one another. 
/// The peer-to-peer network is designed to resolve these splits within a short period of time, so that only one 
/// branch of the chain survives.
/// 
/// The client accepts the 'longest' chain of blocks as valid. The 'length' of the entire block chain refers to 
/// the chain with the most combined difficulty, not the one with the most blocks. This prevents someone from 
/// forking the chain and creating a large number of low-difficulty blocks, and having it accepted by the 
/// network as 'longest'.
/// 
#[derive(Debug)]
pub struct Block {
    // header
    pub version: u32,
    pub previous: [u8; 32],
    pub merkleroot: [u8; 32],
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
    pub transactions: Transactions
}
impl Block {

    pub fn parse(hex: &Vec<u8>) -> Result<Block, Error> {

        trace!("parse");

        if hex.len() < 81 { // might not be true
            return Err(Error::InvalidLength);
        }
        let mut r = Cursor::new(hex);
        let result = Block::decode(&mut r)?;

        if r.position() as usize != hex.len() {
            return Err(Error::RemainingContent);
        }
        Ok(result)
    }
}

impl Decodable for Block {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Block, Error> {

        trace!("decode");

        let version = u32::decode(r).map_err(|_| Error::BlockVersion)?;
        let previous = <[u8; 32]>::decode(r).map_err(|_| Error::BlockPrevious)?;
        let merkleroot = <[u8; 32]>::decode(r).map_err(|_| Error::BlockMerkleRoot)?;
        let time = u32::decode(r).map_err(|_| Error::BlockTime)?;
        let bits = u32::decode(r).map_err(|_| Error::BlockBits)?;
        let nonce = u32::decode(r).map_err(|_| Error::BlockNonce)?;

        let transactions = Transactions::decode(r)?;

        let result = Block {
            version: version,
            previous: previous,
            merkleroot: merkleroot,
            time: time,
            bits: bits,
            nonce: nonce,
            transactions: transactions
        };

        Ok(result)
    }
}

#[cfg(test)]
mod test {

    use crate::utils::hexdump;
    use crate::encode::error::Error;
    use crate::encode::encode::{Encodable, Decodable};
    use crate::block::block::Block;

    use std::io::Cursor;

    #[test]
    fn when_decode_with_empty_vec_then_fail_parse_error_blockversion() {

        let dump = "
00000000                                                      ................
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = Block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 0);

        if let Err(e) = block {
            assert_eq!(e, Error::BlockVersion);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_with_1_vec_then_fail_parse_error_blockversion() {

        let dump = "
00000000   01                                                 ................
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = Block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 0);

        if let Err(e) = block {
            assert_eq!(e, Error::BlockVersion);
        } else {
            panic!("should have failed");
        }
    }


    #[test]
    fn when_decode_with_4_vec_then_fail_parse_error_blockprevious() {

        let dump = "
00000000   01 00 00 00                                        ver.............
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = Block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 4);

        if let Err(e) = block {
            assert_eq!(e, Error::BlockPrevious);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_with_36_vec_then_fail_parse_error_blockmerkleroot() {

        let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ver.previous.pre
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   vious.previous.p
00000020   00 00 00 00                                        rev.............
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = Block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 36);

        if let Err(e) = block {
            assert_eq!(e, Error::BlockMerkleRoot);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_with_68_vec_then_fail_parse_error_blocktime() {

        let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ver.previous.pre
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   vious.previous.p
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   rev.merkleroot.m
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   erkleroot.merkle
00000040   00 00 00 00                                        roo.............
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = Block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 68);

        if let Err(e) = block {
            assert_eq!(e, Error::BlockTime);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_with_68_vec_then_fail_parse_error_bits() {

        let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ver.previous.pre
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   vious.previous.p
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   rev.merkleroot.m
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   erkleroot.merkle
00000040   00 00 00 00 00 00 00 00                            roo.time........
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = Block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 72);

        if let Err(e) = block {
            assert_eq!(e, Error::BlockBits);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_with_68_vec_then_fail_parse_error_nonce() {

        let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ver.previous.pre
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   vious.previous.p
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   rev.merkleroot.m
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   erkleroot.merkle
00000040   00 00 00 00 00 00 00 00  00 00 00 00               roo.time.bits...
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = Block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 76);

        if let Err(e) = block {
            assert_eq!(e, Error::BlockNonce);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_with_68_vec_then_fail_parse_error_transaction_count() {

        let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ver.previous.pre
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   vious.previous.p
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   rev.merkleroot.m
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   erkleroot.merkle
00000040   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   roo.time.bit.non
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = Block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 80);

        if let Err(e) = block {
            assert_eq!(e, Error::TransactionsCount);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_with_68_vec_then_parse_ok() {

        let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ver.previous.pre
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   vious.previous.p
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   rev.merkleroot.m
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   erkleroot.merkle
00000040   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   roo.tim.bits.no.
00000050   00                                                 t...............
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = Block::decode(&mut c);
        assert!(block.is_ok());
        assert_eq!(c.position(), 81);

        let b : Block = block.unwrap();

        assert_eq!(b.version, 0x00000001, "b.version");
        assert_eq!(b.previous, [0; 32], "b.previous");
        assert_eq!(b.merkleroot, [0; 32], "b.merkleroot");
        assert_eq!(b.time, 0x00000000, "b.time"); // Unix Epoch	1231006505 - Time (UTC)   Sat Jan 03 18:15:05 2009 UTC
        assert_eq!(b.bits, 0x00000000, "b.bits");
        assert_eq!(b.nonce, 0x00000000, "b.nonce");
        assert_eq!(b.transactions.len(), 0, "b.transactions.len");


    }

    #[test]
    fn when_decode_with_68_numbered_then_parse_ok() {

        let dump = "
00000000   00 01 02 03 04 05 06 07 08  09 0A 0B 0C 0D 0E 0F   ver.previous.pre
00000010   00 01 02 03 04 05 06 07 08  09 0A 0B 0C 0D 0E 0F   vious.previous.p
00000030   00 01 02 03 04 05 06 07 08  09 0A 0B 0C 0D 0E 0F   rev.merkleroot.m
00000030   00 01 02 03 04 05 06 07 08  09 0A 0B 0C 0D 0E 0F   erkleroot.merkle
00000040   00 01 02 03 04 05 06 07 08  09 0A 0B 0C 0D 0E 0F   roo.tim.bits.no.
00000050   00                                                 t...............
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = Block::decode(&mut c);
        assert!(block.is_ok());
        assert_eq!(c.position(), 81);

        let b : Block = block.unwrap();

        assert_eq!(b.version, 0x03020100, "b.version");
        assert_eq!(b.previous, [
            0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01, 0x02, 0x03
        ], "b.previous");
        assert_eq!(b.merkleroot, [
            0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01, 0x02, 0x03
        ],"b.merkleroot");
        assert_eq!(b.time, 0x07060504, "b.time"); // Unix Epoch	1231006505 - Time (UTC)   Sat Jan 03 18:15:05 2009 UTC
        assert_eq!(b.bits, 0x0B0A0908, "b.bits");
        assert_eq!(b.nonce, 0x0F0E0D0C, "b.nonce");
        assert_eq!(b.transactions.len(), 0, "b.transactions.len");

    }
}