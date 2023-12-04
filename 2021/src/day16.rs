
#[derive(Debug)]
enum PacketContent {
    LiteralValue{
        v: u64
    },
    Operator {
        packets: Vec<Packet>
    }
}

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    content: PacketContent
}

fn as_bits<'a>(input: &'a str) -> impl Iterator<Item = u8> + 'a {
    input.chars().flat_map(|ch| {
        let half_byte: [u8; 4] = match ch {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => unreachable!()
        };
        half_byte
    })
}

fn decode_number<I: Iterator<Item = u8>>(input: I) -> u64 {
    let mut result: u64 = 0;
    input.for_each(|b| {
        result *= 2;
        result += b as u64;
    });
    return result;
}

fn decode_literal<I: Iterator<Item = u8>>(mut input: I) -> u64 {
    let mut result = 0;
    loop {
        let prefix = input.next().unwrap();
        let half_byte = decode_number(input.by_ref().take(4));
        result *= 16;
        result += half_byte;

        if prefix == 0 {
            break;
        }
    };
    return result;
}

fn decode_packet(input: &mut dyn Iterator<Item = u8>) -> Packet {
    let version = decode_number(input.take(3));
    let type_id = decode_number(input.take(3));
    return match type_id {
        4 => {
            let literal = decode_literal(input);
            Packet {
                version,
                type_id,
                content: PacketContent::LiteralValue {
                    v: literal
                }
            }
        }
        _ => {
            let mode = input.next().unwrap();
            match mode {
                0 => {
                    let subpackets_len = decode_number(input.take(15)) as usize;
                    let mut input = input.take(subpackets_len).peekable();
                    let mut packets = Vec::new();
                    while input.peek().is_some() {
                        packets.push(decode_packet(input.by_ref()));
                    }
                    Packet {
                        version,
                        type_id,
                        content: PacketContent::Operator {
                            packets
                        }
                    }
                }
                1 => {
                    let packets_count = decode_number(input.take(11));
                    let mut packets = Vec::new();
                    for _ in 0..packets_count {
                        packets.push(decode_packet(input));
                    }
                    Packet {
                        version,
                        type_id,
                        content: PacketContent::Operator {
                            packets
                        }
                    }
                }
                _ => unreachable!()
            }
        }
    }
}

fn decode_from_string(input: &str) -> Packet {
    let mut input = as_bits(input);
    decode_packet(&mut input)
}

fn version_sum(packet: &Packet) -> u64 {
    packet.version + match &packet.content {
        PacketContent::LiteralValue { v: _ } => 0,
        PacketContent::Operator { packets } => {
            packets.iter().map(|p| version_sum(p)).sum()
        },
    }
}

fn evaluate(packet: &Packet) -> u64 {
    match &packet.content {
        PacketContent::LiteralValue { v } => *v,
        PacketContent::Operator { packets } => match packet.type_id {
            0 => packets.iter().map(|p| evaluate(p)).sum(),
            1 => packets.iter().map(|p| evaluate(p)).product(),
            2 => packets.iter().map(|p| evaluate(p)).min().unwrap(),
            3 => packets.iter().map(|p| evaluate(p)).max().unwrap(),
            5 => {
                match packets.as_slice() {
                    [p1, p2] => {
                        if evaluate(p1) > evaluate(p2) {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!()
                }
            }
            6 => {
                match packets.as_slice() {
                    [p1, p2] => {
                        if evaluate(p1) < evaluate(p2) {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!()
                }
            }
            7 => {
                match packets.as_slice() {
                    [p1, p2] => {
                        if evaluate(p1) == evaluate(p2) {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }
}

fn main() {
    let input = "620D49005AD2245800D0C9E72BD279CAFB0016B1FA2B1802DC00D0CC611A47FCE2A4ACE1DD144BFABBFACA002FB2C6F33DFF4A0C0119B169B013005F003720004263644384800087C3B8B51C26B449130802D1A0068A5BD7D49DE793A48B5400D8293B1F95C5A3005257B880F5802A00084C788AD0440010F8490F608CACE034401AB4D0F5802726B3392EE2199628CEA007001884005C92015CC8051800130EC0468A01042803B8300D8E200788018C027890088CE0049006028012AB00342A0060801B2EBE400424933980453EFB2ABB36032274C026E4976001237D964FF736AFB56F254CB84CDF136C1007E7EB42298FE713749F973F7283005656F902A004067CD27CC1C00D9CB5FDD4D0014348010C8331C21710021304638C513006E234308B060094BEB76CE3966AA007C6588A5670DC3754395485007A718A7F149CA2DD3B6E7B777800118E7B59C0ECF5AE5D3B6CB1496BAE53B7ADD78C013C00CD2629BF5371D1D4C537EA6E3A3E95A3E180592AC7246B34032CF92804001A1CCF9BA521782ECBD69A98648BC18025800F8C9C37C827CA7BEFB31EADF0AE801BA42B87935B8EF976194EEC426AAF640168CECAF84BC004AE7D1673A6A600B4AB65802D230D35CF81B803D3775683F3A3860087802132FB32F322C92A4C402524F2DE006E8000854378F710C0010D8F30FE224AE428C015E00D40401987F06E3600021D0CE3EC228DA000574E4C3080182931E936E953B200BF656E15400D3496E4A725B92998027C00A84EEEE6B347D30BE60094E537AA73A1D600B880371AA36C3200043235C4C866C018E4963B7E7AA2B379918C639F1550086064BB148BA499EC731004E1AC966BDBC7646600C080370822AC4C1007E38C428BE0008741689D0ECC01197CF216EA16802D3748FE91B25CAF6D5F11C463004E4FD08FAF381F6004D3232CC93E7715B463F780";
    let packet = &decode_from_string(input);
    println!("sum: {:?}", version_sum(packet));
    println!("eval: {:?}", evaluate(packet));
}