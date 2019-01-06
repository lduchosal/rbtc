use crate::network::message::Command;
use crate::network::error::{EncodeError};
use crate::network::message::{NetworkMessage, Encodable};


/// https://en.bitcoin.it/wiki/Protocol_documentation#getaddr
/// 
/// getaddr
/// 
/// The getaddr message sends a request to a node asking for information 
/// about known active peers to help with finding potential nodes in the network. 
/// The response to receiving this message is to transmit one or more addr messages 
/// with one or more peers from a database of known active peers. 
/// 
/// The typical presumption is that a node is likely to be active if it has been sending 
/// a message within the last three hours.
/// 
/// No additional data is transmitted with this message.
/// 
#[derive(Debug)]
pub struct GetAddr {
}

impl NetworkMessage for GetAddr {

    fn command(&self) -> Command {
        Command::GetAddr
    }
}

impl Encodable for GetAddr {

    fn encode(&self, _: &mut Vec<u8>) -> Result<(), EncodeError> {
        Ok(())
    }
}


#[cfg(test)]
mod test {

    use crate::network::message::Encodable;
    use crate::network::getaddr::GetAddr;

    #[test]
    fn when_encode_getaddr_then_nothing_to_encode() {

        let message = GetAddr {};
        let mut data : Vec<u8> = Vec::new();

        let result = message.encode(&mut data);
        assert!(result.is_ok());
        assert_eq!(0, data.len())
    }

}