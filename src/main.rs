use std::fs;
use std::fs::File;
use std::io::Write;

trait InstructionMatcher {
    fn is_match(&self, opcode: u8) -> bool;
}

impl InstructionMatcher for str {
    fn is_match(&self, instruction: u8) -> bool {
        for (idx, bit) in self.chars().enumerate() {
            if bit == '.' {
                continue
            }

            let other_bit = if (instruction & (1 << (7 - idx))) != 0 { '1' } else { '0' };
            if other_bit != bit {
                return false
            }
        }

        true
    }
}
struct CartridgeReader {
    data: Vec<u8>,
    pos: usize
}

impl CartridgeReader {
    fn new(data: Vec<u8>) -> CartridgeReader {
        CartridgeReader { data, pos: 0x100 }
    }

    fn has_next(&self) -> bool {
        self.pos < self.data.len()
    }

    fn get_next(&mut self) -> u8 {
        let opcode = self.data[self.pos];
        self.pos += 1;
        opcode
    }
}

fn get_block(opcode: u8) -> u8 {
    opcode >> 6
}

fn main() {
    let cartridge_name = "hello-world.gb";
    let content = fs::read(cartridge_name).unwrap();

    let mut file = File::create("hello-world.txt").unwrap();

    let mut cartridge = CartridgeReader::new(content);

    while cartridge.has_next() {
        let opcode = cartridge.get_next();
        let line = match get_block(opcode) {
            0 => parse_block_zero(&mut cartridge, opcode),
            1 => parse_block_one(opcode),
            2 => parse_block_two(opcode),
            3 => parse_block_three(&mut cartridge, opcode),
            _ => panic!("wrong block number")
        };

        file.write(line.as_bytes()).unwrap();
        file.write("\n".as_bytes()).unwrap();
    }
}

fn parse_block_zero(cartridge: &mut CartridgeReader, opcode: u8) -> String {
    if "00000000".is_match(opcode) {
        return "nop".to_string()
    } else if "00..0001".is_match(opcode) {
        cartridge.get_next();
        cartridge.get_next();
        return "ld r16, imm16".to_string()
    } else if "00..0010".is_match(opcode) {
        return "ld [r16mem], a".to_string()
    } else if "00..1010".is_match(opcode) {
        return "ld a, [r16mem]".to_string()
    } else if "00001000".is_match(opcode) {
        cartridge.get_next();
        cartridge.get_next();
        return "ld [imm16], sp".to_string()
    } else if "00..0011".is_match(opcode) {
        return "inc r16".to_string()
    } else if "00..1011".is_match(opcode) {
        return "dec r16".to_string()
    } else if "00..1001".is_match(opcode) {
        return "add hl, r16".to_string()
    } else if "00...100".is_match(opcode) {
        return "inc r8".to_string()
    } else if "00...101".is_match(opcode) {
        return "dec r8".to_string()
    } else if "00...110".is_match(opcode) {
        cartridge.get_next();
        return "ld r8, imm8".to_string()
    } else if "00000111".is_match(opcode) {
        return "rlca".to_string()
    } else if "00001111".is_match(opcode) {
        return "rrca".to_string()
    } else if "00010111".is_match(opcode) {
        return "rla".to_string()
    } else if "00011111".is_match(opcode) {
        return "rra".to_string()
    } else if "00100111".is_match(opcode) {
        return "daa".to_string()
    } else if "00101111".is_match(opcode) {
        return "cpl".to_string()
    } else if "00110111".is_match(opcode) {
        return "scf".to_string()
    } else if "00111111".is_match(opcode) {
        return "ccf".to_string()
    } else if "00011000".is_match(opcode) {
        cartridge.get_next();
        return "jr imm8".to_string()
    } else if "001..000".is_match(opcode) {
        cartridge.get_next();
        return "jr cond, imm8".to_string()
    } else if "00010000".is_match(opcode) {
        return "stop".to_string()
    }

    panic!("unrecognized block opcode: {:x}", opcode);
}

fn parse_block_one(opcode: u8) -> String {
    if "01110110".is_match(opcode) {
        return "halt".to_string()
    } else if "01......".is_match(opcode) {
        return "ld r8, r8".to_string()
    }
    panic!("unrecognized block opcode: {:b}", opcode);
}

fn parse_block_two(opcode: u8) -> String {
    if "10000...".is_match(opcode) {
        return "add a, r8".to_string()
    } else if "10001...".is_match(opcode) {
        return "adc a, r8".to_string()
    } else if "10010...".is_match(opcode) {
        return "sub a, r8".to_string()
    } else if "10011...".is_match(opcode) {
        return "sbc a, r8".to_string()
    } else if "10100...".is_match(opcode) {
        return "and a, r8".to_string()
    } else if "10101...".is_match(opcode) {
        return "xor a, r8".to_string()
    } else if "10110...".is_match(opcode) {
        return "or a, r8".to_string()
    } else if "10111...".is_match(opcode) {
        return "cp a, r8".to_string()
    }
    panic!("unrecognized block opcode: {:b}", opcode);
}

fn parse_block_three(cartridge: &mut CartridgeReader, opcode: u8) -> String {
    if "11000110".is_match(opcode) {
        cartridge.get_next();
        return "add a, imm8".to_string();
    } else if "11001110".is_match(opcode) {
        cartridge.get_next();
        return "adc a, imm8".to_string();
    } else if "11010110".is_match(opcode) {
        cartridge.get_next();
        return "sub a, imm8".to_string();
    } else if "11011110".is_match(opcode) {
        cartridge.get_next();
        return "sbc a, imm8".to_string();
    } else if "11100110".is_match(opcode) {
        cartridge.get_next();
        return "and a, imm8".to_string();
    } else if "11101110".is_match(opcode) {
        cartridge.get_next();
        return "xor a, imm8".to_string();
    } else if "11110110".is_match(opcode) {
        cartridge.get_next();
        return "or a, imm8".to_string();
    } else if "11111110".is_match(opcode) {
        cartridge.get_next();
        return "cp a, imm8".to_string();
    } else if "110..000".is_match(opcode) {
        return "ret cond".to_string()
    } else if "11001001".is_match(opcode) {
        return "ret".to_string()
    } else if "11011001".is_match(opcode) {
        return "reti".to_string()
    } else if "110..010".is_match(opcode) {
        cartridge.get_next();
        cartridge.get_next();
        return "jp cond, imm16".to_string()
    } else if "11000011".is_match(opcode) {
        cartridge.get_next();
        cartridge.get_next();
        return "jp imm16".to_string()
    } else if "11101001".is_match(opcode) {
        return "jp hl".to_string()
    } else if "110..100".is_match(opcode) {
        cartridge.get_next();
        cartridge.get_next();
        return "call cond, imm16".to_string()
    } else if "11001101".is_match(opcode) {
        cartridge.get_next();
        cartridge.get_next();
        return "call imm16".to_string()
    } else if "11...111".is_match(opcode) {
        return "rst tgt3".to_string()
    } else if "11..0001".is_match(opcode) {
        return "pop r16stk".to_string();
    } else if "11..0101".is_match(opcode) {
        return "push r16stk".to_string();
    } else if "11001011".is_match(opcode) {
        return "Prefix (see block below)".to_string();
    } else if "11100010".is_match(opcode) {
        return "ldh [c], a".to_string();
    } else if "11100000".is_match(opcode) {
        cartridge.get_next();
        return "ldh [imm8], a".to_string();
    } else if "11101010".is_match(opcode) {
        cartridge.get_next();
        cartridge.get_next();
        return "ld [imm16], a".to_string();
    } else if "11110010".is_match(opcode) {
        return "ldh a, [c]".to_string();
    } else if "11110000".is_match(opcode) {
        cartridge.get_next();
        return "ldh a, [imm8]".to_string();
    } else if "11111010".is_match(opcode) {
        cartridge.get_next();
        cartridge.get_next();
        return "ld a, [imm16]".to_string();
    } else if "11101000".is_match(opcode) {
        cartridge.get_next();
        return "add sp, imm8".to_string();
    } else if "11111000".is_match(opcode) {
        cartridge.get_next();
        return "ld hl, sp + imm8".to_string();
    } else if "11111001".is_match(opcode) {
        return "ld sp, hl".to_string();
    } else if "11110011".is_match(opcode) {
        return "di".to_string();
    } else if "11111011".is_match(opcode) {
        return "ei".to_string();
    }

    panic!("unrecognized block opcode: {:b}", opcode);
}
