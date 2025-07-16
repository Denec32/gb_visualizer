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
    pos: usize,
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

    fn get_imm16(&mut self) -> u16 {
        let higher_nimble = self.get_next();
        let lower_nimble = self.get_next();

        (lower_nimble as u16) << 8 | higher_nimble as u16
    }

    fn set_position(&mut self, pos: usize) {
        self.pos = pos;
    }

    fn get_imm8(&mut self) -> u8 {
        self.get_next()
    }
}

fn main() {
    let cartridge_name = "hello-world.gb";
    let content = fs::read(cartridge_name).unwrap();

    let mut file = File::create("hello-world.txt").unwrap();

    let mut cartridge = CartridgeReader::new(content);

    while cartridge.has_next() {
        let opcode = cartridge.get_next();

        let line = if "00000000".is_match(opcode) {
            "nop".to_string()
        } else if "00..0001".is_match(opcode) {
            cartridge.get_next();
            cartridge.get_next();
            "ld r16, imm16".to_string()
        } else if "00..0010".is_match(opcode) {
            "ld [r16mem], a".to_string()
        } else if "00..1010".is_match(opcode) {
            "ld a, [r16mem]".to_string()
        } else if "00001000".is_match(opcode) {
            cartridge.get_next();
            cartridge.get_next();
            "ld [imm16], sp".to_string()
        } else if "00..0011".is_match(opcode) {
            "inc r16".to_string()
        } else if "00..1011".is_match(opcode) {
            "dec r16".to_string()
        } else if "00..1001".is_match(opcode) {
            "add hl, r16".to_string()
        } else if "00...100".is_match(opcode) {
            "inc r8".to_string()
        } else if "00...101".is_match(opcode) {
            "dec r8".to_string()
        } else if "00...110".is_match(opcode) {
            cartridge.get_next();
            "ld r8, imm8".to_string()
        } else if "00000111".is_match(opcode) {
            "rlca".to_string()
        } else if "00001111".is_match(opcode) {
            "rrca".to_string()
        } else if "00010111".is_match(opcode) {
            "rla".to_string()
        } else if "00011111".is_match(opcode) {
            "rra".to_string()
        } else if "00100111".is_match(opcode) {
            "daa".to_string()
        } else if "00101111".is_match(opcode) {
            "cpl".to_string()
        } else if "00110111".is_match(opcode) {
            "scf".to_string()
        } else if "00111111".is_match(opcode) {
            "ccf".to_string()
        } else if "00011000".is_match(opcode) {
            cartridge.get_next();
            "jr imm8".to_string()
        } else if "001..000".is_match(opcode) {
            cartridge.get_next();
            "jr cond, imm8".to_string()
        } else if "00010000".is_match(opcode) {
            "stop".to_string()
        } else if "01110110".is_match(opcode) {
            "halt".to_string()
        } else if "01......".is_match(opcode) {
            "ld r8, r8".to_string()
        } else if "10000...".is_match(opcode) {
            "add a, r8".to_string()
        } else if "10001...".is_match(opcode) {
            "adc a, r8".to_string()
        } else if "10010...".is_match(opcode) {
            "sub a, r8".to_string()
        } else if "10011...".is_match(opcode) {
            "sbc a, r8".to_string()
        } else if "10100...".is_match(opcode) {
            "and a, r8".to_string()
        } else if "10101...".is_match(opcode) {
            "xor a, r8".to_string()
        } else if "10110...".is_match(opcode) {
            "or a, r8".to_string()
        } else if "10111...".is_match(opcode) {
            "cp a, r8".to_string()
        } else if "11000110".is_match(opcode) {
            cartridge.get_next();
            "add a, imm8".to_string()
        } else if "11001110".is_match(opcode) {
            cartridge.get_next();
            "adc a, imm8".to_string()
        } else if "11010110".is_match(opcode) {
            cartridge.get_next();
            "sub a, imm8".to_string()
        } else if "11011110".is_match(opcode) {
            cartridge.get_next();
            "sbc a, imm8".to_string()
        } else if "11100110".is_match(opcode) {
            cartridge.get_next();
            "and a, imm8".to_string()
        } else if "11101110".is_match(opcode) {
            cartridge.get_next();
            "xor a, imm8".to_string()
        } else if "11110110".is_match(opcode) {
            cartridge.get_next();
            "or a, imm8".to_string()
        } else if "11111110".is_match(opcode) {
            cartridge.get_next();
            "cp a, imm8".to_string()
        } else if "110..000".is_match(opcode) {
            "ret cond".to_string()
        } else if "11001001".is_match(opcode) {
            "ret".to_string()
        } else if "11011001".is_match(opcode) {
            "reti".to_string()
        } else if "110..010".is_match(opcode) {
            // jp cond, imm16
            format!("jp cond, [{:x}]", cartridge.get_imm16())
        } else if "11000011".is_match(opcode) {
            // jp imm16
            let jump_target = cartridge.get_imm16();
            cartridge.set_position(jump_target as usize);
            format!("jp [{:x}]", jump_target)
        } else if "11101001".is_match(opcode) {
            "jp hl".to_string()
        } else if "110..100".is_match(opcode) {
            cartridge.get_next();
            cartridge.get_next();
            "call cond, imm16".to_string()
        } else if "11001101".is_match(opcode) {
            cartridge.get_next();
            cartridge.get_next();
            "call imm16".to_string()
        } else if "11...111".is_match(opcode) {
            "rst tgt3".to_string()
        } else if "11..0001".is_match(opcode) {
            "pop r16stk".to_string()
        } else if "11..0101".is_match(opcode) {
            "push r16stk".to_string()
        } else if "11001011".is_match(opcode) {
            "Prefix (see block below)".to_string()
        } else if "11100010".is_match(opcode) {
            "ldh [c], a".to_string()
        } else if "11100000".is_match(opcode) {
            cartridge.get_next();
            "ldh [imm8], a".to_string()
        } else if "11101010".is_match(opcode) {
            cartridge.get_next();
            cartridge.get_next();
            "ld [imm16], a".to_string()
        } else if "11110010".is_match(opcode) {
            "ldh a, [c]".to_string()
        } else if "11110000".is_match(opcode) {
            cartridge.get_next();
            "ldh a, [imm8]".to_string()
        } else if "11111010".is_match(opcode) {
            cartridge.get_next();
            cartridge.get_next();
            "ld a, [imm16]".to_string()
        } else if "11101000".is_match(opcode) {
            cartridge.get_next();
            "add sp, imm8".to_string()
        } else if "11111000".is_match(opcode) {
            cartridge.get_next();
            "ld hl, sp + imm8".to_string()
        } else if "11111001".is_match(opcode) {
            "ld sp, hl".to_string()
        } else if "11110011".is_match(opcode) {
            "di".to_string()
        } else if "11111011".is_match(opcode) {
            "ei".to_string()
        } else {
            panic!("Unknown opcode: {}", opcode)
        };

        file.write(line.to_uppercase().as_bytes()).unwrap();
        file.write("\n".as_bytes()).unwrap();
    }
}
