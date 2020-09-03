extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;
mod packet;
mod protocol;

use crate::protocol::{DnsPacket, DnsQuestion, QueryType};
use packet::{BytePacketBuffer, Result};
use std::net::UdpSocket;

#[allow(clippy::unused_io_amount)]
fn main() -> Result<()> {
    // Perform an A query for google.com
    let qname = "www.yahoo.com";
    let qtype = QueryType::A;

    // Using googles public DNS server
    let server = ("8.8.8.8", 53);

    // Bind a UDP socket to an arbitrary port
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    // Build our query packet. It's important that we remember to set the
    // `recursion_desired` flag. As noted earlier, the packet id is arbitrary.
    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    // Use our new write method to write the packet to a buffer...
    let mut req_buffer = BytePacketBuffer::new();
    println!("about to write...");
    packet.write(&mut req_buffer)?;
    println!("{:?}", packet);
    println!("about to send... {}", req_buffer.pos);

    // ...and send it off to the server using our socket:
    let sent = socket.send_to(&req_buffer.buf[0..req_buffer.pos], server)?;

    println!("sent {}...", sent);
    // To prepare for receiving the response, we'll create a new `BytePacketBuffer`,
    // and ask the socket to write the response directly into our buffer.
    let mut res_buffer = BytePacketBuffer::new();
    println!("about to recv...");

    let read = socket.recv_from(&mut res_buffer.buf)?;
    println!("buf {:?}", read);
    // As per the previous section, `DnsPacket::from_buffer()` is then used to
    // actually parse the packet after which we can print the response.
    let res_packet = DnsPacket::read(&mut res_buffer)?;
    println!("{:#?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:#?}", q);
    }
    for rec in res_packet.answers {
        println!("{:#?}", rec);
    }
    for rec in res_packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in res_packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
