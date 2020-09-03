use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

const PACKET_SIZE: usize = 512;

pub struct BytePacketBuffer {
    pub buf: [u8; PACKET_SIZE],
    pub pos: usize,
}

impl BytePacketBuffer {
    /// This gives us a fresh buffer for holding the packet contents, and a
    /// field for keeping track of where we are.
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; PACKET_SIZE],
            pos: 0,
        }
    }

    /// Current position within buffer
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Step the buffer position forward a specific number of steps
    pub fn step(&mut self, steps: usize) -> Result<()> {
        self.pos += steps;
        Ok(())
    }

    /// Change the buffer position
    fn seek(&mut self, pos: usize) -> Result<()> {
        self.pos = pos;
        Ok(())
    }

    /// Get a single byte, without changing the buffer position
    fn get(&mut self, pos: usize) -> Result<u8> {
        if pos >= PACKET_SIZE {
            return Err("End of buffer".into());
        }
        Ok(self.buf[pos])
    }

    /// Get a range of bytes
    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len >= PACKET_SIZE {
            return Err("End of buffer".into());
        }
        Ok(&self.buf[start..start + len as usize])
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let mut data = &self.buf[self.pos..];
        data.read_exact(buf)?;
        self.pos += buf.len();
        Ok(())
    }

    /// Read two bytes, stepping two steps forward
    pub fn read_u16(&mut self) -> Result<u16> {
        let mut data = &self.buf[self.pos..];
        let res = data.read_u16::<BigEndian>()?;
        self.pos += 2;
        Ok(res)
    }

    /// Read four bytes, stepping four steps forward
    pub fn read_u32(&mut self) -> Result<u32> {
        let mut data = &self.buf[self.pos..];
        let res = data.read_u32::<BigEndian>()?;
        self.pos += 4;
        Ok(res)
    }

    /// Read a qname
    ///
    /// The tricky part: Reading domain names, taking labels into consideration.
    /// Will take something like [3]www[6]google[3]com[0] and append
    /// www.google.com to outstr.
    pub fn read_qname(&mut self) -> Result<String> {
        let mut parts = vec![];

        // Since we might encounter jumps, we'll keep track of our position
        // locally as opposed to using the position within the struct. This
        // allows us to move the shared position to a point past our current
        // qname, while keeping track of our progress on the current qname
        // using this variable.
        let mut pos = self.pos();

        const MAX_JUMPS: i32 = 5;
        let mut jumps_performed = 0;

        // Our delimiter which we append for each label. Since we don't want a
        // dot at the beginning of the domain name we'll leave it empty for now
        // and set it to "." at the end of the first iteration.

        let mut len = self.get(pos)?;

        while len > 0 && jumps_performed <= MAX_JUMPS {
            // If len has the two most significant bit are set, it represents a
            // jump to some other offset in the packet:
            if (len & 0xC0) == 0xC0 {
                // Update the buffer position to a point past the current
                // label. We don't need to touch it any further.
                if jumps_performed == 0 {
                    self.seek(pos + 2)?;
                }

                // Read another byte, calculate offset and perform the jump by
                // updating our local position variable
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                jumps_performed += 1;
            } else {
                // The base scenario, where we're reading a single label and
                // appending it to the output:
                // Move a single byte forward to move past the length byte.
                pos += 1;

                // Extract the actual ASCII bytes for this label and append them
                // to the output buffer.
                let str_buffer = self.get_range(pos, len as usize)?;
                parts.push(String::from_utf8_lossy(str_buffer).to_lowercase());

                // Move forward the full length of the label.
                pos += len as usize;
            }
            len = self.get(pos)?;
        }

        // Dns Packets are untrusted data, so we need to be paranoid. Someone
        // can craft a packet with a cycle in the jump instructions. This guards
        // against such packets.
        if jumps_performed > MAX_JUMPS {
            return Err(format!("Limit of {} jumps exceeded", MAX_JUMPS).into());
        }
        if jumps_performed == 0 {
            self.seek(pos + 1)?;
        }

        Ok(parts.join("."))
    }

    fn write(&mut self, val: u8) -> Result<()> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        self.buf[self.pos] = val;
        self.pos += 1;
        Ok(())
    }

    pub fn write_exact(&mut self, buf: &[u8]) -> Result<()> {
        println!("slice len = {}", buf.len());
        self.buf[self.pos..self.pos + buf.len()].clone_from_slice(buf);
        self.pos += buf.len();
        Ok(())
    }

    pub fn write_u8(&mut self, val: u8) -> Result<()> {
        self.write(val)?;

        Ok(())
    }

    pub fn write_u16(&mut self, val: u16) -> Result<()> {
        self.write((val >> 8) as u8)?;
        self.write((val & 0xFF) as u8)?;

        Ok(())
    }

    pub fn write_u32(&mut self, val: u32) -> Result<()> {
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write((val & 0xFF) as u8)?;

        Ok(())
    }

    pub fn write_qname(&mut self, qname: &str) -> Result<()> {
        println!("write qname = {}", qname);
        for label in qname.split('.') {
            println!("label = {}", label);
            let len = label.len();
            if len > 0x34 {
                return Err("Single label exceeds 63 characters of length".into());
            }
            println!("len = {}", len);

            self.write_u8(len as u8)?;
            for b in label.as_bytes() {
                self.write_u8(*b)?;
            }
        }

        self.write_u8(0)?;

        Ok(())
    }
}

impl Default for BytePacketBuffer {
    fn default() -> Self {
        Self::new()
    }
}
