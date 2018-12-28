use rbtc::bo::outpoint::OutPoint;
use rbtc::bo::transaction::Transaction;
use rbtc::bo::txin::TxIn;
use rbtc::bo::txout::TxOut;
use rbtc::business::block;
use rbtc::business::error::ParseError;
use rbtc::hexdump;

#[test]
fn test() {
    let dump = "
00000000   01 00 00 00 4d dc cd 54  9d 28 f3 85 ab 45 7e 98   ................
00000000   d1 b1 1c e8 0b fe a2 c5  ab 93 01 5a de 49 73 e4   ................
00000000   00 00 00 00 bf 44 73 e5  37 94 be ae 34 e6 4f cc   ................
00000000   c4 71 da ce 6a e5 44 18  08 16 f8 95 91 89 4e 0f   ................
00000000   41 7a 91 4c d7 4d 6e 49  ff ff 00 1d 32 3b 3a 7b   ................
00000000   02 01 00 00 00 01 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 ff ff  ff ff 08 04 ff ff 00 1d   ................
00000000   02 6e 04 ff ff ff ff 01  00 f2 05 2a 01 00 00 00   ................
00000000   43 41 04 46 ef 01 02 d1  ec 52 40 f0 d0 61 a4 24   ................
00000000   6c 1b de f6 3f c3 db ab  77 33 05 2f bb f0 ec d8   ................
00000000   f4 1f c2 6b f0 49 eb b4  f9 52 7f 37 42 80 25 9e   ................
00000000   7c fa 99 c4 8b 0e 3f 39  c5 13 47 a1 9a 58 19 65   ................
00000000   15 03 a5 ac 00 00 00 00  01 00 00 00 03 21 f7 5f   ................
00000000   31 39 a0 13 f5 0f 31 5b  23 b0 c9 a2 b6 ea c3 1e   ................
00000000   2b ec 98 e5 89 1c 92 46  64 88 99 42 26 00 00 00   ................
00000000   00 49 48 30 45 02 21 00  cb 2c 6b 34 6a 97 8a b8   ................
00000000   c6 1b 18 b5 e9 39 77 55  cb d1 7d 6e b2 fe 00 83   ................
00000000   ef 32 e0 67 fa 6c 78 5a  02 20 6c e4 4e 61 3f 31   ................
00000000   d9 a6 b0 51 7e 46 f3 db  15 76 e9 81 2c c9 8d 15   ................
00000000   9b fd af 75 9a 50 14 08  1b 5c 01 ff ff ff ff 79   ................
00000000   cd a0 94 59 03 62 7c 3d  a1 f8 5f c9 5d 0b 8e e3   ................
00000000   e7 6a e0 cf dc 9a 65 d0  97 44 b1 f8 fc 85 43 00   ................
00000000   00 00 00 49 48 30 45 02  20 47 95 7c dd 95 7c fd   ................
00000000   0b ec d6 42 f6 b8 4d 82  f4 9b 6c b4 c5 1a 91 f4   ................
00000000   92 46 90 8a f7 c3 cf df  4a 02 21 00 e9 6b 46 62   ................
00000000   1f 1b ff cf 5e a5 98 2f  88 ce f6 51 e9 35 4f 57   ................
00000000   91 60 23 69 bf 5a 82 a6  cd 61 a6 25 01 ff ff ff   ................
00000000   ff fe 09 f5 fe 3f fb f5  ee 97 a5 4e b5 e5 06 9e   ................
00000000   9d a6 b4 85 6e e8 6f c5  29 38 c2 f9 79 b0 f3 8e   ................
00000000   82 00 00 00 00 48 47 30  44 02 20 41 65 be 9a 4c   ................
00000000   ba b8 04 9e 1a f9 72 3b  96 19 9b fd 3e 85 f4 4c   ................
00000000   6b 4c 01 77 e3 96 26 86  b2 60 73 02 20 28 f6 38   ................
00000000   da 23 fc 00 37 60 86 1a  d4 81 ea d4 09 93 12 c6   ................
00000000   00 30 d4 cb 57 82 0c e4  d3 38 12 a5 ce 01 ff ff   ................
00000000   ff ff 01 00 9d 96 6b 01  00 00 00 43 41 04 ea 1f   ................
00000000   ef f8 61 b5 1f e3 f5 f8  a3 b1 2d 0f 47 12 db 80   ................
00000000   e9 19 54 8a 80 83 9f c4  7c 6a 21 e6 6d 95 7e 9c   ................
00000000   5d 8c d1 08 c7 a2 d2 32  4b ad 71 f9 90 4a c0 ae   ................
00000000   73 36 50 7d 78 5b 17 a2  c1 15 e4 27 a3 2f ac 00   ................
00000000   00 00 00                                           ...
";

    let hex: Vec<u8> = hexdump::parse(dump);

    assert_eq!(hex.len(), 643);

    let result = block::parse(&hex);
    assert!(result.is_ok());

    let b = result.ok().unwrap();

    assert_eq!(b.version, 0x00000001, "b.version");
    assert_eq!(
        b.previous,
        [
            0x4d, 0xdc, 0xcd, 0x54, 0x9d, 0x28, 0xf3, 0x85, 0xab, 0x45, 0x7e, 0x98, 0xd1, 0xb1,
            0x1c, 0xe8, 0x0b, 0xfe, 0xa2, 0xc5, 0xab, 0x93, 0x01, 0x5a, 0xde, 0x49, 0x73, 0xe4,
            0x00, 0x00, 0x00, 0x00,
        ],
        "b.previous"
    );

    assert_eq!(
        b.merkleroot,
        [
            0xbf, 0x44, 0x73, 0xe5, 0x37, 0x94, 0xbe, 0xae, 0x34, 0xe6, 0x4f, 0xcc, 0xc4, 0x71,
            0xda, 0xce, 0x6a, 0xe5, 0x44, 0x18, 0x08, 0x16, 0xf8, 0x95, 0x91, 0x89, 0x4e, 0x0f,
            0x41, 0x7a, 0x91, 0x4c,
        ],
        "b.merkleroot"
    );

    assert_eq!(b.time, 1231965655, "b.time");
    assert_eq!(b.bits, 486604799, "b.bits");
    assert_eq!(b.nonce, 2067413810, "b.nonce");
    assert_eq!(b.transactions.len(), 2, "b.transactions.len");

    let t1: &Transaction = b.transactions.get(0).unwrap();

    assert_eq!(t1.inputs.len(), 1, "t1.inputs.len");
    assert_eq!(t1.outputs.len(), 1, "t1.outputs.len");
    assert_eq!(t1.version, 0x00000001, "t1.version");
    assert_eq!(t1.locktime, 0x00000000, "t1.locktime");

    let t1i: &TxIn = t1.inputs.get(0).unwrap();

    assert_eq!(t1i.sequence, 0xFFFFFFFF, "t1i.sequence");
    assert_eq!(
        t1i.signature.content.len(),
        0x8,
        "t1i.signature.content.len"
    );
    assert_eq!(
        t1i.signature.content,
        vec![
        0x04, 0xFF, 0xFF, 0x00, 0x1D, 0x02, 0x6E, 0x04, //
    ],
        "t1i.signature.content"
    );

    assert!(t1.witness.is_none(), "t1.witness");

    let t1ip: &OutPoint = &t1i.previous;

    assert_eq!(t1ip.transaction_hash, [0; 32], "t1ip.transaction_hash");
    assert_eq!(t1ip.index, 4294967295, "t1ip.index");

    let t1o: &TxOut = t1.outputs.get(0).unwrap();

    assert_eq!(t1o.amount, 5000000000, "t1o.amount"); // 50BTC
    assert_eq!(
        t1o.script_pubkey.content.len(),
        0x43,
        "t1o.script_pubkey.content.len"
    );

    assert_eq!(
        t1o.script_pubkey.content,
        vec![
            0x41, 0x04, 0x46, 0xef, 0x01, 0x02, 0xd1, 0xec, 0x52, 0x40, 0xf0, 0xd0, 0x61, 0xa4,
            0x24, 0x6c, 0x1b, 0xde, 0xf6, 0x3f, 0xc3, 0xdb, 0xab, 0x77, 0x33, 0x05, 0x2f, 0xbb,
            0xf0, 0xec, 0xd8, 0xf4, 0x1f, 0xc2, 0x6b, 0xf0, 0x49, 0xeb, 0xb4, 0xf9, 0x52, 0x7f,
            0x37, 0x42, 0x80, 0x25, 0x9e, 0x7c, 0xfa, 0x99, 0xc4, 0x8b, 0x0e, 0x3f, 0x39, 0xc5,
            0x13, 0x47, 0xa1, 0x9a, 0x58, 0x19, 0x65, 0x15, 0x03, 0xa5, 0xac
        ],
        "t1o.script_pubkey.content"
    );

    let t2: &Transaction = b.transactions.get(1).unwrap();

    assert_eq!(t2.inputs.len(), 3, "t2.inputs.len");
    assert_eq!(t2.outputs.len(), 1, "t2.outputs.len");
    assert_eq!(t2.version, 0x00000001, "t2.version");
    assert_eq!(t2.locktime, 0x00000000, "t2.locktime");

    let t2i1: &TxIn = t2.inputs.get(0).unwrap();

    assert_eq!(t2i1.sequence, 0xFFFFFFFF, "t2i1.sequence");
    assert_eq!(t2i1.signature.content.len(), 0x49, "t2i1.signature.content.len");
    assert_eq!(t2i1.signature.content, vec![
        0x48, 0x30, 0x45, 0x02, 0x21, 0x00, 0xcb, 0x2c, 0x6b, 0x34, 0x6a, 0x97, 0x8a, 0xb8, 0xc6, 0x1b, 
        0x18, 0xb5, 0xe9, 0x39, 0x77, 0x55, 0xcb, 0xd1, 0x7d, 0x6e, 0xb2, 0xfe, 0x00, 0x83, 0xef, 0x32, 
        0xe0, 0x67, 0xfa, 0x6c, 0x78, 0x5a, 0x02, 0x20, 0x6c, 0xe4, 0x4e, 0x61, 0x3f, 0x31, 0xd9, 0xa6, 
        0xb0, 0x51, 0x7e, 0x46, 0xf3, 0xdb, 0x15, 0x76, 0xe9, 0x81, 0x2c, 0xc9, 0x8d, 0x15, 0x9b, 0xfd, 
        0xaf, 0x75, 0x9a, 0x50, 0x14, 0x08, 0x1b, 0x5c, 0x01, 
        ], "t2i1.signature.content"
    );

    assert!(t2.witness.is_none(), "t2.witness");

    let t2i1p: &OutPoint = &t2i1.previous;

    assert_eq!(t2i1p.transaction_hash, [
        0x21, 0xf7, 0x5f, 0x31, 0x39, 0xa0, 0x13, 0xf5, 0x0f, 0x31, 0x5b, 0x23, 0xb0, 0xc9, 0xa2, 0xb6, 
        0xea, 0xc3, 0x1e, 0x2b, 0xec, 0x98, 0xe5, 0x89, 0x1c, 0x92, 0x46, 0x64, 0x88, 0x99, 0x42, 0x26
        ], "t2i1p.transaction_hash");
    assert_eq!(t2i1p.index, 0x00, "t2i1p.index");


    let t2i2: &TxIn = t2.inputs.get(1).unwrap();

    assert_eq!(t2i2.sequence, 0xFFFFFFFF, "t2i2.sequence");
    assert_eq!(t2i2.signature.content.len(), 0x49, "t2i2.signature.content.len");
    assert_eq!(t2i2.signature.content, vec![
        0x48, 0x30, 0x45, 0x02, 0x20, 0x47, 0x95, 0x7c, 0xdd, 0x95, 0x7c, 0xfd, 0x0b, 0xec, 0xd6, 0x42, 
        0xf6, 0xb8, 0x4d, 0x82, 0xf4, 0x9b, 0x6c, 0xb4, 0xc5, 0x1a, 0x91, 0xf4, 0x92, 0x46, 0x90, 0x8a, 
        0xf7, 0xc3, 0xcf, 0xdf, 0x4a, 0x02, 0x21, 0x00, 0xe9, 0x6b, 0x46, 0x62, 0x1f, 0x1b, 0xff, 0xcf, 
        0x5e, 0xa5, 0x98, 0x2f, 0x88, 0xce, 0xf6, 0x51, 0xe9, 0x35, 0x4f, 0x57, 0x91, 0x60, 0x23, 0x69, 
        0xbf, 0x5a, 0x82, 0xa6, 0xcd, 0x61, 0xa6, 0x25, 0x01,
        ], "t2i2.signature.content"
    );

    let t2i2p: &OutPoint = &t2i2.previous;

    assert_eq!(t2i2p.transaction_hash, [
        0x79, 0xcd, 0xa0, 0x94, 0x59, 0x03, 0x62, 0x7c, 0x3d, 0xa1, 0xf8, 0x5f, 0xc9, 0x5d, 0x0b, 0x8e, 
        0xe3, 0xe7, 0x6a, 0xe0, 0xcf, 0xdc, 0x9a, 0x65, 0xd0, 0x97, 0x44, 0xb1, 0xf8, 0xfc, 0x85, 0x43, 
        ], "t2i2p.transaction_hash");
    assert_eq!(t2i2p.index, 0x00, "t2i2p.index");

    let t2i3: &TxIn = t2.inputs.get(2).unwrap();

    assert_eq!(t2i3.sequence, 0xFFFFFFFF, "t2i3.sequence");
    assert_eq!(t2i3.signature.content.len(), 0x48, "t2i3.signature.content.len");
    assert_eq!(t2i3.signature.content, vec![
        0x47, 0x30, 0x44, 0x02, 0x20, 0x41, 0x65, 0xbe, 0x9a, 0x4c, 0xba, 0xb8, 0x04, 0x9e, 0x1a, 0xf9, 
        0x72, 0x3b, 0x96, 0x19, 0x9b, 0xfd, 0x3e, 0x85, 0xf4, 0x4c, 0x6b, 0x4c, 0x01, 0x77, 0xe3, 0x96, 
        0x26, 0x86, 0xb2, 0x60, 0x73, 0x02, 0x20, 0x28, 0xf6, 0x38, 0xda, 0x23, 0xfc, 0x00, 0x37, 0x60, 
        0x86, 0x1a, 0xd4, 0x81, 0xea, 0xd4, 0x09, 0x93, 0x12, 0xc6, 0x00, 0x30, 0xd4, 0xcb, 0x57, 0x82, 
        0x0c, 0xe4, 0xd3, 0x38, 0x12, 0xa5, 0xce, 0x01
        ], "t2i3.signature.content"
    );

    let t2i3p: &OutPoint = &t2i3.previous;

    assert_eq!(t2i3p.transaction_hash, [
        0xfe, 0x09, 0xf5, 0xfe, 0x3f, 0xfb, 0xf5, 0xee, 0x97, 0xa5, 0x4e, 0xb5, 0xe5, 0x06, 0x9e, 0x9d, 
        0xa6, 0xb4, 0x85, 0x6e, 0xe8, 0x6f, 0xc5, 0x29, 0x38, 0xc2, 0xf9, 0x79, 0xb0, 0xf3, 0x8e, 0x82
        ], "t2i3p.transaction_hash");
    assert_eq!(t2i3p.index, 0x00, "t2i3p.index");



    let t2o: &TxOut = t2.outputs.get(0).unwrap();

    assert_eq!(t2o.amount, 6100000000, "t2o.amount"); // 50BTC
    assert_eq!(
        t2o.script_pubkey.content.len(),
        0x43,
        "t2o.script_pubkey.content.len"
    );

    assert_eq!(
        t2o.script_pubkey.content,
        vec![
            0x41, 0x04, 0xea, 0x1f, 0xef, 0xf8, 0x61, 0xb5, 0x1f, 0xe3, 0xf5, 0xf8, 0xa3, 0xb1, 0x2d, 0x0f, 
            0x47, 0x12, 0xdb, 0x80, 0xe9, 0x19, 0x54, 0x8a, 0x80, 0x83, 0x9f, 0xc4, 0x7c, 0x6a, 0x21, 0xe6,
            0x6d, 0x95, 0x7e, 0x9c, 0x5d, 0x8c, 0xd1, 0x08, 0xc7, 0xa2, 0xd2, 0x32, 0x4b, 0xad, 0x71, 0xf9, 
            0x90, 0x4a, 0xc0, 0xae, 0x73, 0x36, 0x50, 0x7d, 0x78, 0x5b, 0x17, 0xa2, 0xc1, 0x15, 0xe4, 0x27, 
            0xa3, 0x2f, 0xac
        ],
        "t2o.script_pubkey.content"
    );

}
