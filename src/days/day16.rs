use std::fs::File;
use std::io::{prelude::*, BufReader};

enum QVariant {
    Part1,
    Part2,
}

#[derive(Debug, Hash, Copy, Clone)]
struct Point(usize, usize);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    value: u64,
    length_type_id: Option<u8>,
    sub_packets: Vec<Packet>,
    total_bits_read: u64,
}

/*
  Out of convenience, we're gonna use strings to represent binary numbers
*/
fn hex_to_binary<'a>(c: char) -> &'a str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Invalid hex character"),
    }
}

fn binary_to_decimal(binary_str: &str) -> u64 {
    let len = binary_str.len();
    let mut value: u64 = 0;

    for i in 0..len {
        let position = len - i - 1;
        if binary_str.chars().nth(i).unwrap() == '1' {
            value += 1 << position;
        }
    }

    return value;
}

fn read_total_length_sub_packets(binary: &str) -> Vec<Packet> {
    let mut sub_packets = vec![];

    let total_length_bits = binary_to_decimal(&binary[0..15]);
    let mut cur_binary = &binary[15..];

    let mut next_total_length_bits = total_length_bits;
    loop {
        let next_packet = parse_packet(&cur_binary);
        let total_bits_read = next_packet.total_bits_read;
        next_total_length_bits -= total_bits_read;
        sub_packets.push(next_packet);

        if next_total_length_bits == 0 {
            break;
        }

        cur_binary = &cur_binary[total_bits_read as usize..];
    }

    return sub_packets;
}

fn read_num_packet_sub_packets(binary: &str) -> Vec<Packet> {
    let mut sub_packets = vec![];

    let num_sub_packets = binary_to_decimal(&binary[0..11]);
    let mut cur_binary = &binary[11..];

    for _ in 0..num_sub_packets {
        let next_packet = parse_packet(&cur_binary);
        cur_binary = &cur_binary[next_packet.total_bits_read as usize..];
        sub_packets.push(next_packet);
    }

    return sub_packets;
}

fn parse_packet(binary: &str) -> Packet {
    /*
        1. First 3 bits - Version
        2. Next 3 bits - Type
        3. If type is literal, then we read groups of 5 bits
          - If starts with 1, then not the end. Read next 3
          - If starts with 0, then end
          - Rest is junk

        Type - operator
        - May contain subpackets
        - weird rules.
          - Bit after header is length id
            - 0 means total length in bits. Read as a 15 bit number
            - 1 is the next 11 bits. Represents number of sub packets.
    */

    let version = binary_to_decimal(&binary[0..3]) as u8;
    let type_id = binary_to_decimal(&binary[3..6]) as u8;

    match type_id {
        4 => {
            // literal
            let cur_binary = &binary[6..];
            let mut pointer = 0;
            let mut binary_groups: Vec<&str> = vec![];
            while pointer + 5 <= cur_binary.len() {
                let next_pointer = pointer + 5;
                let group = &cur_binary[pointer..next_pointer];
                let first_bit = group.chars().nth(0).unwrap();

                binary_groups.push(&group[1..]);
                pointer = next_pointer;
                if first_bit == '0' {
                    break;
                }
            }

            let binary_literal_value = &binary_groups.join("");
            let literal_value = binary_to_decimal(binary_literal_value);

            Packet {
                version,
                type_id,
                value: literal_value,
                length_type_id: None,
                sub_packets: vec![],
                // Add the value of the literal, plus the extra bit we discarded
                total_bits_read: 6 + (binary_literal_value.len() + binary_groups.len()) as u64,
            }
        }
        _ => {
            let cur_binary = &binary[6..];
            let length_id = if cur_binary.chars().nth(0).unwrap() == '0' {
                0
            } else {
                1
            };

            let mut total_bits_read = 7;

            let sub_packets: Vec<Packet> = match length_id {
                0 => {
                    total_bits_read += 15;
                    read_total_length_sub_packets(&cur_binary[1..])
                }
                1 => {
                    total_bits_read += 11;
                    read_num_packet_sub_packets(&cur_binary[1..])
                }
                _ => panic!("Expected 0 or 1 for length id"),
            };

            total_bits_read += sub_packets.iter().map(|s| s.total_bits_read).sum::<u64>();

            let value = match type_id {
                0 => sub_packets.iter().map(|sub| sub.value).sum(),
                1 => sub_packets.iter().map(|sub| sub.value).product(),
                2 => sub_packets.iter().map(|sub| sub.value).min().unwrap(),
                3 => sub_packets.iter().map(|sub| sub.value).max().unwrap(),
                5 => {
                    if sub_packets[0].value > sub_packets[1].value {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if sub_packets[0].value < sub_packets[1].value {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if sub_packets[0].value == sub_packets[1].value {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("No other type_id should show here"),
            };

            Packet {
                version,
                type_id,
                value,
                length_type_id: Some(length_id),
                sub_packets,
                total_bits_read,
            }
        }
    }
}

fn parse_input(input: &str) -> Packet {
    let binary: String = input
        .chars()
        .map(|c| hex_to_binary(c))
        .collect::<Vec<&str>>()
        .join("");
    parse_packet(&binary)
}

fn read_version_total(packet: &Packet) -> u32 {
    let mut current_total: u32 = packet.version as u32;

    for sub_packet in &packet.sub_packets {
        current_total += read_version_total(&sub_packet);
    }

    current_total as u32
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day16/input.txt").unwrap();
    let reader = BufReader::new(file);

    let packet = parse_input(&reader.lines().next().unwrap().unwrap());

    let version_total = read_version_total(&packet);

    match variant {
        QVariant::Part1 => {
            println!("Answer - {}", version_total);
        }
        QVariant::Part2 => {
            println!("Answer - {}", packet.value);
        }
    }
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
