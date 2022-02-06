use sdl2::video;
use sdl2::render;
use sdl2::Sdl;
use sdl2::pixels::{PixelFormatEnum,Color};
use sdl2::rect::{Rect,Point};
use std::io;
use std::io::Write;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

// 64 x 32
pub struct Display {
    gfx: [[u8;64]; 32],
    draw_flag: bool,
    //pub window: video::Window,
    sdl_context: Sdl,
    pub screen: render::WindowCanvas,
}

impl Display {
    pub fn new() -> Result<Display, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.set_scale(4.5, 4.5);
        canvas.clear();
        //canvas.present();
        Ok(Display {
            gfx: [[0; 64]; 32],
            draw_flag: true,
            //window: window,
            sdl_context: sdl_context,
            screen: canvas,
        })
    }
    pub fn present(&mut self){
        for y in 0..32 {
            for x in 0..64 {

                let p = Point::new(x as i32, y as i32);
                if self.gfx[y][x] != 0 {
                    self.screen.set_draw_color(Color::RGB(0, 0, 0));
                } else{
                    self.screen.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
                }

                self.screen.draw_point(p).unwrap();
            }
        }
        self.screen.present();
    }

    pub fn draw(&mut self, x: u8, y:u8, sprite:&[u8]) -> bool{
        //println!("draw {:?} {:?} ", x, y);
        let mut flip = false;
        /*
        // print out sprite
        for h in 0..sprite.len() {
            for w in 0..8 {
                if sprite[h] & (0x80 >> w) == 0 {
                    print!(" ");
                } else {
                    print!("*");
                }
            }
            println!("");
        }
        println!("");
        io::stdout().flush().unwrap();
        */
        // height of sprite
        for h in 0..sprite.len() {

            // width of sprite
            for w in 0..8 {
                // just check each individual bit
                // of the sprite
                // to see if the value needs to be added to
                // the graphics buffer
                if (sprite[h] & (0x80 >> w)) != 0 {
                    if self.gfx[(y as usize+h) as usize][(x+w) as usize] == 1 {
                        flip = true;
                    }
                    self.gfx[(y as usize +h) as usize][(x+w) as usize] ^= 1;

                }
            }
        }
        /*
        // print out pixels
        for c in 0..32 {
            for r in 0..64 {
                if self.gfx[c][r] == 0 {
                    print!(" ");
                } else {
                    print!("*");
                }
            }
            println!("");
        }
        println!("");
        io::stdout().flush().unwrap();
        */


        flip
    }

    pub fn poll_event(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump()?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::Window { win_event: WindowEvent::Close, .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Ok(());
                },
                _ => {}
            }
        }

        Ok(())
    }

    /*
    pub fn display_pixels(&mut self){

        let old_draw_color = self.screen.draw_color();
        //self.screen.clear();
        //let any_unflipped
        for c in 0..32{
            for r in 0..64{

            let new_rect = Rect::new((r as i32), (c as i32), 8, self.gfx[c][r] as u32);
            //self.screen.draw_point(p);
            if self.gfx[c][r] > 0 {
                self.screen.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
            } else {
                self.screen.set_draw_color(Color::RGB(0, 0, 0));
            }
            self.screen.fill_rect(new_rect);
            }
        }
        //self.screen.set_draw_color(old_draw_color);
        self.screen.present();

        self.screen.set_draw_color(old_draw_color);
    }
    */

}
