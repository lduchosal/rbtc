use crate::utils::hexdump;
use crate::encode::encode::{Encodable, Decodable};
use crate::network::message::Message;

use std::io::Cursor;

#[test]
fn test() {

    let dump = "
00000000   F9 BE B4 D9 76 65 72 73  69 6F 6E 00 00 00 00 00   ....version.....
00000010   68 00 00 00 E8 F3 FD 18  7F 11 01 00 0D 04 00 00   h...............
00000020   00 00 00 00 A6 85 32 5C  00 00 00 00 00 00 00 00   ......2.........
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000040   00 00 00 00 00 00 0D 04  00 00 00 00 00 00 00 00   ................
00000050   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000060   A2 57 8C BA 40 A6 CC B8  12 2F 53 61 74 6F 73 68   .W..@..../Satosh
00000070   69 3A 30 2E 31 37 2E 30  2E 31 2F 02 38 08 00 01   i:0.17.0.1/.8...
00000080   F9 BE B4 D9 76 65 72 61  63 6B 00 00 00 00 00 00   ....verack......
00000090   00 00 00 00 5D F6 E0 E2  F9 BE B4 D9 61 6C 65 72   ....].......aler
000000a0   74 00 00 00 00 00 00 00  A8 00 00 00 1B F9 AA EA   t...............
000000b0   60 01 00 00 00 00 00 00  00 00 00 00 00 FF FF FF   `...............
000000c0   7F 00 00 00 00 FF FF FF  7F FE FF FF 7F 01 FF FF   ................
000000d0   FF 7F 00 00 00 00 FF FF  FF 7F 00 FF FF FF 7F 00   ................
000000e0   2F 55 52 47 45 4E 54 3A  20 41 6C 65 72 74 20 6B   /URGENT: Alert k
000000f0   65 79 20 63 6F 6D 70 72  6F 6D 69 73 65 64 2C 20   ey compromised,
00000100   75 70 67 72 61 64 65 20  72 65 71 75 69 72 65 64   upgrade required
00000110   00 46 30 44 02 20 65 3F  EB D6 41 0F 47 0F 6B AE   .F0D. e?..A.G.k.
00000120   11 CA D1 9C 48 41 3B EC  B1 AC 2C 17 F9 08 FD 0F   ....HA;...,.....
00000130   D5 3B DC 3A BD 52 02 20  6D 0E 9C 96 FE 88 D4 A0   .;.:.R. m.......
00000140   F0 1E D9 DE DA E2 B6 F9  E0 0D A9 4C AD 0F EC AA   ...........L....
00000150   E6 6E CF 68 9B F7 1B 50                            .n.h...P
    ";

    let data : Vec<u8> = hexdump::decode(dump);
    assert_eq!(data.len(), 0x158);

    let mut r = Cursor::new(&data);
    let decoded = <Vec<Message>>::decode(&mut r);
    
    println!("{:?}", decoded);

    assert!(decoded.is_ok());

    let messages = decoded.unwrap();
    for message in messages {
        println!("{:?}", message);
    }

}