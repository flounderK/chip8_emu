
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::convert::TryInto;
mod display;
use display::Display;

#[derive(Default)]
struct Registers {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
    // address register
    i:  u16,
    pc: usize,
}

impl Registers {
    pub fn reg_for(&mut self, i:usize) -> &mut u8{
        match i & 0xf {
            0x0 => { &mut self.v0 }
            0x1 => { &mut self.v1 }
            0x2 => { &mut self.v2 }
            0x3 => { &mut self.v3 }
            0x4 => { &mut self.v4 }
            0x5 => { &mut self.v5 }
            0x6 => { &mut self.v6 }
            0x7 => { &mut self.v7 }
            0x8 => { &mut self.v8 }
            0x9 => { &mut self.v9 }
            0xa => { &mut self.va }
            0xb => { &mut self.vb }
            0xc => { &mut self.vc }
            0xd => { &mut self.vd }
            0xe => { &mut self.ve }
            0xf => { &mut self.vf }
            _ => return &mut self.vf
        }
    }
}

struct Emu {
    regs: Registers,
    mem: Vec<u8>,
    stack: Vec<u16>,
    pub display: Display,
}

impl Emu {
    pub fn new() -> Emu {
        let mut regs = Registers {..Default::default()};
        let mut mem = vec![0u8;0x1000];
        let mut stack = vec![0u16];
        regs.pc = 0x200;
        Emu {
              regs: regs,
              mem: mem,
              stack: stack,
              display: Display::new().unwrap(),
        }
    }
    pub fn tick(&mut self){
        //let length  = self.mem.len();
        let mut input = &mut self.mem[..];
        //while self.regs.pc < length {
            input = &mut self.mem[self.regs.pc..];
            let (op_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
            let op = u16::from_be_bytes(op_bytes.try_into().unwrap());

            match op {
                // display clear
                0x00E0 => {
                    println!("I'd clear the display here if I could");
                }
                // return
                0x00EE => {
                    //println!("return ");
                    let ret_addr = self.stack.pop();
                    if ret_addr.is_none() {
                        println!("ERR Out of return addresses");
                        return;
                    }
                    self.regs.pc = ret_addr.unwrap() as usize;
                    return;
                }
                // 0x0NNN call NNN
                0x0000..=0x0fff | 0x2000..=0x2fff => {
                    self.stack.push(self.regs.pc as u16 + std::mem::size_of::<u16>() as u16);
                    self.regs.pc = (op & 0xfff) as usize;
                    return;
                }
                // 0x1NNN goto NNN
                0x1000..=0x1fff => {
                    self.regs.pc = (op & 0xfff) as usize;
                    //println!("goto {:x}", i);
                    return;
                }

                //3XNN if (Vx == NN)
                0x3000..=0x3fff => {
                    let vx = self.regs.reg_for(((op & 0x0f00) >> 8) as usize);
                    let nn = (op & 0xff) as u8;
                    if *vx == nn {
                        self.regs.pc += std::mem::size_of::<u16>();
                    }
                }

                //4XNN if (Vx != NN)
                0x4000..=0x4fff => {
                    let vx = self.regs.reg_for(((op & 0x0f00) >> 8) as usize);
                    let nn = (op & 0xff) as u8;
                    if *vx != nn {
                        self.regs.pc += std::mem::size_of::<u16>();
                    }

                }
                //5XY0 if (Vx == Vy)
                0x5000..=0x5fff => {
                    let vx = *self.regs.reg_for(((op & 0x0f00) >> 8) as usize);
                    let vy = *self.regs.reg_for(((op & 0x00f0) >> 4) as usize);
                    if vx == vy {
                        self.regs.pc += std::mem::size_of::<u16>();
                    }

                }
                //6XNN 	Vx = N
                0x6000..=0x6fff => {
                    let vx = self.regs.reg_for(((op & 0x0f00) >> 8) as usize);
                    let nn = (op & 0xff) as u8;
                    *vx = nn;

                }
                //7XNN  Vx += N
                // TODO: check this
                0x7000..=0x7fff => {
                    let vx = self.regs.reg_for(((op & 0x0f00) >> 8) as usize);
                    let nn = (op & 0xff) as u8;
                    let a = *vx as u8;
                    *vx =  (a.wrapping_add(nn) % 0xff) as u8;

                }
                // 8xxx series
                0x8000..=0x8fff => {
                    let vy = *self.regs.reg_for(((op & 0x00f0) >> 4) as usize);
                    let p_vx = self.regs.reg_for(((op & 0x0f00) >> 8) as usize);
                    let mut carry_or_borrow = false;
                    //let nn = (op & 0xff) as u8;
                    match op & 0xf {
                        //8XY0 vx = Vy
                        0 => {
                            *p_vx = vy;
                        }
                        // 8XY1 vx |= Vy
                        1 => {
                            *p_vx |= vy;
                        }
                        // 8XY2 vx &= Vy
                        2 => {
                            *p_vx &= vy;
                        }
                        // 8XY3 vx ^= Vy
                        3 => {
                            *p_vx ^= vy;
                        }
                        // 8XY4 vx += Vy
                        // TODO: set carry if there is one
                        4 => {
                            let a = *p_vx;
                            let b = vy;
                            if (*p_vx & vy) > 0 {
                                carry_or_borrow = true;
                            }
                            *p_vx = (a.wrapping_add(b)) as u8;
                            if a > *p_vx {
                                self.regs.vf = 1;
                            } else {
                                self.regs.vf = 0;
                            }
                        }
                        // 8XY5 vx -= Vy
                        // TODO: unset borrow if there is one
                        5 => {
                            let a = *p_vx as usize;
                            let b = vy as usize;
                            let nvx: u8 = !*p_vx;
                            let nvy: u8 = !vy;
                            if ((nvx & nvy) > 0) {
                                carry_or_borrow = true;
                            }
                            //*p_vx = ((a - b) % 0xff) as u8;
                            *p_vx = (a.wrapping_sub(b) % 0xff) as u8;
                            if carry_or_borrow == true {
                                self.regs.vf = 0;
                            } else {
                                self.regs.vf = 1;
                            }
                        }
                        // 8XY6 vx >>= Vy
                        6 => {
                            let least_sig = *p_vx & 0x1;
                            *p_vx >>= 1;
                            self.regs.vf = least_sig;
                        }

                        // 8XY7 vx = Vy - vx
                        // TODO: unset borrow if there is one
                        7 => {
                           *p_vx = vy - *p_vx;
                        }

                        // 8XYE vx <<= Vy
                        0xe => {
                            *p_vx <<= 1;
                        }

                        _ => {
                            println!("unhandled opcode {:x}", op);
                        }

                    }
                }
                //9XY0
                0x9000..=0x9fff => {
                    let vx = *self.regs.reg_for(((op & 0x0f00) >> 8) as usize);
                    let vy = *self.regs.reg_for(((op & 0x00f0) >> 4) as usize);
                    if vx != vy {
                        self.regs.pc += std::mem::size_of::<u16>();
                    }
                }
                // ANNN
                0xa000..=0xafff => {
                    self.regs.i = (op & 0xfff) as u16;
                }

                //BNNN
                0xb000..=0xbfff => {
                    self.regs.pc = self.regs.v0 as usize + ((op & 0xfff)as usize);
                    return;
                }
                //CXNN Vx = rand() & NN
                0xc000..=0xcfff => {
                    let p_vx = self.regs.reg_for(((op & 0x0f00) >> 8) as usize);
                    *p_vx = 44;
                }

                //DXYN // draw(Vx, Vy, N)
                //Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels
                //and a height of N pixels. Each row of 8 pixels is read as bit-coded
                //starting from memory location I; I value does not change after the
                //execution of this instruction. As described above, VF is set to 1
                //if any screen pixels are flipped from set to unset when the sprite is
                //drawn, and to 0 if that does not happen
                0xd000..=0xdfff => {
                    let vx = *self.regs.reg_for(((op & 0x0f00) >> 8) as usize);
                    let vy = *self.regs.reg_for(((op & 0x00f0) >> 4) as usize);
                    let n = (op & 0xf) as usize;
                    let ind = self.regs.i as usize;
                    let sprite = &self.mem[ind..ind+n];
                    let set_vf = self.display.draw(vx, vy, sprite);
                    self.display.present();
                    if set_vf == true {
                        self.regs.vf = 1;
                    }
                }

                0xe000..=0xefff => {
                    match op & 0xff {
                        // EX9E if (key() == Vx)
                        0x9e => {

                        }

                        // EXA1 if (key() != Vx)
                        0xa1 => {

                        }
                        _ => {
                            println!("unhandled opcode {:x}", op);
                        }
                    }

                }

                0xf000..=0xffff => {
                    let p_vx = self.regs.reg_for(((op & 0x0f00) >> 8) as usize);
                    match op & 0xff {
                        // Sets VX to the value of the delay timer.
                        0x07 => {

                        }
                        // get key
                        0x0a => {
                            println!("get key");

                        }
                        // Sets the delay timer to VX.
                        0x15 => {

                        }
                        // set sound timer to vx
                        0x18 => {

                        }
                        0x1e => {
                            let a = *p_vx as u16;
                            self.regs.i = self.regs.i.wrapping_add(a);
                        }

                        0x29 => {
                            println!("set i to sprite addr {:x}", *p_vx);
                        }

                        0x33 => {
                            println!("set binary coded decimal");
                        }

                        0x55 => {
                            println!("reg dump to {:x}", self.regs.i);
                        }

                        0x65 => {
                            println!("reg load from {:x}", self.regs.i);
                        }

                        _ => {
                            println!("unhandled opcode {:x}", op);
                        }
                    }
                }

                _ => {
                    println!("unhandled opcode {:x}", op);
                }

            }
            self.regs.pc += std::mem::size_of::<u16>();
        //}
    }
}

pub fn draw(){

}

fn main() -> std::io::Result<()> {
    let mut emu = Emu::new();
    let arg = env::args_os().nth(1);  //.map(|n| (*n, false));b
    if arg == None {
        println!("Usage: $0 <chip-8-binary>");
        return Ok(())
    }


    let file = File::open(arg.unwrap())?;
    //let file = File::open("SCTEST.CH8")?;
    let mut buf_reader = BufReader::new(file);
    // chip 8 assumes programs are loaded at 0x200
    // and apparently so do the programs
    buf_reader.read(&mut emu.mem[0x200..]);
    let length = emu.mem.len();
    while emu.regs.pc < length {
        emu.tick();
    }

    Ok(())
}
