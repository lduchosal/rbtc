use crate::bo::outpoint::OutPoint;
use crate::bo::transaction::Transaction;
use crate::bo::txin::TxIn;
use crate::bo::txout::TxOut;
use crate::business::block;
use crate::business::block::ParseError;
use crate::hexdump;

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
00000000   73 36 50 7d 78 5b 17 a2  c1 15 e4 27 a3 2f ac      ...............
";

    let hex: Vec<u8> = hexdump::parse(dump);

    assert_eq!(hex.len(), 639);

    let result = block::parse(&hex);
    assert!(result.is_err());

    let err = result.err().unwrap();

    assert_eq!(err, ParseError::TransactionLockTime);
}
