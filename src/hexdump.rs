use regex::Regex;

pub fn parse(hexdump: &str) -> Vec<u8> {

    //00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
    //00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
    let re = Regex::new(r"^(?P<offset>[0-9A-F]{8})\s{3}(?P<hexa>[0-9A-F\s]{48})\s{3}(?P<dump>.{1,16})$").unwrap();

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