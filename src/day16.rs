use aoc_runner_derive::{aoc, aoc_generator};

fn to_bits(n: usize, i: u32) -> Vec<bool> {
    let v = (0..n).map(|index| ((i >> index) & 1) == 1).rev().collect();
    v
}

fn from_bits(bits: &[bool]) -> u64 {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(index, &bit)| if bit { 1 << index } else { 0 })
        .sum()
}

#[aoc_generator(day16)]
fn parse_input(data: &str) -> Vec<bool> {
    data.trim()
        .chars()
        .map(|c| char::to_digit(c, 16).unwrap())
        .flat_map(|i| to_bits(4, i))
        .collect::<Vec<_>>()
}

#[derive(Debug, Eq, PartialEq)]
struct Version(u8);

impl Version {
    fn from_bits(bits: &[bool]) -> Self {
        Self(from_bits(bits) as u8)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TypeId {
    LiteralValue,
    Operator,
}

impl TypeId {
    fn from_bits(bits: &[bool]) -> Self {
        match from_bits(bits) {
            4 => Self::LiteralValue,
            _ => Self::Operator,
        }
    }
}

enum LengthTypeId {
    TotalLength(usize),
    NumSubPackets(usize),
}

impl LengthTypeId {
    fn from_bit(bit: bool) -> Self {
        if bit {
            Self::NumSubPackets(11)
        } else {
            Self::TotalLength(15)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Payload {
    LiteralValue(u64),
    Operator(Vec<Packet>),
}

impl Payload {
    fn from_bits(bits: &[bool]) -> (Self, usize) {
        let type_id = TypeId::from_bits(&bits[..3]);
        let mut index = 3;
        match type_id {
            TypeId::LiteralValue => {
                let mut value_bits = Vec::new();
                while bits[index] {
                    value_bits.extend_from_slice(&bits[index + 1..index + 5]);
                    index += 5;
                }
                // Parse the final value bits that start with 0
                value_bits.extend_from_slice(&bits[index + 1..index + 5]);
                index += 5;
                (Self::LiteralValue(from_bits(&value_bits)), index)
            }
            TypeId::Operator => {
                let length_type_id = LengthTypeId::from_bit(bits[index]);
                index += 1;
                let mut subpackets = Vec::new();
                match length_type_id {
                    LengthTypeId::TotalLength(n_bits) => {
                        let length = from_bits(&bits[index..index + n_bits]) as usize;
                        index += n_bits;
                        let previous_index = index;
                        while index < length + previous_index {
                            let (subpacket, next_index) = Packet::from_bits(&bits[index..]);
                            subpackets.push(subpacket);
                            index += next_index;
                        }
                    }
                    LengthTypeId::NumSubPackets(n_bits) => {
                        let num_subpackets = from_bits(&bits[index..index + n_bits]) as usize;
                        index += n_bits;
                        while subpackets.len() < num_subpackets {
                            let (subpacket, next_index) = Packet::from_bits(&bits[index..]);
                            subpackets.push(subpacket);
                            index += next_index;
                        }
                    }
                }
                (Self::Operator(subpackets), index)
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: Version,
    payload: Payload,
}

impl Packet {
    fn from_bits(bits: &[bool]) -> (Self, usize) {
        let version = Version::from_bits(&bits[..3]);
        let (payload, index) = Payload::from_bits(&bits[3..]);
        (Self { version, payload }, 3 + index)
    }
    fn version_sum(&self) -> usize {

        let sum = self.version.0 as usize + match &self.payload {
            Payload::LiteralValue(_) => 0,
            Payload::Operator(packets) => packets.iter().map(|p| p.version_sum()).sum()
        };
        sum
    }
}

#[aoc(day16, part1)]
fn part1(data: &[bool]) -> usize {
    let (packet, _) = Packet::from_bits(data);
    //println!("{:#?}", packet);
    packet.version_sum()
}

#[aoc(day16, part2)]
fn part2(data: &[bool]) -> usize {
    42
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<bool> {
        parse_input(include_str!("../input/2021/day16.txt"))
    }
    use super::*;
    #[test]
    fn test_decode_literal_value_packet() {
        assert_eq!(
            (
                Packet {
                    version: Version(6),
                    payload: Payload::LiteralValue(2021)
                },
                21
            ),
            Packet::from_bits(&parse_input("D2FE28"))
        );
    }
    #[test]
    fn test_decode_operator_packet_with_total_length() {
        assert_eq!(
            (
                Packet {
                    version: Version(1),
                    payload: Payload::Operator(vec![
                        Packet {
                            version: Version(6),
                            payload: Payload::LiteralValue(10)
                        },
                        Packet {
                            version: Version(2),
                            payload: Payload::LiteralValue(20)
                        },
                    ])
                },
                49
            ),
            Packet::from_bits(&parse_input("38006F45291200"))
        );
    }
    #[test]
    fn test_decode_operator_packet_with_num_subpackets() {
        assert_eq!(
            (
                Packet {
                    version: Version(7),
                    payload: Payload::Operator(vec![
                        Packet {
                            version: Version(2),
                            payload: Payload::LiteralValue(1)
                        },
                        Packet {
                            version: Version(4),
                            payload: Payload::LiteralValue(2)
                        },
                        Packet {
                            version: Version(1),
                            payload: Payload::LiteralValue(3)
                        },
                    ])
                },
                51
            ),
            Packet::from_bits(&parse_input("EE00D40C823060"))
        );
    }
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&parse_input("8A004A801A8002F478")), 16);
        assert_eq!(part1(&parse_input("620080001611562C8802118E34")), 12);
        assert_eq!(part1(&parse_input("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part1(&parse_input("A0016C880162017C3686B18A3D4780")), 31)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1014)
    }
    /*
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 0)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 0)
    }
    */
}
