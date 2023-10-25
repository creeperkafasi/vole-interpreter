use std::{env::args, fs, mem::transmute};

fn main() {
    let mut memory = vec![0; 256];
    let mut registers: [u32; 16] = [0; 16];

    let filename: String;
    match args().nth(1) {
        Some(s) => filename = s.to_string(),
        None => panic!("Please enter a valid file name to run."),
    };

    match fs::read_to_string(filename) {
        Ok(contents) => {
            let program = contents
                .split("\n")
                .filter(|line| line.len() != 0)
                .filter(|line| !line.starts_with("//"))
                .map(|line| {
                    line[2..6].chars().map(|c| match c {
                        '0' => 0x0,
                        '1' => 0x1,
                        '2' => 0x2,
                        '3' => 0x3,
                        '4' => 0x4,
                        '6' => 0x6,
                        '5' => 0x5,
                        '7' => 0x7,
                        '8' => 0x8,
                        '9' => 0x8,
                        'A' | 'a' => 0xA,
                        'B' | 'b' => 0xB,
                        'C' | 'c' => 0xC,
                        'D' | 'd' => 0xD,
                        'E' | 'e' => 0xE,
                        'F' | 'f' => 0xF,
                        _ => panic!("Invalid hex"),
                    })
                })
                .flatten()
                .collect::<Vec<u8>>();

            program.clone_into(&mut memory);

            let mut i = 0;
            loop {
                let line = &memory[i..i + 4];
                match *line {
                    [0x1, r, x, y] => {
                        registers[r as usize] = memory[((x as usize) << 4) + y as usize] as u32;
                    }
                    [0x2, r, x, y] => {
                        registers[r as usize] = ((x as u32) << 4) + y as u32;
                    }
                    [0x3, r, x, y] => {
                        memory[((x as usize) << 4) + y as usize] = registers[r as usize] as u8;
                    }
                    [0x4, 0, r, s] => {
                        registers[s as usize] = registers[r as usize];
                    }
                    [0x5, r, s, t] => unsafe {
                        registers[r as usize] = transmute(
                            transmute::<u32, i32>(registers[s as usize])
                                + transmute::<u32, i32>(registers[t as usize]),
                        );
                    },
                    [0x6, r, s, t] => unsafe {
                        registers[r as usize] = transmute(
                            transmute::<u32, f32>(registers[s as usize])
                                + transmute::<u32, f32>(registers[t as usize]),
                        );
                    },
                    [0x7, r, s, t] => {
                        registers[r as usize] = registers[s as usize] | registers[t as usize];
                    }
                    [0x8, r, s, t] => {
                        registers[r as usize] = registers[s as usize] & registers[t as usize];
                    }
                    [0x9, r, s, t] => {
                        registers[r as usize] = registers[s as usize] ^ registers[t as usize];
                    }
                    [0xA, r, 0, x] => {
                        registers[r as usize] = registers[r as usize].rotate_left(x as u32);
                    }
                    [0xB, r, x, y] => {
                        if registers[r as usize] == registers[0] {
                            i = (((x as usize) << 4) + y as usize) * 4;
                            continue;
                        }
                    }
                    [0xC, 0, 0, 0] => {
                        break;
                    }
                    // Extended ops for printing
                    [0xD, 0, 0, r] => {
                        println!("{}", registers[r as usize])
                    }

                    [0xE, 0, x, y] => {
                        println!("{}", memory[((x as usize) << 4) + y as usize])
                    }
                    _ => {
                        panic!("Invalid Operation At Line {}: {}", i, 0)
                    }
                };
                i += 4;
            }
        }
        Err(e) => panic!("{}", e),
    };
}
