use crate::block::transaction;
use crate::block::error::DecodeError;
use crate::primitives::block::Block;

use std::io::Read;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub fn parse(hex: &Vec<u8>) -> Result<Block, DecodeError> {

    if hex.len() < 81 { // might not be true
        return Err(DecodeError::InvalidLength);
    }
    let mut r = Cursor::new(hex);
    let result = decode(&mut r)?;

    if r.position() as usize != hex.len() {
        return Err(DecodeError::RemainingContent);
    }
    Ok(result)
}

/// https://en.bitcoin.it/wiki/Block
/// 
/// Block structure
/// +----------------------+------------------------------------------------+--------------------------+
/// | Field                | Description                                    | Size                     |
/// +----------------------+------------------------------------------------+--------------------------+
/// | Magic no             | value always 0xD9B4BEF9                        |  4 bytes                 |
/// +----------------------+------------------------------------------------+--------------------------+
/// | Blocksize            | number of bytes following up to end of block   |  4 bytes                 |
/// +----------------------+------------------------------------------------+--------------------------+
/// | Blockheader          | consists of 6 items                            | 80 bytes                 |
/// +----------------------+------------------------------------------------+--------------------------+
/// | Transaction counter  | positive integer VI = VarInt                   |  1 - 9 bytes             |
/// +----------------------+------------------------------------------------+--------------------------+
/// | transactions         | the (non empty) list of transactions           |  <Transaction counter>-  |
/// |                      |                                                |  many transactions       |
/// +----------------------+------------------------------------------------+--------------------------+
/// 
fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Block, DecodeError> {

    let version = r.read_u32::<LittleEndian>().map_err(|_| DecodeError::BlockVersion)?;

    let mut previous = [0; 32];
    r.read_exact(&mut previous).map_err(|_| DecodeError::BlockPrevious)?;

    let mut merkleroot = [0; 32];
    r.read_exact(&mut merkleroot).map_err(|_| DecodeError::BlockMerkleRoot)?;

    let time = r.read_u32::<LittleEndian>().map_err(|_| DecodeError::BlockTime)?;
    let bits = r.read_u32::<LittleEndian>().map_err(|_| DecodeError::BlockBits)?;
    let nonce = r.read_u32::<LittleEndian>().map_err(|_| DecodeError::BlockNonce)?;

    let transactions = transaction::decode_all(r)?;

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

#[cfg(test)]
mod test {

    use crate::block::block;
    use crate::utils::hexdump;
    use crate::block::error::DecodeError;

    use crate::primitives::block::Block;
    use std::io::Cursor;

    #[test]
    fn when_decode_with_empty_vec_then_fail_parse_error_blockversion() {

        let dump = "
00000000                                                      ................
";

        let data : Vec<u8> = hexdump::parse(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 0);

        if let Err(e) = block {
            assert_eq!(e, DecodeError::BlockVersion);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_with_1_vec_then_fail_parse_error_blockversion() {

        let dump = "
00000000   01                                                 ................
";

        let data : Vec<u8> = hexdump::parse(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 0);

        if let Err(e) = block {
            assert_eq!(e, DecodeError::BlockVersion);
        } else {
            panic!("should have failed");
        }
    }


    #[test]
    fn when_decode_with_4_vec_then_fail_parse_error_blockprevious() {

        let dump = "
00000000   01 00 00 00                                        ver.............
";

        let data : Vec<u8> = hexdump::parse(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 4);

        if let Err(e) = block {
            assert_eq!(e, DecodeError::BlockPrevious);
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

        let data : Vec<u8> = hexdump::parse(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 36);

        if let Err(e) = block {
            assert_eq!(e, DecodeError::BlockMerkleRoot);
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

        let data : Vec<u8> = hexdump::parse(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 68);

        if let Err(e) = block {
            assert_eq!(e, DecodeError::BlockTime);
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

        let data : Vec<u8> = hexdump::parse(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 72);

        if let Err(e) = block {
            assert_eq!(e, DecodeError::BlockBits);
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

        let data : Vec<u8> = hexdump::parse(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 76);

        if let Err(e) = block {
            assert_eq!(e, DecodeError::BlockNonce);
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

        let data : Vec<u8> = hexdump::parse(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = block::decode(&mut c);
        assert!(block.is_err());
        assert_eq!(c.position(), 80);

        if let Err(e) = block {
            assert_eq!(e, DecodeError::TransactionsCount);
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

        let data : Vec<u8> = hexdump::parse(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = block::decode(&mut c);
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

        let data : Vec<u8> = hexdump::parse(dump);
        let mut c = Cursor::new(data.as_ref());
        let block = block::decode(&mut c);
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