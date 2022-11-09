//this piece of code from sdl example lib
use cgmath::Vector2;
use sdl2::{
    pixels::Color, 
    event::Event, 
    rect::Rect, 
    render::Canvas, 
    video::Window
};
mod render_trait;
use self::render_trait::ScreenColor;
use crate::engine::{Engine, Matrix};

const INIT_SIZE: Vector2<u32> = Vector2::new(1024,1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10,0x10,0x18);
const PLACEHOLDER_1: Color = Color::RGB(0x66, 0x77, 0x77);
const PLACEHOLDER_2: Color = Color::RGB(0x77, 0x88, 0x88);



pub fn run(engine: Engine) {
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
        draw(&mut canvas, &engine);
    }

}

fn draw(canvas: &mut Canvas<Window>, engine: &Engine) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    let ui_sqare = {
            let Vector2{x,y} = Vector2::from(canvas.viewport().size())
                .cast::<i32>()
                .unwrap();
        
            if x > y {
        
                let mid_point = x / 2;
                let left_edge = mid_point - (y / 2);
                Rect::new(left_edge,0,y as u32,y as u32)

            } else {
                let mid_point = y / 2;
                let top_edge =  mid_point - (x / 2);
                Rect::new(0,top_edge,x as u32,x as u32)
            }


    };

    let matrix = {

        let mut middle_section = ui_sqare;  // section with game field
        middle_section.set_width(middle_section.width() / 2);
        middle_section.center_on(ui_sqare.center());
        canvas.set_draw_color(Color::WHITE);

        let mut matrix = middle_section;   //game field.
        matrix.set_width((matrix.width() as f32 * (7.0 / 8.0)) as _); // as _ - convert in required format (i32 in this)
        matrix.set_height((matrix.height() as f32 * (7.0 / 8.0)) as _);

        matrix.center_on(middle_section.center());
        matrix
    };

    let up_next = {
        let mut bounding_rect = ui_sqare;
        let quarter = ui_sqare.width() / 4;
        bounding_rect.resize(quarter, quarter);
        bounding_rect.offset((quarter * 3) as _, 0);
        
        let mut rect = bounding_rect;
        let inner_dim = bounding_rect.width() * 3 / 4;
        rect.resize(inner_dim, inner_dim);

        rect.center_on(bounding_rect.center());
        rect
    };

    let hold = {
        let mut bounding_rect = ui_sqare;
        let quarter = ui_sqare.width() / 4;
        bounding_rect.resize(quarter, quarter);
        
        let mut rect = bounding_rect;
        let inner_dim = bounding_rect.width() * 3 / 4;
        rect.resize(inner_dim, inner_dim);

        rect.center_on(bounding_rect.center());
        rect
    };

    let queue = {
        let mut bounding_rect = ui_sqare;
        let quarter = ui_sqare.width() / 4;
        bounding_rect.resize(quarter, quarter * 3);
        bounding_rect.offset((quarter * 3) as i32, quarter as i32 ); 
        
        let mut rect = bounding_rect;
        let inner_width = bounding_rect.width() * 5 / 8;
        let inner_height = bounding_rect.height() * 23 / 24;
        rect.resize(inner_width, inner_height);
        rect.center_on(bounding_rect.center());
        rect.set_y( bounding_rect.top());

        rect.center_on(bounding_rect.center());
        rect
    };

    let score_area = {
        let mut bounding_rect = ui_sqare;
        let quarter = ui_sqare.width() / 4;
        let sixteenth = quarter / 4;

        bounding_rect.resize(quarter, quarter * 2);
        bounding_rect.offset(0, (sixteenth * 5 )as i32 ); 
        
        let mut rect = bounding_rect;
        let inner_width = bounding_rect.width() * 7 / 8;
        rect.set_width(inner_width);
        rect.center_on(bounding_rect.center());
        rect.set_y( bounding_rect.top());

        rect.center_on(bounding_rect.center());
        rect
    };

    //this block is draw blocks for background

    canvas.set_draw_color(PLACEHOLDER_1);
    canvas.fill_rect(matrix).unwrap();
    canvas.set_draw_color(PLACEHOLDER_2);
    canvas.fill_rect(up_next).unwrap();
    canvas.fill_rect(hold).unwrap();
    canvas.fill_rect(queue).unwrap();
    canvas.fill_rect(score_area).unwrap();

    /*for cell_x in 0 .. Matrix::WIDTH {
        for cell_y in 0 .. Matrix::HEIGHT {
            todo!()
        }
    } */ //insted using two cycles, use iterator

    let matrix_origin = matrix.bottom_left();
    let (matrix_width, matrix_height) = matrix.size();

    for (coord, cell) in engine.cells() {
        let Some(cell_color) = cell else {
            continue;
        };

        let coord = coord.cast::<i32>().unwrap();
        
        let this_x = (coord.x as u32 + 0) * matrix_width / Matrix::WIDTH as u32;
        let this_y = (coord.y as u32 + 1) * matrix_height / Matrix::HEIGHT as u32;
        let next_x = (coord.x as u32 + 1) * matrix_width / Matrix::WIDTH as u32;
        let next_y = (coord.y as u32 + 2) * matrix_height / Matrix::HEIGHT as u32;
        let cell_rect = Rect::new(
            matrix_origin.x + this_x as i32,
            matrix_origin.y - this_y as i32,
            next_x - this_x,
            next_y - this_y,
        );
        
        canvas.set_draw_color(cell_color.screen_color());
        canvas.fill_rect(cell_rect).unwrap();
    }
    canvas.present();
}

