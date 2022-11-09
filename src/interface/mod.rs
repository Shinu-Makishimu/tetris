//this piece of code from sdl example lib
use cgmath::Vector2;
use sdl2::{pixels::Color, event::Event};


use crate::engine::Engine;

const INIT_SIZE: Vector2<u32> = Vector2::new(1024,1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10,0x10,0x18);


pub fn run(_engine: Engine) {
    let sdl = sdl2::init().expect("Fail to init SDL2");

    let mut canvas = {
        let video = sdl.video().expect("Fail to acqure display");
        let window = video
            .window("Tetris", INIT_SIZE.x, INIT_SIZE.y)
            .position_centered()
            .resizable()
            .build()
            .expect("Fail to create window");

        window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("Fail to render canvas")
    };

    let mut events = sdl.event_pump().expect("Fail to get event loop");
    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } =>return,
                _ => {}
                }
            }
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();
        canvas.present();
    }

}
