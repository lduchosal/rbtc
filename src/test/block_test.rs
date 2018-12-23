use crate::bo::outpoint::OutPoint;
use crate::bo::block::Block;
use crate::bo::transaction::Transaction;
use crate::bo::txin::TxIn;
use crate::bo::txout::TxOut;

use crate::business::block;
use crate::hexdump;

#[cfg(test)]
#[test]
fn when_genesis_block_then_version1() {
    let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000020   00 00 00 00 3B A3 ED FD  7A 7B 12 B2 7A C7 2C 3E   ....;£íýz{.²zÇ,>
00000030   67 76 8F 61 7F C8 1B C3  88 8A 51 32 3A 9F B8 AA   gv.a.È.ÃˆŠQ2:Ÿ¸ª
00000040   4B 1E 5E 4A 29 AB 5F 49  FF FF 00 1D 1D AC 2B 7C   K.^J)«_Iÿÿ...¬+|
00000050   01 01 00 00 00 01 00 00  00 00 00 00 00 00 00 00   ................
00000060   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000070   00 00 00 00 00 00 FF FF  FF FF 4D 04 FF FF 00 1D   ......ÿÿÿÿM.ÿÿ..
00000080   01 04 45 54 68 65 20 54  69 6D 65 73 20 30 33 2F   ..EThe Times 03/
00000090   4A 61 6E 2F 32 30 30 39  20 43 68 61 6E 63 65 6C   Jan/2009 Chancel
000000A0   6C 6F 72 20 6F 6E 20 62  72 69 6E 6B 20 6F 66 20   lor on brink of 
000000B0   73 65 63 6F 6E 64 20 62  61 69 6C 6F 75 74 20 66   second bailout f
000000C0   6F 72 20 62 61 6E 6B 73  FF FF FF FF 01 00 F2 05   or banksÿÿÿÿ..ò.
000000D0   2A 01 00 00 00 43 41 04  67 8A FD B0 FE 55 48 27   *....CA.gŠý°þUH'
000000E0   19 67 F1 A6 71 30 B7 10  5C D6 A8 28 E0 39 09 A6   .gñ¦q0·.\\Ö¨(à9.¦
000000F0   79 62 E0 EA 1F 61 DE B6  49 F6 BC 3F 4C EF 38 C4   ybàê.aÞ¶Iö¼?Lï8Ä
00000100   F3 55 04 E5 1E C1 12 DE  5C 38 4D F7 BA 0B 8D 57   óU.å.Á.Þ\\8M÷º..W
00000110   8A 4C 70 2B 6B F1 1D 5F  AC 00 00 00 00            ŠLp+kñ._¬....
";


    let hex : Vec<u8> = hexdump::parse(dump);

    assert_eq!(hex.len(), 285);

    let b : Block = block::parse(&hex)
        .ok()
        .unwrap();

    assert_eq!(b.version, 0x00000001, "b.version");
    assert_eq!(b.previous, [0; 32], "b.previous");
    assert_eq!(b.merkleroot, [
        0x3B, 0xA3, 0xED, 0xFD, 0x7A, 0x7B, 0x12, 0xB2, 
        0x7A, 0xC7, 0x2C, 0x3E, 0x67, 0x76, 0x8F, 0x61, 
        0x7F, 0xC8, 0x1B, 0xC3, 0x88, 0x8A, 0x51, 0x32, 
        0x3A, 0x9F, 0xB8, 0xAA, 0x4B, 0x1E, 0x5E, 0x4A
    ], "b.merkleroot");
    assert_eq!(b.time, 1231006505, "b.time"); // Unix Epoch	1231006505 - Time (UTC)   Sat Jan 03 18:15:05 2009 UTC
    assert_eq!(b.bits, 0x1D00FFFF, "b.bits");
    assert_eq!(b.nonce, 0x7C2BAC1D, "b.nonce");
    assert_eq!(b.transactions.len(), 1, "b.transactions.len");

    let t : &Transaction = b.transactions.get(0).unwrap();

    assert_eq!(t.inputs.len(), 1, "t.inputs.len");
    assert_eq!(t.outputs.len(), 1, "t.outputs.len");
    assert_eq!(t.version, 0x00000001, "t.version");
    assert_eq!(t.locktime, 0x00000000, "t.locktime");

    let i : &TxIn = t.inputs.get(0).unwrap();

    assert_eq!(i.sequence, 0xFFFFFFFF, "i.sequence");
    assert_eq!(i.signature.content.len(), 0x4D, "i.signature.content.len");
    assert_eq!(i.signature.content, vec![
        0x04, 0xFF, 0xFF, 0x00, 0x1D, 0x01, 0x04, 0x45, 
        0x54, 0x68, 0x65, 0x20, 0x54, 0x69, 0x6D, 0x65, // The Times 03/
        0x73, 0x20, 0x30, 0x33, 0x2F, 0x4A, 0x61, 0x6E, // Jan/2009 Chancel
        0x2F, 0x32, 0x30, 0x30, 0x39, 0x20, 0x43, 0x68, // lor on brink of 
        0x61, 0x6E, 0x63, 0x65, 0x6C, 0x6C, 0x6F, 0x72, // second bailout f
        0x20, 0x6F, 0x6E, 0x20, 0x62, 0x72, 0x69, 0x6E, // or banks
        0x6B, 0x20, 0x6F, 0x66, 0x20, 0x73, 0x65, 0x63,
        0x6F, 0x6E, 0x64, 0x20, 0x62, 0x61, 0x69, 0x6C,
        0x6F, 0x75, 0x74, 0x20, 0x66, 0x6F, 0x72, 0x20,
        0x62, 0x61, 0x6E, 0x6B ,0x73
    ], "i.signature.content");

    assert_eq!(i.witness.len(), 0, "i.witness.len");

    let p : &OutPoint = &i.previous;

    assert_eq!(p.transaction_hash, [0; 32], "p.transaction_hash");
    assert_eq!(p.index, 0xFFFFFFFF, "p.index");

    let o : &TxOut = t.outputs.get(0).unwrap();

    assert_eq!(o.script_pubkey.content.len(), 0x43, "o.script_pubkey.content.len");
    assert_eq!(o.script_pubkey.content, vec![
        0x41, 0x04, 0x67, 0x8A, 0xFD, 0xB0, 0xFE, 0x55,
        0x48, 0x27, 0x19, 0x67, 0xF1, 0xA6, 0x71, 0x30,
        0xB7, 0x10, 0x5C, 0xD6, 0xA8, 0x28, 0xE0, 0x39,
        0x09, 0xA6, 0x79, 0x62, 0xE0, 0xEA, 0x1F, 0x61,
        0xDE, 0xB6, 0x49, 0xF6, 0xBC, 0x3F, 0x4C, 0xEF,
        0x38, 0xC4, 0xF3, 0x55, 0x04, 0xE5, 0x1E, 0xC1, 
        0x12, 0xDE, 0x5C, 0x38, 0x4D, 0xF7, 0xBA, 0x0B, 
        0x8D, 0x57, 0x8A, 0x4C, 0x70, 0x2B, 0x6B, 0xF1, 
        0x1D, 0x5F, 0xAC
    ], "o.script_pubkey.content");
    assert_eq!(o.amount, 5000000000, "0.amount"); // 50BTC

}