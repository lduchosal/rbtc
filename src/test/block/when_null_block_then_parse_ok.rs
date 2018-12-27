
use crate::bo::transaction::Transaction;
use crate::business::block;
use crate::business::error::ParseError;
use crate::hexdump;

#[test]
fn test() {

    let dump = "
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00                                                 .
";


    let hex : Vec<u8> = hexdump::parse(dump);

    assert_eq!(hex.len(), 81);

    let result = block::parse(&hex);
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