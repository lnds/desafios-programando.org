use crate::packed_struct::PackedStruct;

use crate::packet::{BytePacketBuffer, Result};
use packed_struct::prelude::*;
use std::net::Ipv4Addr;

#[derive(PrimitiveEnum_u8, Debug, Clone, Copy, PartialEq)]
pub enum ResultCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

#[derive(PackedStruct, Clone, Debug)]
#[packed_struct(bit_numbering = "msb0")]
pub struct DnsHeader {
    #[packed_field(bits = "0..=15", endian = "msb")]
    pub id: u16, // 16 bits
    #[packed_field(bits = "16")]
    pub recursion_desired: bool, // 1 bit
    #[packed_field(bits = "17")]
    pub truncated_message: bool, // 1 bit
    #[packed_field(bits = "18")]
    pub authoritative_answer: bool, // 1 bit
    #[packed_field(bits = "19..=22")]
    pub opcode: Integer<u8, packed_bits::Bits4>, // 4 bits
    #[packed_field(bits = "23")]
    pub response: bool, // 1 bit
    #[packed_field(bits = "24..=27", ty = "enum")]
    pub rescode: EnumCatchAll<ResultCode>, // 4 bits
    #[packed_field(bits = "28")]
    pub checking_disabled: bool, // 1 bit
    #[packed_field(bits = "29")]
    pub authed_data: bool, // 1 bit
    #[packed_field(bits = "30")]
    pub z: bool, // 1 bit
    #[packed_field(bits = "31")]
    pub recursion_available: bool, // 1 bit
    #[packed_field(bits = "32..=47", endian = "msb")]
    pub questions: u16, // 16 bits
    #[packed_field(bits = "48..=63", endian = "msb")]
    pub answers: u16, // 16 bits
    #[packed_field(bits = "64..=79", endian = "msb")]
    pub authoritative_entries: u16, // 16 bits
    #[packed_field(bits = "80..=95", endian = "msb")]
    pub resource_entries: u16, // 16 bits
}

impl DnsHeader {
    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsHeader> {
        let mut packed = [0u8; 12];
        buffer.read_exact(&mut packed)?;
        Ok(DnsHeader::unpack(&packed).unwrap())
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType {
    UNKNOWN(u16),
    A, // 1
}

impl QueryType {
    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::A,
            _ => QueryType::UNKNOWN(num),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType,
}

impl DnsQuestion {
    pub fn from(buffer: &mut BytePacketBuffer) -> Result<Self> {
        let name = buffer.read_qname()?;
        let qtype = QueryType::from_num(buffer.read_u16()?);
        let _ = buffer.read_u16()?; // classs
        Ok(DnsQuestion { name, qtype })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DnsRecord {
    UNKNOWN {
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32,
    },
    // 0
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32,
    }, // 1
}

impl DnsRecord {
    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord> {
        let domain = buffer.read_qname()?;

        let qtype_num = buffer.read_u16()?;
        let qtype = QueryType::from_num(qtype_num);
        let _ = buffer.read_u16()?;
        let ttl = buffer.read_u32()?;
        let data_len = buffer.read_u16()?;

        match qtype {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                let addr = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    (raw_addr & 0xFF) as u8,
                );

                Ok(DnsRecord::A { domain, addr, ttl })
            }
            QueryType::UNKNOWN(_) => {
                buffer.step(data_len as usize)?;

                Ok(DnsRecord::UNKNOWN {
                    domain,
                    qtype: qtype_num,
                    data_len,
                    ttl,
                })
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket> {
        let header = DnsHeader::read(buffer)?;
        let questions = (0..header.questions)
            .flat_map(|_| DnsQuestion::from(buffer))
            .collect();
        let answers = (0..header.answers)
            .flat_map(|_| DnsRecord::read(buffer))
            .collect();
        let authorities = (0..header.authoritative_entries)
            .flat_map(|_| DnsRecord::read(buffer))
            .collect();
        let resources = (0..header.resource_entries)
            .flat_map(|_| DnsRecord::read(buffer))
            .collect();
        Ok(DnsPacket {
            header,
            questions,
            answers,
            authorities,
            resources,
        })
    }
}
