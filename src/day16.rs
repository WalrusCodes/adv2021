type Value = u64;

#[derive(Debug)]
enum PacketType {
    Sum,                 // 0
    Product,             // 1
    Minimum,             // 2
    Maximum,             // 3
    LiteralValue(Value), // 4
    GreaterThan,         // 5
    LessThan,            // 6
    EqualTo,             // 7
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    packets: Vec<Packet>,
}

fn read_n_bits(input: &[u8], pos: usize, bits: usize) -> Value {
    let mut out = 0;

    for i in 0..bits {
        let pos2 = pos + i;
        let byte = pos2 / 8;
        let rem = pos2 % 8;
        let bit = ((input[byte] as Value) >> (7 - rem)) & 1;
        out = (out << 1) | bit;
    }
    out
}

fn read_value(input: &[u8], mut pos: usize) -> (Value, usize) {
    let mut out = 0;
    loop {
        let b5 = read_n_bits(input, pos, 5);
        out = (out << 4) | (b5 & 0b1111);
        pos += 5;
        if b5 & 0b10000 == 0 {
            break;
        }
    }
    (out, pos)
}

impl Packet {
    fn parse(input: &[u8], start: usize) -> (Packet, usize) {
        // 3 bits encode the packet version
        // 3 bits encode packet type ID
        //
        // type id 4: literal value
        //   multiple of four bits
        //   1xxxx 1xxxx 0xxxx
        //
        let mut pos = start;

        let version = read_n_bits(input, pos, 3) as u8;
        let type_id = read_n_bits(input, pos + 3, 3);
        pos += 6;

        let mut packets = Vec::new();
        let contents = if type_id == 4 {
            let (value, new_pos) = read_value(input, pos);
            pos = new_pos;
            PacketType::LiteralValue(value)
        } else {
            let length_type = read_n_bits(input, pos, 1);
            pos += 1;
            if length_type == 0 {
                // 15 bit length
                let length = read_n_bits(input, pos, 15) as usize;
                pos += 15;

                let end = pos + length;
                while pos < end {
                    let (pkt, new_pos) = Packet::parse(input, pos);
                    packets.push(pkt);
                    pos = new_pos;
                }
            } else {
                // 11 bit packet count
                let count = read_n_bits(input, pos, 11) as usize;
                pos += 11;

                for _ in 0..count {
                    let (pkt, new_pos) = Packet::parse(input, pos);
                    packets.push(pkt);
                    pos = new_pos;
                }
            }
            match type_id {
                0 => PacketType::Sum,
                1 => PacketType::Product,
                2 => PacketType::Minimum,
                3 => PacketType::Maximum,
                5 => PacketType::GreaterThan,
                6 => PacketType::LessThan,
                7 => PacketType::EqualTo,
                _ => panic!(),
            }
        };
        (
            Packet {
                version,
                packet_type: contents,
                packets,
            },
            pos,
        )
    }

    fn version_sum(&self) -> Value {
        (self.version as Value) + self.packets.iter().map(|p| p.version_sum()).sum::<Value>()
    }

    fn value(&self) -> Value {
        use PacketType::*;

        match self.packet_type {
            Sum => self.packets.iter().map(|p| p.value()).sum(),
            Product => self.packets.iter().map(|p| p.value()).product(),
            Minimum => self.packets.iter().map(|p| p.value()).min().unwrap(),
            Maximum => self.packets.iter().map(|p| p.value()).max().unwrap(),
            LiteralValue(value) => value,
            GreaterThan => {
                assert_eq!(self.packets.len(), 2);
                if self.packets[0].value() > self.packets[1].value() {
                    1
                } else {
                    0
                }
            }
            LessThan => {
                assert_eq!(self.packets.len(), 2);
                if self.packets[0].value() < self.packets[1].value() {
                    1
                } else {
                    0
                }
            }
            EqualTo => {
                assert_eq!(self.packets.len(), 2);
                if self.packets[0].value() == self.packets[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn parse_input(line: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for (i, c) in line.trim().chars().enumerate() {
        let nibble = c.to_digit(16).unwrap() as u8;
        if i % 2 == 0 {
            out.push(nibble << 4);
        } else {
            *out.last_mut().unwrap() |= nibble;
        }
    }

    out
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");
    let input_string = parse_input(&contents);
    let (packet, _) = Packet::parse(&input_string, 0);
    dbg!(packet.version_sum());
    dbg!(packet.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let input = parse_input("D2FE28");
        assert_eq!(read_n_bits(&input, 0, 3), 0b110);
        assert_eq!(read_n_bits(&input, 3, 3), 0b100);
        assert_eq!(read_n_bits(&input, 6, 5), 0b10111);
        assert_eq!(read_n_bits(&input, 11, 5), 0b11110);
        assert_eq!(read_n_bits(&input, 16, 5), 0b00101);
    }

    #[test]
    fn test_literal() {
        let input = parse_input("D2FE28");
        let (packet, pos) = Packet::parse(&input, 0);
        assert_eq!(pos, 21);
        assert!(matches!(packet.packet_type, PacketType::LiteralValue(2021)));
    }

    #[test]
    fn test_operator_with_length() {
        let input = parse_input("38006F45291200");
        let (packet, pos) = Packet::parse(&input, 0);
        assert_eq!(packet.packets.len(), 2);
        assert_eq!(pos, 49);
    }

    #[test]
    fn test_operator_with_count() {
        let input = parse_input("EE00D40C823060");
        let (packet, pos) = Packet::parse(&input, 0);
        assert_eq!(packet.packets.len(), 3);
        assert_eq!(pos, 51);
    }

    #[test]
    fn test_sum() {
        let input = parse_input("A0016C880162017C3686B18A3D4780");
        let (packet, _) = Packet::parse(&input, 0);
        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn test_value() {
        let input = parse_input("9C0141080250320F1802104A08");
        let (packet, _) = Packet::parse(&input, 0);
        assert_eq!(packet.value(), 1);
    }
}
