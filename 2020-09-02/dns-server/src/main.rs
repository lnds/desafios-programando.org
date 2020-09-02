extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;
mod packet;
mod protocol;

use crate::protocol::DnsPacket;
use packet::{BytePacketBuffer, Result};
use std::fs::File;
use std::io::Read;

#[allow(clippy::unused_io_amount)]
fn main() -> Result<()> {
    let mut f = File::open("response_packet.txt")?;
    let mut buffer = BytePacketBuffer::default();
    f.read(&mut buffer.buf)?;
    let packet = DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
