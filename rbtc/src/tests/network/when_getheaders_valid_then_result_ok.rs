
use crate::utils::sha256::Sha256;
use crate::network::getheaders::GetHeaders;
use crate::network::getheaders;
use crate::utils::hexdump;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};


#[test]
fn test() {

    let dump = "
00000000   f9 be b4 d9 67 65 74 68  65 61 64 65 72 73 00 00   main.getheaders.
00000000   01 02 03 04 01 02 03 04  71 11 01 00 02 d3 9f 60   siz.has.ver.c.bl
00000000   8a 77 75 b5 37 72 98 84  d4 e6 63 3b b2 10 5e 55   ock1.block1.bloc
00000000   a1 6a 14 d3 1b 00 00 00  00 00 00 00 00 00 5c 3e   block1.block1..b
00000000   64 03 d4 08 37 11 0a 2e  8a fb 60 2b 1c 01 71 4b   lock2.block2.blo
00000000   da 7c e2 3b ea 0a 00 00  00 00 00 00 00 00 00 00   ck2.block2.blo.s
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   top.stop.stop.st
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00            top.stop.sto
";


    let data : Vec<u8> = hexdump::decode(dump);
    let mut c = Cursor::new(data.as_ref());
    c.set_position(24); // move beyond network protocol headers
    let result = getheaders::decode(&mut c);

    assert_eq!(c.position() as usize, data.len());
    assert!(result.is_ok());

    let message : GetHeaders = result.unwrap();

    assert_eq!(message.version, 70001);
    assert_eq!(message.locators.len(), 0x02);

    let locator1 : &Sha256 = message.locators.get(0).unwrap();
    assert_eq!(locator1.hash, [
        0xd3, 0x9f, 0x60, 0x8a, 0x77, 0x75, 0xb5, 0x37, 0x72, 0x98, 0x84, 0xd4, 0xe6, 0x63, 0x3b, 0xb2, 
        0x10, 0x5e, 0x55, 0xa1, 0x6a, 0x14, 0xd3, 0x1b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ]); 

    let locator2 : &Sha256 = message.locators.get(1).unwrap();
    assert_eq!(locator2.hash, [
        0x00, 0x5c, 0x3e, 0x64, 0x03, 0xd4, 0x08, 0x37, 0x11, 0x0a, 0x2e, 0x8a, 0xfb, 0x60, 0x2b, 0x1c, 
        0x01, 0x71, 0x4b, 0xda, 0x7c, 0xe2, 0x3b, 0xea, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ]); 

    let stop : Sha256 = message.stop;
    assert_eq!(stop.hash, [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ]); 

}