use regex::Regex;

pub fn decode(hexdump: &str) -> Vec<u8> {

    //00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
    //00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
    let re = Regex::new(r"^(?P<offset>[0-9A-Fa-f]{8})\s{3}(?P<hexa>[0-9A-Fa-f\s]{48})\s{3}(?P<dump>.{1,16})$").unwrap();

    let mut result : Vec<u8> = Vec::new();
    for line in hexdump.lines() {
        let capture = re.captures(line);
        match capture {
            Some(cap) => {
                let hexa = cap.name("hexa")
                    .map_or("", |m| m.as_str())
                    .replace(" ", "")
                    ;
                
                let mut vhex = hex::decode(hexa).unwrap();
                result.append(&mut vhex);
            },
            None => continue,
        }
    }
    result
}

pub fn encode(data: Vec<u8>) -> String {
    let mut result = "".to_string();
    let mut encoded = hex::encode_upper(&data);
    if encoded.len() == 0 {
        encoded.push_str(" ");
    }
    while encoded.len() % 32 != 0 {
        encoded.push_str(" ");
    }

    let mut orig = data.iter();
    let mut bytes = encoded.bytes();
    let count = bytes.len();
    result.push_str("\n");

    let mut endstring = "".to_string();
    for i in 0..count {

        if i % 32 == 0 {
            result.push_str(&format!("{:01$x}  ", i / 2, 8));
        } 

        if i % 2 == 0 {
            result.push_str(" ");
            let nextc =  orig.next();
            if let Some(c) = nextc {
                if *c >= 0x20     // non printable
                    && *c <= 0x7E // non printable
                    && *c != 0x5C // \ trouble copy/pasting to rust string
                    && *c != 0x34 // " trouble copy/pasting to rust string
                    && !(*c as char).is_control()
                {
                    let vec = vec![*c];
                    let svec = String::from_utf8(vec);
                    endstring.push_str(svec.as_ref().unwrap());
                } else {
                    endstring.push_str(".");
                }
            } else {
                endstring.push_str(" ");
            }
        } 

        let c = bytes.nth(0).unwrap();
        result.push(c as char);

        if (i+1) % 16 == 0 {
            result.push_str(" ");
        } 

        if (i+1) % 32 == 0 {
            result.push_str(&format!("  {}\n", endstring));
            endstring.truncate(0);
        } 
    }

    result
}



#[cfg(test)]
mod test {

    use crate::utils::hexdump;

    #[test]
    fn when_encode_16_then_ok() {

        let data : Vec<u8> = vec![0; 16];
        let expected = "
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
";
        let result = hexdump::encode(data);

        assert_eq!(expected, result)
    }
    #[test]
    fn when_encode_32_then_ok() {

        let data : Vec<u8> = vec![0; 32];
        let expected = "
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
";
        let result = hexdump::encode(data);

        assert_eq!(expected, result)
    }


    #[test]
    fn when_encode_48_then_ok() {

        let data : Vec<u8> = vec![0; 48];
        let expected = "
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000020   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
";
        let result = hexdump::encode(data);

        assert_eq!(expected, result)
    }


    #[test]
    fn when_encode_256_then_ok() {

        let data : Vec<u8> = vec![0; 256];
        let expected = "
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000020   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000030   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000040   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000050   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000060   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000070   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000080   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000090   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
000000a0   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
000000b0   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
000000c0   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
000000d0   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
000000e0   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
000000f0   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
";
        let result = hexdump::encode(data);

        assert_eq!(expected, result)
    }


    #[test]
    fn when_encode_8_then_ok() {

        let data : Vec<u8> = vec![0; 8];
        let expected = "
00000000   00 00 00 00 00 00 00 00                            ........        
";
        let result = hexdump::encode(data);

        assert_eq!(expected, result)
    }

    #[test]
    fn when_encode_0_then_ok() {

        let data : Vec<u8> = vec![0; 0];
        let expected = "
00000000                                                                      
";
        let result = hexdump::encode(data);

        assert_eq!(expected, result)
    }


    #[test]
    fn when_encode_24_then_ok() {

        let data : Vec<u8> = vec![0; 24];
        let expected = "
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000010   00 00 00 00 00 00 00 00                            ........        
";
        let result = hexdump::encode(data);

        assert_eq!(expected, result)
    }

    #[test]
    fn when_encode_40_then_ok() {

        let data : Vec<u8> = vec![0; 40];
        let expected = "
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000010   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000020   00 00 00 00 00 00 00 00                            ........        
";
        let result = hexdump::encode(data);

        assert_eq!(expected, result)
    }

    #[test]
    fn when_encode_40_one_then_ok() {

        let data : Vec<u8> = vec![1; 40];
        let expected = "
00000000   01 01 01 01 01 01 01 01  01 01 01 01 01 01 01 01   ................
00000010   01 01 01 01 01 01 01 01  01 01 01 01 01 01 01 01   ................
00000020   01 01 01 01 01 01 01 01                            ........        
";
        let result = hexdump::encode(data);

        assert_eq!(expected, result)
    }

    #[test]
    fn when_encode_40_61_then_ok() {

        let data : Vec<u8> = vec![97; 40];
        let expected = "
00000000   61 61 61 61 61 61 61 61  61 61 61 61 61 61 61 61   aaaaaaaaaaaaaaaa
00000010   61 61 61 61 61 61 61 61  61 61 61 61 61 61 61 61   aaaaaaaaaaaaaaaa
00000020   61 61 61 61 61 61 61 61                            aaaaaaaa        
";
        let result = hexdump::encode(data);

        assert_eq!(expected, result)
    }

}