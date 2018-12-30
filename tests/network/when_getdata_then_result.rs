
use rbtc::message::getheaders;
use rbtc::utils::hexdump;

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
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00      top.stop.stop.
";

    let hex : Vec<u8> = hexdump::parse(dump);

    assert_eq!(hex.len(), 81);

    let result = getheaders::parse(&hex);
    assert!(result.is_ok());

    let b = result.ok().unwrap();

    assert_eq!(b.version, 0x00000000, "b.version");
    assert_eq!(b.previous, [0; 32], "b.previous");
    assert_eq!(b.merkleroot, [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    ], "b.merkleroot");
    assert_eq!(b.time, 0000000000, "b.time");
    assert_eq!(b.bits, 0x00000000, "b.bits");
    assert_eq!(b.nonce, 0x00000000, "b.nonce");
    assert_eq!(b.transactions.len(), 0, "b.transactions.len");

}