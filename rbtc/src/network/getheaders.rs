use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::utils::sha256::Sha256;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};


/// https://github.com/rust-bitcoin/rust-bitcoin/blob/45140a3251d9eca8d17baf7a4e900a4ac5baae3b/src/network/message_blockdata.rs
/// The `getheaders` message
/// 
/// https://en.bitcoin.it/wiki/Protocol_documentation
/// getheaders 
/// Return a headers packet containing the headers of blocks starting right after the last known hash 
/// in the block locator object, up to hash_stop or 2000 blocks, whichever comes first. To receive the 
/// next block headers, one needs to issue getheaders again with a new block locator object. 
/// Keep in mind that some clients may provide headers of blocks which are invalid if the block 
/// locator object contains a hash on the invalid branch.
/// 
/// Payload:
/// 
/// +---------------+--------------------------+-------------+-------------------------------------------------------+
/// | Field Size    | Description              | Data type   | Comments                                              | 
/// +---------------+--------------------------+-------------+-------------------------------------------------------+ 
/// |     4         | version                  | uint32_t    | the protocol version                                  |
/// +---------------+--------------------------+-------------+-------------------------------------------------------+ 
/// |     1+        | hash count               | var_int     | number of block locator hash entries                  |
/// +---------------+--------------------------+-------------+-------------------------------------------------------+ 
/// |     32+       | block locator hashes     | char[32]    | block locator object; newest back to genesis block    |
/// |               |                          |             | (dense to start, but then sparse)                     |
/// +---------------+--------------------------+-------------+-------------------------------------------------------+ 
/// |     32        | hash_stop                | char[32]    | hash of the last desired block header;                |
/// |               |                          |             | set to zero to get as many blocks as possible (2000)  |
/// +---------------+--------------------------+-------------+-------------------------------------------------------+ 
/// 
/// For the block locator object in this packet, the same rules apply as for the getblocks packet.
/// 
#[derive(Debug)]
pub struct GetHeaders {
    /// The protocol version
    pub version: u32,
    /// Locator hashes --- ordered newest to oldest. The remote peer will
    /// reply with its longest known chain, starting from a locator hash
    /// if possible and block 1 otherwise.
    pub locators: Vec<Sha256>,
    /// References the header to stop at, or zero to just fetch the maximum 2000 headers
    pub stop: Sha256
}

impl Encodable for GetHeaders {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {

        trace!("encode");
        self.version.encode(w).map_err(|_| Error::GetHeadersVersion)?;
        self.locators.encode(w).map_err(|_| Error::GetHeadersLocators)?;
        self.stop.encode(w).map_err(|_| Error::GetHeadersStop)?;

        Ok(())
    }
}
impl Decodable for GetHeaders {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<GetHeaders, Error> {

        trace!("decode");
        let version = u32::decode(r).map_err(|_| Error::GetHeadersVersion)?;
        let locators = <Vec<Sha256>>::decode(r).map_err(|_| Error::GetHeadersLocators)?;
        let stop = Sha256::decode(r).map_err(|_| Error::GetHeadersStop)?;

        let result = GetHeaders {
            version: version,
            locators: locators,
            stop: stop
        };

        Ok(result)
    }
}

#[cfg(test)]
mod test {

    use crate::network::message::Payload;
    use crate::encode::encode::{Encodable, Decodable};
    use crate::network::getheaders;
    use crate::network::getheaders::GetHeaders;
    use crate::network::getheaders::Error;
    use crate::utils::hexdump;
    use crate::utils::sha256::Sha256;

    use std::io::Cursor;

    #[test]
    fn when_decode_with_empty_vec_then_fail_parse_error_version() {

        let dump = "
00000000                                                      ................
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let result = GetHeaders::decode(&mut c);
        assert!(result.is_err());
        assert_eq!(c.position(), 0);

        if let Err(e) = result {
            assert_eq!(e, Error::GetHeadersVersion);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_with_1_vec_then_fail_parse_error_version() {

        let dump = "
00000000   01                                                 ................
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let result = GetHeaders::decode(&mut c);
        assert!(result.is_err());
        assert_eq!(c.position(), 0);

        if let Err(e) = result {
            assert_eq!(e, Error::GetHeadersVersion);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_with_4_vec_then_fail_parse_error_locators() {

        let dump = "
00000000   01 00 00 00                                        ver.............
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        let result = GetHeaders::decode(&mut c);
        assert!(result.is_err());
        assert_eq!(c.position(), 4);

        if let Err(e) = result {
            assert_eq!(e, Error::GetHeadersLocators);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_getheaders_valid_dump_then_decode_ok() {

        let dump = "
00000000   f9 be b4 d9 67 65 74 68  65 61 64 65 72 73 00 00   main.getheaders.
00000010   01 02 03 04 01 02 03 04  71 11 01 00 02 10 10 10   siz.has.ver.c.bl
00000020   10 11 11 11 11 12 12 12  12 13 13 13 13 14 14 14   ock1.block1.bloc
00000030   14 15 15 15 15 16 16 16  16 00 00 00 00 20 20 20   block1.block1..b
00000040   20 21 21 21 21 22 22 22  22 23 23 23 23 24 24 24   lock2.block2.blo
00000050   24 25 25 25 25 26 26 26  26 00 00 00 00 30 30 30   ck2.block2.blo.s
00000060   30 31 31 31 31 32 32 32  32 33 33 33 33 34 34 34   top.stop.stop.st
00000070   34 35 35 35 35 36 36 36  36 00 00 00 00            top.stop.sto
";

        let data : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(data.as_ref());
        c.set_position(24); // move beyond network protocol headers

        let result = GetHeaders::decode(&mut c);
        assert!(result.is_ok());
        assert_eq!(c.position() as usize, data.len());

        let message : GetHeaders = result.unwrap();
        assert_eq!(message.version, 70001);
        assert_eq!(message.locators.len(), 0x02);

        let locator1 : &Sha256 = message.locators.get(0).unwrap();
        assert_eq!(locator1.hash, [
            0x10, 0x10, 0x10, 0x10, 0x11, 0x11, 0x11, 0x11, 0x12, 0x12, 0x12, 0x12, 0x13, 0x13, 0x13, 0x13,  
            0x14, 0x14, 0x14, 0x14, 0x15, 0x15, 0x15, 0x15, 0x16, 0x16, 0x16, 0x16, 0x00, 0x00, 0x00, 0x00,
        ]); 

        let locator2 : &Sha256 = message.locators.get(1).unwrap();
        assert_eq!(locator2.hash, [
            0x20, 0x20, 0x20, 0x20, 0x21, 0x21, 0x21, 0x21, 0x22, 0x22, 0x22, 0x22, 0x23, 0x23, 0x23, 0x23,  
            0x24, 0x24, 0x24, 0x24, 0x25, 0x25, 0x25, 0x25, 0x26, 0x26, 0x26, 0x26, 0x00, 0x00, 0x00, 0x00,
        ]); 

        let stop : Sha256 = message.stop;
        assert_eq!(stop.hash, [
            0x30, 0x30, 0x30, 0x30, 0x31, 0x31, 0x31, 0x31, 0x32, 0x32, 0x32, 0x32, 0x33, 0x33, 0x33, 0x33,  
            0x34, 0x34, 0x34, 0x34, 0x35, 0x35, 0x35, 0x35, 0x36, 0x36, 0x36, 0x36, 0x00, 0x00, 0x00, 0x00,
        ]); 
    }

    #[test]
    fn when_decode_encode_getheaders_then_same() {

        let dump = "
00000000                            71 11 01 00 02 10 10 10           ver.c.bl
00000010   10 11 11 11 11 12 12 12  12 13 13 13 13 14 14 14   ock1.block1.bloc
00000020   14 15 15 15 15 16 16 16  16 00 00 00 00 20 20 20   block1.block1..b
00000030   20 21 21 21 21 22 22 22  22 23 23 23 23 24 24 24   lock2.block2.blo
00000040   24 25 25 25 25 26 26 26  26 00 00 00 00 30 30 30   ck2.block2.blo.s
00000050   30 31 31 31 31 32 32 32  32 33 33 33 33 34 34 34   top.stop.stop.st
00000060   34 35 35 35 35 36 36 36  36 00 00 00 00            top.stop.sto
";

        let original : Vec<u8> = hexdump::decode(dump);
        let mut c = Cursor::new(original.as_ref());

        let decoded = GetHeaders::decode(&mut c);
        assert!(decoded.is_ok());
        assert_eq!(c.position() as usize, original.len());

        let mut result : Vec<u8> = Vec::new();
        let encoded = decoded.unwrap().encode(&mut result);
        assert!(encoded.is_ok());

        assert_eq!(original, result);
    }
    
    #[test]
    fn when_encode_getheaders_then_same() {

        let dump = "
00000000                            71 11 01 00 02 10 10 10           ver.c.bl
00000010   10 11 11 11 11 12 12 12  12 13 13 13 13 14 14 14   ock1.block1.bloc
00000020   14 15 15 15 15 16 16 16  16 00 00 00 00 20 20 20   block1.block1..b
00000030   20 21 21 21 21 22 22 22  22 23 23 23 23 24 24 24   lock2.block2.blo
00000040   24 25 25 25 25 26 26 26  26 00 00 00 00 30 30 30   ck2.block2.blo.s
00000050   30 31 31 31 31 32 32 32  32 33 33 33 33 34 34 34   top.stop.stop.st
00000060   34 35 35 35 35 36 36 36  36 00 00 00 00            top.stop.sto
";

        let original : Vec<u8> = hexdump::decode(dump);
        
        let mut locators : Vec<Sha256> = Vec::new();
        let loc1 = Sha256 {
            hash: [
                0x10, 0x10, 0x10, 0x10, 0x11, 0x11, 0x11, 0x11, 
                0x12, 0x12, 0x12, 0x12, 0x13, 0x13, 0x13, 0x13, 
                0x14, 0x14, 0x14, 0x14, 0x15, 0x15, 0x15, 0x15, 
                0x16, 0x16, 0x16, 0x16, 0x00, 0x00, 0x00, 0x00
            ]
        };

        let loc2 = Sha256 {
            hash: [
                0x20, 0x20, 0x20, 0x20, 0x21, 0x21, 0x21, 0x21, 
                0x22, 0x22, 0x22, 0x22, 0x23, 0x23, 0x23, 0x23, 
                0x24, 0x24, 0x24, 0x24, 0x25, 0x25, 0x25, 0x25, 
                0x26, 0x26, 0x26, 0x26, 0x00, 0x00, 0x00, 0x00
            ]
        };
        locators.push(loc1);
        locators.push(loc2);

        let stop = Sha256 {
            hash: [
                0x30, 0x30, 0x30, 0x30, 0x31, 0x31, 0x31, 0x31, 
                0x32, 0x32, 0x32, 0x32, 0x33, 0x33, 0x33, 0x33, 
                0x34, 0x34, 0x34, 0x34, 0x35, 0x35, 0x35, 0x35, 
                0x36, 0x36, 0x36, 0x36, 0x00, 0x00, 0x00, 0x00
            ]
        };

        let message = GetHeaders {
            version: 70001,
            locators: locators,
            stop: stop
        };

        let mut result : Vec<u8> = Vec::new();
        let encoded = message.encode(&mut result);
        assert!(encoded.is_ok());

        assert_eq!(original, result);
    }

}