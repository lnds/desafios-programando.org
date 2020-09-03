use crate::packed_struct::PackedStruct;

use crate::packet::{BytePacketBuffer, Result};
use packed_struct::prelude::*;
use std::net::Ipv4Addr;

#[derive(PrimitiveEnum_u8, Debug, Clone, Copy, PartialEq, Eq)]
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
    #[packed_field(bits = "31")]
    pub recursion_desired: bool, // 1 bit
    #[packed_field(bits = "30")]
    pub truncated_message: bool, // 1 bit
    #[packed_field(bits = "29")]
    pub authoritative_answer: bool, // 1 bit
    #[packed_field(bits = "25..=28")]
    pub opcode: Integer<u8, packed_bits::Bits4>, // 4 bits
    #[packed_field(bits = "24")]
    pub response: bool, // 1 bit
    #[packed_field(bits = "20..23", ty = "enum")]
    pub rescode: EnumCatchAll<ResultCode>, // 4 bits
    #[packed_field(bits = "19")]
    pub checking_disabled: bool, // 1 bit
    #[packed_field(bits = "18")]
    pub authed_data: bool, // 1 bit
    #[packed_field(bits = "17")]
    pub z: bool, // 1 bit
    #[packed_field(bits = "16")]
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
    pub fn new() -> Self {
        let packed = [0u8; 12];
        DnsHeader::unpack(&packed).unwrap()
    }

    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsHeader> {
        let mut packed = [0u8; 12];
        buffer.read_exact(&mut packed)?;
        Ok(DnsHeader::unpack(&packed).unwrap())
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<()> {
        let packed = self.pack();
        buffer.write_exact(&packed)?;
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType {
    UNKNOWN(u16),
    A, // 1
}

impl QueryType {
    pub fn to_num(&self) -> u16 {
        match *self {
            QueryType::UNKNOWN(x) => x,
            QueryType::A => 1,
        }
    }

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
    pub fn new(name: String, qtype: QueryType) -> Self {
        DnsQuestion { name, qtype }
    }

    pub fn read(buffer: &mut BytePacketBuffer) -> Result<Self> {
        let name = buffer.read_qname()?;
        let qtype = QueryType::from_num(buffer.read_u16()?);
        let _ = buffer.read_u16()?; // classs
        Ok(DnsQuestion { name, qtype })
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.write_qname(&self.name)?;

        let typenum = self.qtype.to_num();
        buffer.write_u16(typenum)?;
        buffer.write_u16(1)?;

        Ok(())
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

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<usize> {
        let start_pos = buffer.pos();

        match *self {
            DnsRecord::A {
                ref domain,
                ref addr,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::A.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;
                buffer.write_u16(4)?;

                let octets = addr.octets();
                buffer.write_u8(octets[0])?;
                buffer.write_u8(octets[1])?;
                buffer.write_u8(octets[2])?;
                buffer.write_u8(octets[3])?;
            }
            DnsRecord::UNKNOWN { .. } => {
                println!("Skipping record: {:?}", self);
            }
        }

        Ok(buffer.pos() - start_pos)
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
    pub fn new() -> DnsPacket {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new(),
        }
    }

    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsPacket> {
        let header = DnsHeader::read(buffer)?;
        let questions = (0..header.questions)
            .flat_map(|_| DnsQuestion::read(buffer))
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

    pub fn write(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {
        self.header.questions = self.questions.len() as u16;
        self.header.answers = self.answers.len() as u16;
        self.header.authoritative_entries = self.authorities.len() as u16;
        self.header.resource_entries = self.resources.len() as u16;

        self.header.write(buffer)?;

        for question in &self.questions {
            question.write(buffer)?;
        }
        for rec in &self.answers {
            rec.write(buffer)?;
        }
        for rec in &self.authorities {
            rec.write(buffer)?;
        }
        for rec in &self.resources {
            rec.write(buffer)?;
        }

        Ok(())
    }
}
