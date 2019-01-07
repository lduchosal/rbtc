use crate::block::witness::Witness;
use crate::block::block::Block;
use crate::block::outpoint::OutPoint;
use crate::block::transaction::Transaction;
use crate::block::txin::TxIn;
use crate::block::txout::TxOut;

use crate::block::block;

use crate::utils::hexdump;

#[test]
fn test() {
     
    let dump = "
00000000   00 00 00 20 2a a2 f2 ca  79 4c cb d4 0c 16 e2 f3   ................
00000010   33 3f 6b 8b 68 3f 9e 71  79 b2 c4 d7 49 06 00 00   ................
00000020   00 00 00 00 10 bc 26 e7  0a 2f 67 2a d4 20 a6 15   ................
00000030   3d d0 c2 8b 40 a6 00 2c  55 53 1b fc 99 bf 89 94   ................
00000040   a8 e8 f6 7e 55 03 bd 57  50 d4 06 1a 4e d9 0a 70   ................
00000050   0f 01 00 00 00 00 01 01  00 00 00 00 00 00 00 00   ................
00000060   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000070   00 00 00 00 00 00 00 00  ff ff ff ff 36 03 da 1b   ................
000000A0   0e 00 04 55 03 bd 57 04  c7 dd 8a 0d 0c ed 13 bb   ................
000000B0   57 85 01 08 00 00 00 00  00 0a 63 6b 70 6f 6f 6c   ................
000000C0   12 2f 4e 69 6e 6a 61 50  6f 6f 6c 2f 53 45 47 57   ................
000000D0   49 54 2f ff ff ff ff 02  b4 e5 a2 12 00 00 00 00   ................
000000E0   19 76 a9 14 87 6f bb 82  ec 05 ca a6 af 7a 3b 5e   ................
000000F0   5a 98 3a ae 6c 6c c6 d6  88 ac 00 00 00 00 00 00   ................
00000000   00 00 26 6a 24 aa 21 a9  ed f9 1c 46 b4 9e b8 a2   ................
00000010   90 89 98 0f 02 ee 6b 57  e7 d6 3d 33 b1 8b 4f dd   ................
00000020   ac 2b cd 7d b2 a3 98 37  04 01 20 00 00 00 00 00   ................
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000040   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 01   ................
00000050   00 00 00 01 7e 4f 81 17  53 32 a7 33 e2 6d 4b a4   ................
00000060   e2 9f 53 f6 7b 7a 5d 7c  2a de bb 27 6e 44 7c a7   ................
00000070   1d 13 0b 55 00 00 00 00  6b 48 30 45 02 21 00 ca   ................
00000080   c8 09 cd 1a 3d 9a d5 d5  e3 1a 84 e2 e1 d8 ec 55   ................
00000090   42 84 1e 4d 14 c6 b5 2e  8b 38 cb e1 ff 17 28 02   ................
000000A0   20 64 47 0b 7f b0 c2 ef  ec cb 2e 84 bf a3 6e c5   ................
000000B0   f9 e4 34 c8 4b 11 01 c0  0f 7e e3 2f 72 63 71 b7   ................
000000C0   41 01 21 02 0e 62 28 07  98 b6 b8 c3 7f 06 8d f0   ................
000000D0   91 5b 08 65 b6 3f ab c4  01 c2 45 7c bc 3e f9 68   ................
000000E0   87 dd 36 47 ff ff ff ff  02 ca 2f 78 0c 00 00 00   ................
000000F0   00 19 76 a9 14 c6 b5 54  5b 35 92 cb 47 7d 70 98   ................
00000000   96 fa 70 55 92 c9 b6 11  3a 88 ac 66 3b 2a 06 00   ................
00000010   00 00 00 19 76 a9 14 e7  c1 34 5f c8 f8 7c 68 17   ................
00000020   0b 3a a7 98 a9 56 c2 fe  6a 9e ff 88 ac 00 00 00   ................
00000030   00 01 00 00 00 01 1e 99  f5 a7 85 e6 77 e0 17 d3   ................
00000040   6b 50 aa 4f d1 00 10 ff  d0 39 f3 8f 42 f4 47 ca   ................
00000050   88 95 25 0e 12 1f 01 00  00 00 d9 00 47 30 44 02   ................
00000060   20 0d 3d 29 6a d6 41 a2  81 dd 5c 0d 68 b9 ab 0d   ................
00000070   1a d5 f7 05 2b ec 14 8c  1f b8 1f b1 ba 69 18 1e   ................
00000080   c5 02 20 1a 37 2b b1 6f  b8 e0 54 ee 9b ef 41 e3   ................
00000090   00 d2 92 15 38 30 f8 41  a4 db 0a b7 f7 40 7f 65   ................
000000A0   81 b9 bc 01 47 30 44 02  20 02 58 4f 31 3a e9 90   ................
000000B0   23 6b 6b eb b8 2f bb b0  06 a2 b0 2a 44 8d d5 c9   ................
000000C0   34 34 42 89 91 ea e9 60  d6 02 20 49 1d 67 d2 66   ................
000000D0   0c 4d de 19 02 5c f8 6e  51 64 a5 59 e2 c7 9c 3b   ................
000000E0   98 b4 0e 14 6f ab 97 4a  cd 24 69 01 47 52 21 02   ................
000000F0   63 21 78 d0 46 67 3c 97  29 d8 28 cf ee 38 8e 12   ................
00000000   1f 49 77 07 f8 10 c1 31  e0 d3 fc 0f e0 bd 66 d6   ................
00000000   21 03 a0 95 1e c7 d3 a9  da 9d e1 71 61 70 26 44   ................
00000000   2f cd 30 f3 4d 66 10 0f  ab 53 98 53 b4 3f 50 87   ................
00000000   87 d4 52 ae ff ff ff ff  02 40 42 0f 00 00 00 00   ................
00000000   00 17 a9 14 0f fd cf 96  70 04 55 07 42 92 a8 21   ................
00000000   c7 49 22 e8 65 29 93 99  87 88 99 7b c6 00 00 00   ................
00000000   00 17 a9 14 8c e5 40 8c  fe ad db 7c cb 25 45 de   ................
00000000   d4 1e f4 78 10 94 54 84  87 00 00 00 00 01 00 00   ................
00000000   00 01 13 10 0b 09 e6 a7  8d 63 ec 48 50 65 4a b0   ................
00000000   f6 88 06 de 29 71 0b 09  17 2e dd fe f7 30 65 2b   ................
00000000   15 55 01 00 00 00 da 00  47 30 44 02 20 15 38 94   ................
00000000   08 e3 44 6a 3f 36 a0 50  60 e0 e4 a3 c8 b9 2f f3   ................
00000000   90 1b a2 51 1a a9 44 ec  91 a5 37 a1 cb 02 20 45   ................
00000000   a3 3b 6e c4 76 05 b1 71  8e d2 e7 53 26 3e 54 91   ................
00000000   8e db f6 12 65 08 ff 03  96 21 fb 92 8d 28 a0 01   ................
00000000   48 30 45 02 21 00 bb 95  2f de 81 f2 16 f7 06 35   ................
00000000   75 c0 bb 2b ed c0 50 ce  08 c9 6d 9b 43 7e a9 22   ................
00000000   f5 eb 98 c8 82 da 02 20  1b 7c bf 3a 2f 94 ea 4c   ................
00000000   5e b7 f0 df 3a f2 eb ca  fa 87 05 af 7f 41 0a b5   ................
00000000   d3 d4 ba c1 3d 6b c6 12  01 47 52 21 02 63 21 78   ................
00000000   d0 46 67 3c 97 29 d8 28  cf ee 38 8e 12 1f 49 77   ................
00000000   07 f8 10 c1 31 e0 d3 fc  0f e0 bd 66 d6 21 03 a0   ................
00000000   95 1e c7 d3 a9 da 9d e1  71 61 70 26 44 2f cd 30   ................
00000000   f3 4d 66 10 0f ab 53 98  53 b4 3f 50 87 87 d4 52   ................
00000000   ae ff ff ff ff 02 40 42  0f 00 00 00 00 00 17 a9   ................
00000000   14 d3 db 9a 20 31 2c 3a  b8 96 a3 16 eb 10 8d bd   ................
00000000   01 e4 7e 17 d6 87 e0 ba  7a c6 00 00 00 00 17 a9   ................
00000000   14 8c e5 40 8c fe ad db  7c cb 25 45 de d4 1e f4   ................
00000000   78 10 94 54 84 87 00 00  00 00 01 00 00 00 01 6e   ................
00000000   3c ca 15 99 cd e5 48 78  e2 f2 7f 43 4d f6 9d f0   ................
00000000   af d1 f3 13 cb 6e 38 c0  8d 3f fb 57 f9 7a 6c 01   ................
00000000   00 00 00 da 00 48 30 45  02 21 00 95 62 3b 70 ec   ................
00000000   31 94 fa 40 37 a1 c1 10  6c 25 80 ca ed c3 90 e2   ................
00000000   5e 5b 33 0b be b3 11 1e  81 84 bc 02 20 5a e9 73   ................
00000000   c4 a4 45 4b e2 a3 a0 3b  eb 66 29 71 43 c1 04 4a   ................
00000000   3c 47 43 74 2c 5c dd 1d  51 6a 1a d3 04 01 47 30   ................
00000000   44 02 20 2f 3d 6d 89 99  6f 5b 42 77 3d d6 eb af   ................
00000000   36 7f 1a f1 f3 a9 5c 7c  7b 48 7e c0 40 13 1c 40   ................
00000000   f4 a4 a3 02 20 52 4f fb  b0 b5 63 f3 7b 3e b1 34   ................
00000000   12 28 f7 92 e8 f8 41 11  b7 c4 a9 f4 9c dd 99 8e   ................
00000000   05 2e e4 2e fa 01 47 52  21 02 63 21 78 d0 46 67   ................
00000000   3c 97 29 d8 28 cf ee 38  8e 12 1f 49 77 07 f8 10   ................
00000000   c1 31 e0 d3 fc 0f e0 bd  66 d6 21 03 a0 95 1e c7   ................
00000000   d3 a9 da 9d e1 71 61 70  26 44 2f cd 30 f3 4d 66   ................
00000000   10 0f ab 53 98 53 b4 3f  50 87 87 d4 52 ae ff ff   ................
00000000   ff ff 02 40 42 0f 00 00  00 00 00 17 a9 14 1a de   ................
00000000   6b 95 89 6d de 8e c4 de  e9 e5 9a f8 84 9d 37 97   ................
00000000   34 8e 87 28 af 7a c6 00  00 00 00 17 a9 14 8c e5   ................
00000000   40 8c fe ad db 7c cb 25  45 de d4 1e f4 78 10 94   ................
00000000   54 84 87 00 00 00 00 01  00 00 00 01 1d 9d c3 a5   ................
00000000   df 9b 5b 2e eb 2b d1 1a  2d b2 43 be 9e 8c c2 3e   ................
00000000   2f 18 0b f3 17 d3 2a 49  99 04 c1 55 01 00 00 00   ................
00000000   db 00 48 30 45 02 21 00  eb bd 1c 9a 8c e6 26 ed   ................
00000000   bb 1a 78 81 df 81 e8 72  ef 8c 64 24 fe da 36 fa   ................
00000000   a8 a5 74 51 57 40 0c 6a  02 20 6e b4 63 bc 8a cd   ................
00000000   5e a0 6a 28 9e 86 11 5e  1d aa e0 c2 cf 10 d9 cb   ................
00000000   bd 19 9e 13 11 17 0d 55  43 ef 01 48 30 45 02 21   ................
00000000   00 80 94 11 a9 17 dc 8c  f4 f3 a7 77 f0 38 8f de   ................
00000000   a6 de 06 24 3e f7 69 1e  50 0c 60 ab d1 c7 f1 9a   ................
00000000   e6 02 20 52 55 d2 b1 19  1d 8a de db 77 b8 14 cc   ................
00000000   b6 64 71 eb 84 86 cb 4f  f8 72 78 24 25 4e e5 58   ................
00000000   9f 17 6b 01 47 52 21 02  63 21 78 d0 46 67 3c 97   ................
00000000   29 d8 28 cf ee 38 8e 12  1f 49 77 07 f8 10 c1 31   ................
00000000   e0 d3 fc 0f e0 bd 66 d6  21 03 a0 95 1e c7 d3 a9   ................
00000000   da 9d e1 71 61 70 26 44  2f cd 30 f3 4d 66 10 0f   ................
00000000   ab 53 98 53 b4 3f 50 87  87 d4 52 ae ff ff ff ff   ................
00000000   02 40 42 0f 00 00 00 00  00 17 a9 14 75 9a 49 c7   ................
00000000   72 34 7b e8 1c 49 51 7f  9e 1e 6d ef 6a 88 d4 dd   ................
00000000   87 80 0b 85 c6 00 00 00  00 17 a9 14 8c e5 40 8c   ................
00000000   fe ad db 7c cb 25 45 de  d4 1e f4 78 10 94 54 84   ................
00000000   87 00 00 00 00 01 00 00  00 01 8c 51 90 2a ff d8   ................
00000000   e5 24 7d fc c2 e5 d0 52  8a 38 15 f5 3c 8b 6d 2c   ................
00000000   20 0f f2 90 b2 b2 b4 86  d7 70 4f 00 00 00 6a 47   ................
00000000   30 44 02 20 1b e0 d4 85  f6 a3 ce 87 1b e8 00 64   ................
00000000   c5 93 c5 32 7b 3f d7 e4  50 f0 5a b7 fa e3 83 85   ................
00000000   bc 40 cf be 02 20 6e 2a  6c 99 70 b5 d1 d1 02 07   ................
00000000   89 23 76 73 37 57 48 66  34 fc e4 f3 52 e7 72 14   ................
00000000   9c 48 68 57 61 21 01 21  03 50 c3 3b c9 a7 90 c9   ................
00000000   49 51 95 76 15 77 b3 49  12 a9 49 b7 3d 5b c5 ae   ................
00000000   53 43 f5 ba 08 b3 32 20  cc ff ff ff ff 01 10 27   ................
00000000   00 00 00 00 00 00 19 76  a9 14 2a b1 c6 27 10 a7   ................
00000000   bd fd b4 bb 63 94 bb ed  c5 8b 32 b4 d5 a3 88 ac   ................
00000000   00 00 00 00 01 00 00 00  01 8c 51 90 2a ff d8 e5   ................
00000000   24 7d fc c2 e5 d0 52 8a  38 15 f5 3c 8b 6d 2c 20   ................
00000000   0f f2 90 b2 b2 b4 86 d7  70 4e 00 00 00 6b 48 30   ................
00000000   45 02 21 00 cc c8 c0 ac  90 bd b0 40 28 42 ae c9   ................
00000000   18 30 c7 65 cd ea d7 a7  28 55 2a 6a 34 de 7d 13   ................
00000000   a6 da b2 8e 02 20 6c 96  f8 64 0c f3 44 40 54 e9   ................
00000000   63 2b 19 7b e3 05 98 a0  9c 3d 5d ef cd 95 75 0b   ................
00000000   db 92 2a 60 d6 48 01 21  03 50 c3 3b c9 a7 90 c9   ................
00000000   49 51 95 76 15 77 b3 49  12 a9 49 b7 3d 5b c5 ae   ................
00000000   53 43 f5 ba 08 b3 32 20  cc ff ff ff ff 01 10 27   ................
00000000   00 00 00 00 00 00 19 76  a9 14 2a b1 c6 27 10 a7   ................
00000000   bd fd b4 bb 63 94 bb ed  c5 8b 32 b4 d5 a3 88 ac   ................
00000000   00 00 00 00 01 00 00 00  01 1b 43 66 69 c0 6c bf   ................
00000000   34 42 e2 1a 2f e3 ed c2  0c d3 cf 13 c3 58 c5 32   ................
00000000   34 bc 4d 88 bf d8 c4 bd  2a 00 00 00 00 6a 47 30   ................
00000000   44 02 20 4a 63 41 0e e1  3d b5 2c 76 09 ab 08 e2   ................
00000000   5b 7f e3 c6 08 cc 21 cc  17 55 ad 13 46 06 85 eb   ................
00000000   55 19 32 02 20 4c d1 ea  80 c0 6a 81 57 11 19 be   ................
00000000   0b 8c cc d9 6e f7 cd d9  0f 62 c1 fe 2d 53 86 22   ................
00000000   fe b0 8e 22 ba 01 21 02  4b aa 8b 67 cc 9e d8 a9   ................
00000000   7d 90 89 5e 37 16 b2 54  69 b6 7c b2 6d 33 24 d7   ................
00000000   af f2 13 f5 07 76 47 65  ff ff ff ff 01 00 00 00   ................
00000000   00 00 00 00 00 30 6a 2e  51 6d 64 52 33 65 34 52   ................
00000000   61 44 56 53 32 4d 43 6a  73 6e 53 61 71 73 4a 57   ................
00000000   53 32 44 65 65 54 46 62  42 38 35 45 41 79 4a 4d   ................
00000000   58 43 78 4c 79 34 00 00  00 00 01 00 00 00 01 be   ................
00000000   4a 95 ed 36 31 6c ad a5  11 8b 19 82 e4 cb 4a 07   ................
00000000   f9 3e 7a 41 53 e2 27 46  6f 1c b0 77 6d e9 95 00   ................
00000000   00 00 00 6b 48 30 45 02  21 00 a2 2d 52 51 de ea   ................
00000000   04 70 80 6b ab 81 70 13  d6 75 a6 3c d5 22 18 d6   ................
00000000   e4 77 ab 0c 9d 60 1d 01  8b 7f 02 20 42 12 1b 46   ................
00000000   af cd cd 0c 66 f1 89 39  82 12 b6 60 85 e8 8c 69   ................
00000000   73 ae 56 0f 18 10 c1 3e  55 e2 be e4 01 21 02 4b   ................
00000000   aa 8b 67 cc 9e d8 a9 7d  90 89 5e 37 16 b2 54 69   ................
00000000   b6 7c b2 6d 33 24 d7 af  f2 13 f5 07 76 47 65 ff   ................
00000000   ff ff ff 01 00 00 00 00  00 00 00 00 30 6a 2e 51   ................
00000000   6d 57 48 4d 57 50 4e 52  48 51 58 72 50 4c 73 38   ................
00000000   55 4c 58 6b 4d 48 37 46  74 53 56 41 36 75 36 6b   ................
00000000   5a 6b 4a 4e 38 51 79 6e  4e 58 37 51 34 00 00 00   ................
00000000   00 01 00 00 00 01 6c 06  1a 65 b4 9e de c2 1a cd   ................
00000000   bc 22 f9 7d c8 53 aa 87  23 02 ae ef 13 fa bf 0b   ................
00000000   f6 80 7d e1 b8 bd 01 00  00 00 6b 48 30 45 02 21   ................
00000000   00 dd 80 38 1f 2d 15 8b  4d ad 7f 98 d2 d9 73 17   ................
00000000   c5 33 fb 36 e7 37 54 24  73 fe b0 5f a7 4d 0b 73   ................
00000000   bb 02 20 70 97 d4 33 11  96 06 91 67 e5 25 b6 1d   ................
00000000   13 25 32 29 2f d7 5c c0  39 a5 83 9c 04 c2 54 5d   ................
00000000   42 7e 2b 01 21 03 5e 9a  59 7d f8 b4 17 be f6 68   ................
00000000   11 88 2a 28 44 60 4f c5  91 c4 27 f6 42 62 8f 0f   ................
00000000   ef 46 be 19 a4 c9 fe ff  ff ff 02 80 a4 bf 07 00   ................
00000000   00 00 00 19 76 a9 14 57  3b 91 06 e1 6e e0 b5 c1   ................
00000000   43 dc 40 f0 72 4f 77 dd  0e 28 20 88 ac 95 33 b2   ................
00000000   2c 00 00 00 00 19 76 a9  14 9c 4d a6 07 ef b1 d7   ................
00000000   59 d3 3d a7 17 78 bc 6c  af a5 6a cb 59 88 ac d3   ................
00000000   1b 0e 00 01 00 00 00 01  7d ae 20 99 4b 69 b2 85   ................
00000000   34 e5 b2 2f 3d 7c 50 f9  d7 54 13 48 cb f6 f4 3f   ................
00000000   cc 65 42 63 eb af 8f 68  00 00 00 00 6b 48 30 45   ................
00000000   02 21 00 a8 53 00 eb 94  b2 4b 04 48 77 d0 b0 d6   ................
00000000   1e 08 e1 6d bc 82 ec 7d  69 c7 23 a8 a4 55 19 f9   ................
00000000   5c 35 b0 02 20 3d 78 37  6e 6b ee 31 b4 55 c0 97   ................
00000000   55 7a f7 fe 4d 6b 62 0b  c7 42 69 e9 a7 5e 2a ad   ................
00000000   2b 54 5a bd db 01 21 03  b0 d0 8a ba 2a 5a c6 cf   ................
00000000   27 88 fd a9 41 c3 86 04  0e 35 e4 9d 3a 57 d2 ae   ................
00000000   fb 16 c0 43 8f b9 8a cb  fe ff ff ff 02 22 22 30   ................
00000000   5f 00 00 00 00 19 76 a9  14 cf da 30 dd 83 6b 59   ................
00000000   6d b6 a9 c2 30 c4 5a e2  17 91 07 f0 48 88 ac 80   ................
00000000   a4 bf 07 00 00 00 00 19  76 a9 14 42 df cf 58 23   ................
00000000   aa cb 18 58 44 e6 63 87  3c 35 fb 98 bf d2 1b 88   ................
00000000   ac d3 1b 0e 00 01 00 00  00 02 ad 3e 85 e4 af 30   ................
00000000   67 8a 33 0f 89 41 ed 7a  9c a1 7c d0 23 63 68 d2   ................
00000000   38 ca c4 e9 ff 09 c4 66  fe d1 02 00 00 00 6b 48   ................
00000000   30 45 02 21 00 d1 19 6c  48 a0 39 2e 09 59 2f 1b   ................
00000000   96 b4 ae c3 2a b0 ce cb  6f d1 7b 1d 0c 85 ab 32   ................
00000000   50 a2 fe 45 d9 02 20 59  21 7c 82 f6 84 fc de cd   ................
00000000   be 66 0a 20 77 ea 95 6d  fb bb 96 4d 26 48 bc 1e   ................
00000000   8a e0 f0 fe 56 54 49 01  21 03 b6 4e 32 e5 f6 2e   ................
00000000   03 70 14 28 fb 1e 31 51  e9 a5 7f 14 9c 67 70 8f   ................
00000000   61 64 a2 35 c8 19 9f e1  7c c2 ff ff ff ff 34 f0   ................
00000000   a7 1c 1c 2c d6 10 52 2e  9c 18 c6 79 31 cd ed 5e   ................
00000000   96 47 d4 41 9c 49 b9 97  15 e2 a0 79 5f 3d 02 00   ................
00000000   00 00 6a 47 30 44 02 20  31 6e 81 d8 24 2a bf 3c   ................
00000000   5f 88 5d 20 0f ec a1 2c  3a db 63 cf 2c d4 dc 74   ................
00000000   60 2f 7b 8b 0c ba 50 34  02 20 21 0d 52 57 58 df   ................
00000000   77 cc dc a6 90 83 11 c1  89 52 75 e0 7b bb 29 b4   ................
00000000   59 63 a1 92 52 ac de 55  87 3f 01 21 03 b6 4e 32   ................
00000000   e5 f6 2e 03 70 14 28 fb  1e 31 51 e9 a5 7f 14 9c   ................
00000000   67 70 8f 61 64 a2 35 c8  19 9f e1 7c c2 ff ff ff   ................
00000000   ff 05 10 27 00 00 00 00  00 00 19 76 a9 14 44 9d   ................
00000000   23 94 dd e0 57 bc 19 9f  23 fb 8a a2 e4 00 f3 44   ................
00000000   61 17 88 ac 10 27 00 00  00 00 00 00 19 76 a9 14   ................
00000000   44 9d 23 94 dd e0 57 bc  19 9f 23 fb 8a a2 e4 00   ................
00000000   f3 44 61 17 88 ac a0 86  01 00 00 00 00 00 19 76   ................
00000000   a9 14 13 d3 5a d3 37 dd  80 a0 55 75 7e 5e a0 a4   ................
00000000   5b 59 fe e3 06 0c 88 ac  70 11 01 00 00 00 00 00   ................
00000000   19 76 a9 14 13 d3 5a d3  37 dd 80 a0 55 75 7e 5e   ................
00000000   a0 a4 5b 59 fe e3 06 0c  88 ac 00 00 00 00 00 00   ................
00000000   00 00 02 6a 00 00 00 00  00 01 00 00 00 01 8e 33   ................
00000000   fe cc 2d db d8 6c 5e a9  19 f7 bd 5a 5a cf 8a 09   ................
00000000   f3 e0 cd aa af 4f 08 c5  ef 09 51 61 ef 11 00 00   ................
00000000   00 00 fd fe 00 00 48 30  45 02 21 00 d2 48 9b 22   ....signature...
00000000   5d 39 b7 d8 b6 76 7a 69  28 c8 02 9a 2a 12 97 c0   ................
00000000   8f df 00 d6 83 ba 0c 19  87 e7 d7 00 02 20 17 6c   ................
00000000   b6 6c 8a 24 38 06 bb 74  21 f6 58 32 5a 69 a5 1c   ................
00000000   82 c0 c3 31 4e 37 f2 40  0f 33 62 63 90 21 01 48   ................
00000000   30 45 02 21 00 96 cf a5  76 62 a5 45 83 0d 0e 29   ................
00000000   61 0b ec d4 1e a0 31 e2  56 33 99 13 71 8c e1 8d   ................
00000000   bb 1a 27 bd b0 02 20 48  29 11 c8 51 d1 5a dc d3   ................
00000000   70 97 df f9 9a 9f f1 f9  7d 95 3b ce bc 52 88 35   ................
00000000   11 8f 44 74 12 55 3e 01  4c 69 52 21 02 8d 98 89   ................
00000000   86 2b 29 43 02 78 c0 84  b5 c4 09 0b 7b 80 7b 31   ................
00000000   e0 47 bc d2 12 eb c2 c4  e4 3f c0 e3 c5 21 03 16   ................
00000000   09 49 a7 c8 c8 1f 2c 25  d7 76 3f 57 eb 1c b4 07   ................
00000000   d8 67 c5 b7 c2 90 33 1b  d2 dc 4b 11 82 c6 d3 21   ................
00000000   03 fb ef 3b 60 91 4b da  91 73 76 59 02 01 3a 25   ................
00000000   1e c8 94 50 c7 5d 0b 5a  96 a1 43 db 1d ab f9 8d   .......signature
00000000   95 53 ae ff ff ff ff 02  20 e8 89 1c 01 00 00 00   ................
00000000   17 a9 14 d9 96 71 5e 08  1c 50 f8 f6 b1 b4 e7 fb   ................
00000000   6c a2 14 f9 92 4f df 87  80 96 98 00 00 00 00 00   ................
00000000   17 a9 14 56 11 d8 12 26  3f 32 96 02 28 cb 5f 85   ................
00000000   32 9b ce 47 70 a2 18 87  00 00 00 00 01 00 00 00   ................
00000000   01 77 20 50 7d cb e6 c6  9f 65 2b 0c 0c e1 94 06   ................
00000000   f4 82 37 2d 1a 8a bc 05  d4 5f b7 ac f9 7f b8 0e   ................
00000000   ec 00 00 00 00 fd fe 00  00 48 30 45 02 21 00 98   ................
00000000   21 d8 e1 17 de 44 b1 20  2c 82 9c 0f 50 63 99 7a   ................
00000000   cf 00 7c f9 b5 61 c6 fb  8d 12 12 cd db 6c 40 02   ................
00000000   20 10 ff 50 67 b0 d9 d4  ec a2 da 0c eb 87 6e 9a   ................
00000000   16 f1 a2 14 2d a8 66 d3  04 2a 7b ae 89 68 81 3e   ................
00000000   80 01 48 30 45 02 21 00  de a7 59 d1 4a 8a 1c 5d   ................
00000000   a5 f3 dc c5 50 98 71 aa  a2 c1 e3 be 03 75 2c 1b   ................
00000000   85 8d 80 fa 42 27 16 37  02 20 51 83 d7 0c c2 8d   ................
00000000   cb 6d f9 b0 37 71 4c 8b  64 42 ef 84 e0 dd ce 07   ................
00000000   71 1a 30 c7 31 e9 f0 92  50 90 01 4c 69 52 21 02   ................
00000000   8d 70 ea 66 fe 7a 7d ef  28 2d f7 b2 b4 98 00 7e   ................
00000000   50 72 93 3e 42 c1 8f 63  ce 85 97 5d cb cf 1a 88   ................
00000000   21 03 7e 8f 84 2b 1e 47  e2 1d 88 00 2c 5a ab 25   ................
00000000   59 21 2a 4c 2c 9d be 5e  f5 34 7f 2a 29 af d0 51   ................
00000000   0e c1 21 02 51 25 9c b9  fd 4f 62 06 48 84 08 28   ................
00000000   6e 44 75 c9 c9 fe 88 7e  57 a3 e3 2a e4 da 22 27   ................
00000000   78 a2 ae df 53 ae ff ff  ff ff 02 33 80 cb 02 00   ................
00000000   00 00 00 17 a9 14 3b 5a  7e 85 b2 26 56 a3 4d 43   ................
00000000   18 7a c8 dd 09 ac d7 10  9d 24 87 80 96 98 00 00   ................
00000000   00 00 00 17 a9 14 b9 b4  b5 55 f5 94 a3 4d ee c3   ................
00000000   ad 61 d5 c5 f3 73 8b 17  ee 15 87 00 00 00 00      ...............
";

    let hex: Vec<u8> = hexdump::decode(dump);

    assert_eq!(hex.len(), 4319);

    let result = block::parse(&hex);
    assert!(result.is_ok());

    let b : Block = result.ok().unwrap();

    assert_eq!(b.version, 0x20000000, "b.version");
    assert_eq!(
        b.previous,
        [
            0x2a, 0xa2, 0xf2, 0xca, 0x79, 0x4c, 0xcb, 0xd4, 0x0c, 0x16, 0xe2, 0xf3, 0x33, 0x3f, 0x6b, 0x8b, 
            0x68, 0x3f, 0x9e, 0x71, 0x79, 0xb2, 0xc4, 0xd7, 0x49, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ],
        "b.previous"
    );

    assert_eq!(
        b.merkleroot,
        [
            0x10, 0xbc, 0x26, 0xe7, 0x0a, 0x2f, 0x67, 0x2a, 0xd4, 0x20, 0xa6, 0x15, 0x3d, 0xd0, 0xc2, 0x8b, 
            0x40, 0xa6, 0x00, 0x2c, 0x55, 0x53, 0x1b, 0xfc, 0x99, 0xbf, 0x89, 0x94, 0xa8, 0xe8, 0xf6, 0x7e
        ],
        "b.merkleroot"
    );

    assert_eq!(b.time, 1472004949, "b.time");
    assert_eq!(b.bits, 436655184, "b.bits");
    assert_eq!(b.nonce, 1879759182, "b.nonce");
    assert_eq!(b.transactions.len(), 15, "b.transactions.len");

    let t1: &Transaction = b.transactions.get(0).unwrap();

    assert_eq!(t1.inputs.len(), 1, "t1.inputs.len");
    assert_eq!(t1.outputs.len(), 2, "t1.outputs.len");
    assert_eq!(t1.version, 0x00000001, "t1.version");
    assert_eq!(t1.locktime, 0x00000000, "t1.locktime");

    let t1i: &TxIn = t1.inputs.get(0).unwrap();

    assert_eq!(t1i.sequence, 0xFFFFFFFF, "t1i.sequence");
    assert_eq!(
        t1i.signature.content.len(),
        0x36,
        "t1i.signature.content.len"
    );
    assert_eq!(
        t1i.signature.content,
        vec![
        3, 218, 27, 14, 0, 4, 85, 3, 189, 87, 4, 199, 221, 138, 13, 12, 237, 
        19, 187, 87, 133, 1, 8, 0, 0, 0, 0, 0, 10, 99, 107, 112, 111, 111, 
        108, 18, 47, 78, 105, 110, 106, 97, 80, 111, 111, 108, 47, 83, 
        69, 71, 87, 73, 84, 47
    ],
        "t1i.signature.content"
    );

    let t1ip: &OutPoint = &t1i.previous;

    assert_eq!(t1ip.transaction_hash, [0; 32], "t1ip.transaction_hash");
    assert_eq!(t1ip.index, 4294967295, "t1ip.index");

    let t1o: &TxOut = t1.outputs.get(0).unwrap();

    assert_eq!(t1o.amount, 312665524, "t1o.amount"); 
    assert_eq!(
        t1o.script_pubkey.content.len(),
        0x19,
        "t1o.script_pubkey.content.len"
    );

    assert_eq!(
        t1o.script_pubkey.content,
        vec![
            118, 169, 20, 135, 111, 187, 130, 236, 5, 202, 166, 175, 122, 59, 94, 90, 
            152, 58, 174, 108, 108, 198, 214, 136, 172
        ],
        "t1o.script_pubkey.content"
    );

    assert!(t1.witness.is_some(), "t1.witness");
    let t1ws : &Vec<Witness> = t1.witness.as_ref().unwrap();
    let t1w : &Witness = t1ws.get(0).unwrap();

    assert_eq!(t1w.data.len(), 0x20, "t1w.data.len");
    assert_eq!(
        t1w.data,
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        ],
        "t1w.data"
    );


    let t2: &Transaction = b.transactions.get(1).unwrap();

    assert_eq!(t2.inputs.len(), 1, "t2.inputs.len");
    assert_eq!(t2.outputs.len(), 2, "t2.outputs.len");
    assert_eq!(t2.version, 0x00000001, "t2.version");
    assert_eq!(t2.locktime, 0x00000000, "t2.locktime");

    let t2i1: &TxIn = t2.inputs.get(0).unwrap();

    assert_eq!(t2i1.sequence, 0xFFFFFFFF, "t2i1.sequence");
    assert_eq!(t2i1.signature.content.len(), 0x6B, "t2i1.signature.content.len");
    assert_eq!(t2i1.signature.content, vec![
        72, 48, 69, 2, 33, 0, 202, 200, 9, 205, 26, 61, 154, 213, 213, 227, 26, 
        132, 226, 225, 216, 236, 85, 66, 132, 30, 77, 20, 198, 181, 46, 139, 56, 
        203, 225, 255, 23, 40, 2, 32, 100, 71, 11, 127, 176, 194, 239, 236, 203, 
        46, 132, 191, 163, 110, 197, 249, 228, 52, 200, 75, 17, 1, 192, 15, 126, 
        227, 47, 114, 99, 113, 183, 65, 1, 33, 2, 14, 98, 40, 7, 152, 182, 184, 
        195, 127, 6, 141, 240, 145, 91, 8, 101, 182, 63, 171, 196, 1, 194, 69, 
        124, 188, 62, 249, 104, 135, 221, 54, 71 
        ], "t2i1.signature.content"
    );

    assert!(t2.witness.is_none(), "t2.witness");

    let t2i1p: &OutPoint = &t2i1.previous;

    assert_eq!(t2i1p.transaction_hash, [
        126, 79, 129, 23, 83, 50, 167, 51, 226, 109, 75, 164, 226, 159, 83, 246, 
        123, 122, 93, 124, 42, 222, 187, 39, 110, 68, 124, 167, 29, 19, 11, 85
    ], "t2i1p.transaction_hash");
    assert_eq!(t2i1p.index, 0x00, "t2i1p.index");


    let t2o: &TxOut = t2.outputs.get(0).unwrap();

    assert_eq!(t2o.amount, 209203146, "t2o.amount");
    assert_eq!(
        t2o.script_pubkey.content.len(),
        0x19,
        "t2o.script_pubkey.content.len"
    );

    assert_eq!(
        t2o.script_pubkey.content,
        vec![
            118, 169, 20, 198, 181, 84, 91, 53, 146, 203, 71, 125, 112, 152, 
            150, 250, 112, 85, 146, 201, 182, 17, 58, 136, 172
        ],
        "t2o.script_pubkey.content"
    );

}
