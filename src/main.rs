use std::collections::{BTreeMap, VecDeque};
use std::fs;

trait InstructionMatcher {
    fn is_match(&self, opcode: u8) -> bool;
}

impl InstructionMatcher for str {
    fn is_match(&self, instruction: u8) -> bool {
        for (idx, bit) in self.chars().enumerate() {
            if bit == '.' {
                continue;
            }

            let other_bit = if (instruction & (1 << (7 - idx))) != 0 {
                '1'
            } else {
                '0'
            };
            if other_bit != bit {
                return false;
            }
        }

        true
    }
}
struct CartridgeReader {
    data: Vec<u8>,
}

impl CartridgeReader {
    fn new(data: Vec<u8>) -> CartridgeReader {
        CartridgeReader { data }
    }

    fn read_instruction(&mut self, pos: usize) -> u8 {
        self.data[pos]
    }

    fn read_imm8(&mut self, pos: usize) -> u8 {
        self.data[pos + 1]
    }

    fn read_imm16(&mut self, pos: usize) -> u16 {
        let higher_nimble = self.data[pos + 1];
        let lower_nimble = self.data[pos + 2];

        (lower_nimble as u16) << 8 | higher_nimble as u16
    }

    fn decode_r8(&self, r8: u8) -> String {
        match r8 {
            0 => String::from("B"),
            1 => String::from("C"),
            2 => String::from("D"),
            3 => String::from("E"),
            4 => String::from("H"),
            5 => String::from("L"),
            6 => String::from("HL"),
            7 => String::from("A"),
            _ => panic!("Invalid register code: {r8}")
        }
    }

    fn decode_r16(&self, r16: u8) -> String {
        match r16 {
            0 => String::from("BC"),
            1 => String::from("DE"),
            2 => String::from("HL"),
            3 => String::from("SP"),
            _ => panic!("Invalid register code: {r16}")
        }
    }

    fn decode_r16mem(&self, r16mem: u8) -> String {
        match r16mem {
            0 => String::from("BC"),
            1 => String::from("DE"),
            2 => String::from("HL+"),
            3 => String::from("HL-"),
            _ => panic!("Invalid register code: {r16mem}")
        }
    }

    fn decode_condition(&self, cond: u8) -> String {
        match cond {
            0 => String::from("ZERO FLAG NOT SET"),
            1 => String::from("ZERO FLAG SET"),
            2 => String::from("CARRY FLAG NOT SET"),
            3 => String::from("CARRY FLAG SET"),
            _ => panic!("Invalid condition code: {cond}")
        }
    }

    fn decode_memory_address(&self, address: u16) -> String {
        match address {
            0x90 => String::from("CARTRIDGE ROM"),
            0xFF26 => String::from("NR52 ACU"),
            0xFF40 => String::from("LCD Control"),
            0xFF44 => String::from("LY"),
            0xFF47 => String::from("BGP"),
            _ => panic!("Invalid memory address: {address}")
        }
    }
}

fn main() {
    let cartridge_name = "hello-world.gb";
    let content = fs::read(cartridge_name).unwrap();
    let mut cartridge = CartridgeReader::new(content);

    let mut visited = BTreeMap::new();

    let mut queue = VecDeque::new();
    queue.push_front(0x100usize);

    while !queue.is_empty() {
        let pos = queue.pop_back().unwrap();
        if visited.contains_key(&pos) {
            continue;
        }

        let opcode = cartridge.read_instruction(pos);

        if "00000000".is_match(opcode) {
            // nop
            queue.push_front(pos);
            visited.insert(pos, "nop".to_string());
        } else if "00..0001".is_match(opcode) {
            //ld r16, imm16
            let imm16 = cartridge.read_imm16(pos);
            let r16 = cartridge.decode_r16(opcode >> 4);
            queue.push_front(pos + 3);
            visited.insert(pos, format!("LD {r16}, {:#X}", imm16));
        } else if "00..0010".is_match(opcode) {
            //ld [r16mem], a
            queue.push_front(pos + 1);
            let r16mem = cartridge.decode_r16mem(opcode >> 4);
            visited.insert(pos, format!("LD [{r16mem}], A"));
        } else if "00..1010".is_match(opcode) {
            //ld a, [r16mem]
            queue.push_front(pos + 1);
            let r16mem = cartridge.decode_r16mem(opcode >> 4);
            visited.insert(pos, format!("LD A, [{r16mem}]"));
        } else if "00001000".is_match(opcode) {
            //ld [imm16], sp
            let imm16 = cartridge.read_imm16(pos);
            queue.push_front(pos + 3);
            visited.insert(pos, format!("ld [{:x}], sp", imm16));
        } else if "00..0011".is_match(opcode) {
            //inc r16
            queue.push_front(pos + 1);
            let r16 = cartridge.decode_r16(opcode >> 4);
            visited.insert(pos, format!("INC {r16}"));
        } else if "00..1011".is_match(opcode) {
            //dec r16
            queue.push_front(pos + 1);
            let r16 = cartridge.decode_r16(opcode >> 4);
            visited.insert(pos, format!("DEC {r16}"));
        } else if "00..1001".is_match(opcode) {
            //add hl, r16
            queue.push_front(pos + 1);
            visited.insert(pos, "hl, r16".to_string());
        } else if "00...100".is_match(opcode) {
            //inc r8
            queue.push_front(pos + 1);
            visited.insert(pos, "inc r8".to_string());
        } else if "00...101".is_match(opcode) {
            //dec r8
            queue.push_front(pos + 1);
            visited.insert(pos, "dec r8".to_string());
        } else if "00...110".is_match(opcode) {
            //"ld r8, imm8"
            let r8 = cartridge.decode_r8(opcode >> 3);
            let imm8 = cartridge.read_imm8(pos);
            queue.push_front(pos + 2);
            visited.insert(pos, format!("LD {r8}, {:#X}", imm8));
        } else if "00000111".is_match(opcode) {
            //rlca
            queue.push_front(pos + 1);
            visited.insert(pos, "rlca".to_string());
        } else if "00001111".is_match(opcode) {
            //rrca
            queue.push_front(pos + 1);
            visited.insert(pos, "rrca".to_string());
        } else if "00010111".is_match(opcode) {
            //rla
            queue.push_front(pos + 1);
            visited.insert(pos, "rla".to_string());
        } else if "00011111".is_match(opcode) {
            //rra
            queue.push_front(pos + 1);
            visited.insert(pos, "rra".to_string());
        } else if "00100111".is_match(opcode) {
            //daa
            queue.push_front(pos + 1);
            visited.insert(pos, "daa".to_string());
        } else if "00101111".is_match(opcode) {
            //cpl
            queue.push_front(pos + 1);
            visited.insert(pos, "cpl".to_string());
        } else if "00110111".is_match(opcode) {
            //scf
            queue.push_front(pos + 1);
            visited.insert(pos, "scf".to_string());
        } else if "00111111".is_match(opcode) {
            //ccf
            queue.push_front(pos + 1);
            visited.insert(pos, "ccf".to_string());
        } else if "00011000".is_match(opcode) {
            //jr imm8
            panic!("unimplemented");
        } else if "001..000".is_match(opcode) {
            //cartridge.get_next();
            //"jr cond, imm8".to_string()
            panic!("unimplemented");
        } else if "00010000".is_match(opcode) {
            //"stop".to_string()
            panic!("unimplemented");
        } else if "01110110".is_match(opcode) {
            //"halt".to_string()
            panic!("unimplemented");
        } else if "01......".is_match(opcode) {
            //ld r8, r8
            let r8_1 = cartridge.decode_r8((0b00111000 & opcode) >> 4);
            let r8_2 = cartridge.decode_r8(0b00000111 & opcode);
            queue.push_front(pos + 1);
            visited.insert(pos, format!("LD {r8_1}, {r8_2}"));
        } else if "10000...".is_match(opcode) {
            //add a, r8
            queue.push_front(pos + 1);
            visited.insert(pos, "add a, r8".to_string());
        } else if "10001...".is_match(opcode) {
            //adc a, r8
            queue.push_front(pos + 1);
            visited.insert(pos, "adc a, r8".to_string());
        } else if "10010...".is_match(opcode) {
            //sub a, r8
            queue.push_front(pos + 1);
            visited.insert(pos, "sub a, r8".to_string());
        } else if "10011...".is_match(opcode) {
            //sbc a, r8
            queue.push_front(pos + 1);
            visited.insert(pos, "sbc a, r8".to_string());
        } else if "10100...".is_match(opcode) {
            //and a, r8
            queue.push_front(pos + 1);
            visited.insert(pos, "and a, r8".to_string());
        } else if "10101...".is_match(opcode) {
            //xor a, r8
            queue.push_front(pos + 1);
            visited.insert(pos, "xor a, r8".to_string());
        } else if "10110...".is_match(opcode) {
            //or a, r8
            queue.push_front(pos + 1);
            let r8 = cartridge.decode_r8(opcode & 0b00000111);
            visited.insert(pos, format!("OR A, {r8}"));
        } else if "10111...".is_match(opcode) {
            //cp a, r8
            queue.push_front(pos + 1);
            visited.insert(pos, "cp a, r8".to_string());
        } else if "11000110".is_match(opcode) {
            //cartridge.get_next();
            //"add a, imm8".to_string()
            panic!("unimplemented");
        } else if "11001110".is_match(opcode) {
            //cartridge.get_next();
            //"adc a, imm8".to_string()
            panic!("unimplemented");
        } else if "11010110".is_match(opcode) {
            //cartridge.get_next();
            //"sub a, imm8".to_string()
            panic!("unimplemented");
        } else if "11011110".is_match(opcode) {
            //cartridge.get_next();
            //"sbc a, imm8".to_string()
            panic!("unimplemented");
        } else if "11100110".is_match(opcode) {
            //cartridge.get_next();
            //"and a, imm8".to_string()
            panic!("unimplemented");
        } else if "11101110".is_match(opcode) {
            //cartridge.get_next();
            //"xor a, imm8".to_string()
            panic!("unimplemented");
        } else if "11110110".is_match(opcode) {
            //cartridge.get_next();
            //"or a, imm8".to_string()
            panic!("unimplemented");
        } else if "11111110".is_match(opcode) {
            //cp a, imm8
            let imm8 = cartridge.read_imm8(pos);
            queue.push_front(pos + 2);
            visited.insert(pos, format!("CP A, {:#X}", imm8));
        } else if "110..000".is_match(opcode) {
            //"ret cond".to_string()
            panic!("unimplemented");
        } else if "11001001".is_match(opcode) {
            //"ret".to_string()
            panic!("unimplemented");
        } else if "11011001".is_match(opcode) {
            //"reti".to_string()
            panic!("unimplemented");
        } else if "110..010".is_match(opcode) {
            // jp cond, imm16
            let imm16 = cartridge.read_imm16(pos);
            let cond = cartridge.decode_condition((opcode & 0b00011000) >> 3);
            queue.push_front(pos + 3);
            queue.push_front(imm16 as usize);
            visited.insert(pos, format!("JP [{cond}], {imm16}"));
        } else if "11000011".is_match(opcode) {
            // jp imm16
            let imm16 = cartridge.read_imm16(pos);
            queue.push_front(imm16 as usize);
            visited.insert(pos, format!("JP {imm16}"));
        } else if "11101001".is_match(opcode) {
            //"jp hl".to_string()
            panic!("unimplemented");
        } else if "110..100".is_match(opcode) {
            //cartridge.get_next();
            //cartridge.get_next();
            //"call cond, imm16".to_string()
            panic!("unimplemented");
        } else if "11001101".is_match(opcode) {
            //cartridge.get_next();
            //cartridge.get_next();
            //"call imm16".to_string()
            panic!("unimplemented");
        } else if "11...111".is_match(opcode) {
            //"rst tgt3".to_string()
            panic!("unimplemented");
        } else if "11..0001".is_match(opcode) {
            //"pop r16stk".to_string()
            panic!("unimplemented");
        } else if "11..0101".is_match(opcode) {
            //"push r16stk".to_string()
            panic!("unimplemented");
        } else if "11001011".is_match(opcode) {
            //"Prefix (see block below)".to_string()
            panic!("unimplemented");
        } else if "11100010".is_match(opcode) {
            //"ldh [c], a".to_string()
            panic!("unimplemented");
        } else if "11100000".is_match(opcode) {
            //cartridge.get_next();
            //"ldh [imm8], a".to_string()
            panic!("unimplemented");
        } else if "11101010".is_match(opcode) {
            //ld [imm16], a
            let imm16 = cartridge.read_imm16(pos);
            queue.push_front(pos + 3);
            visited.insert(pos, format!("LD [m:{}], A", cartridge.decode_memory_address(imm16)));
        } else if "11110010".is_match(opcode) {
            //"ldh a, [c]".to_string()
            panic!("unimplemented");
        } else if "11110000".is_match(opcode) {
            //cartridge.get_next();
            //"ldh a, [imm8]".to_string()
            panic!("unimplemented");
        } else if "11111010".is_match(opcode) {
            //ld a, [imm16]
            let imm16 = cartridge.read_imm16(pos);
            queue.push_front(pos + 3);
            visited.insert(pos, format!("LD A, [m:{}]", cartridge.decode_memory_address(imm16)));
        } else if "11101000".is_match(opcode) {
            //cartridge.get_next();
            //"add sp, imm8".to_string()
            panic!("unimplemented");
        } else if "11111000".is_match(opcode) {
            //cartridge.get_next();
            //"ld hl, sp + imm8".to_string()
            panic!("unimplemented");
        } else if "11111001".is_match(opcode) {
            //"ld sp, hl".to_string()
            panic!("unimplemented");
        } else if "11110011".is_match(opcode) {
            //"di".to_string()
            panic!("unimplemented");
        } else if "11111011".is_match(opcode) {
            //"ei".to_string()
            panic!("unimplemented");
        } else {
            panic!("Unknown opcode: {}", opcode)
        };

    }

    for entry in &visited {
        println!("{}: {}", entry.0, entry.1);
    }
}
