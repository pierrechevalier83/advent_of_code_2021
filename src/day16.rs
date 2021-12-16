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
enum OperatorId {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, Eq, PartialEq)]
enum TypeId {
    LiteralValue,
    Operator(OperatorId),
}

impl TypeId {
    fn from_bits(bits: &[bool]) -> Self {
        match from_bits(bits) {
            0 => Self::Operator(OperatorId::Sum),
            1 => Self::Operator(OperatorId::Product),
            2 => Self::Operator(OperatorId::Min),
            3 => Self::Operator(OperatorId::Max),
            4 => Self::LiteralValue,
            5 => Self::Operator(OperatorId::GreaterThan),
            6 => Self::Operator(OperatorId::LessThan),
            7 => Self::Operator(OperatorId::EqualTo),
            _ => panic!("invalid operator"),
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
struct Operator {
    id: OperatorId,
    packets: Vec<Packet>,
}

impl Operator {
    fn calculate(&self) -> u64 {
        let mut values = self.packets.iter().map(|p| p.calculate());
        match self.id {
            OperatorId::Sum => values.sum(),
            OperatorId::Product => values.product(),
            OperatorId::Min => values.min().unwrap_or(0),
            OperatorId::Max => values.max().unwrap_or(0),
            OperatorId::GreaterThan => {
                if values.next() > values.next() {
                    1
                } else {
                    0
                }
            }
            OperatorId::LessThan => {
                if values.next() < values.next() {
                    1
                } else {
                    0
                }
            }
            OperatorId::EqualTo => {
                if values.next() == values.next() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Payload {
    LiteralValue(u64),
    Operator(Operator),
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
            TypeId::Operator(op_id) => {
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
                (
                    Self::Operator(Operator {
                        id: op_id,
                        packets: subpackets,
                    }),
                    index,
                )
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
        self.version.0 as usize
            + match &self.payload {
                Payload::LiteralValue(_) => 0,
                Payload::Operator(op) => op.packets.iter().map(|p| p.version_sum()).sum(),
            }
    }
    fn calculate(&self) -> u64 {
        match &self.payload {
            Payload::LiteralValue(x) => *x,
            Payload::Operator(op) => op.calculate(),
        }
    }
}

#[aoc(day16, part1)]
fn part1(data: &[bool]) -> usize {
    let (packet, _) = Packet::from_bits(data);
    packet.version_sum()
}

#[aoc(day16, part2)]
fn part2(data: &[bool]) -> u64 {
    let (packet, _) = Packet::from_bits(data);
    packet.calculate()
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
                    payload: Payload::Operator(Operator {
                        id: OperatorId::LessThan,
                        packets: vec![
                            Packet {
                                version: Version(6),
                                payload: Payload::LiteralValue(10)
                            },
                            Packet {
                                version: Version(2),
                                payload: Payload::LiteralValue(20)
                            },
                        ]
                    })
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
                    payload: Payload::Operator(Operator {
                        id: OperatorId::Max,
                        packets: vec![
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
                        ]
                    })
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
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&parse_input("C200B40A82")), 3);
        assert_eq!(part2(&parse_input("04005AC33890")), 54);
        assert_eq!(part2(&parse_input("880086C3E88112")), 7);
        assert_eq!(part2(&parse_input("CE00C43D881120")), 9);
        assert_eq!(part2(&parse_input("D8005AC2A8F0")), 1);
        assert_eq!(part2(&parse_input("F600BC2D8F")), 0);
        assert_eq!(part2(&parse_input("9C005AC2F8F0")), 0);
        assert_eq!(part2(&parse_input("9C0141080250320F1802104A08")), 1)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1922490999789)
    }
}
