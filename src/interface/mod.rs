use std::time::Duration;

//this piece of code from sdl example lib
use cgmath::{Vector2, EuclideanSpace, ElementWise, Point2};
use sdl2::{pixels::Color, event::Event, rect::{Rect, Point}, render::Canvas, video::Window, sys::KeyCode, keyboard::Keycode};
mod render_trait;
use self::render_trait::ScreenColor;
use crate::engine::{Engine, Matrix, Color as SemanticColor, MoveKind};

const INIT_SIZE: Vector2<u32> = Vector2::new(1024,1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10,0x10,0x18);
const PLACEHOLDER_1: Color = Color::RGB(0x66, 0x77, 0x77);
const PLACEHOLDER_2: Color = Color::RGB(0x77, 0x88, 0x88);

struct Tick;
struct LockTick;
struct SoftDropTick;
struct Sleep(Duration);


pub fn run(mut engine: Engine) {
    let sdl = sdl2::init().expect("Fail to init SDL2");

    let event_subsys = sdl.event().expect("faled to activate event subsystem");
    event_subsys.register_custom_event::<Tick>().unwrap();
    event_subsys.register_custom_event::<LockTick>().unwrap();

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
    
    event_subsys.push_custom_event(Tick).unwrap();
    event_subsys.push_custom_event(LockTick).unwrap();
    let mut dirty: bool = true;
    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } =>return,
                Event::User { .. } if event.as_user_event_type::<Tick>().is_some() => {
                        println!("tick ev");
                        dirty = true;

                    },
                Event::User { .. } if event.as_user_event_type::<LockTick>().is_some() => {
                        println!("lock tick  ev");
                        dirty = true;
                    },
                Event::KeyDown { keycode: Some(key) , ..} => match key {
                        Keycode::Right => drop(engine.move_cursor(MoveKind::Right)),
                        Keycode::Left => drop(engine.move_cursor(MoveKind::Left)),
                        Keycode::Up => engine.hard_drop(),
                        Keycode::Down => todo!("soft drop"),
                        _ => {}
                    },
                _ => {}
                }
            }
        draw(&mut canvas, &engine);
        dirty = false;
    }

}

enum Input {
    Move(MoveKind),
    SoftDrop,
    HardDrop,
}

impl TryFrom<KeyCode> for Input {
    type Error = ();
    
    fn try_from(key: KeyCode) -> Result<Self, Self::Error> {
        Ok(match key {
            Keycode::Right => Self::Move(MoveKind::Right), 
            Keycode::Left  => Self::Move(MoveKind::Left),
            Keycode::Up    => Self::HardDrop,
            Keycode::Down  => Self::SoftDrop,
            _ => return Err(())
        })
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

    for sub_rect in [&matrix, &up_next, &hold, &queue, &score_area] {
        canvas.fill_rect(*sub_rect).unwrap();
    }

    let mut cell_draw_ctx = CellDrawContext {
        origin: matrix.bottom_left(),
        dims: Vector2::from(matrix.size()),
        canvas
    };
    
    
    for (coord, cell) in engine.cells() {
        cell_draw_ctx.try_draw_cell(coord, cell);
    }

    if let Some ((cursor_cells , cursor_color)) = engine.cursor_info() {
        for coord in cursor_cells  {
            cell_draw_ctx.draw_cell(coord, cursor_color);  
        }

    }
    



    canvas.present();
}


struct CellDrawContext<'canvas> {
    origin: Point,
    dims: Vector2<u32>,
    canvas: &'canvas mut Canvas<Window>,
}

impl CellDrawContext<'_> {
    const CELL_COUNT: Vector2<u32> = Vector2::new(Matrix::WIDTH as u32, Matrix::HEIGHT as u32);
    
    fn try_draw_cell(
        &mut self, 
        coord: Point2<usize>, 
        cell: Option<SemanticColor>, 
    ) {
        let Some(color) = cell else {
            return;
        };
        self.draw_cell(coord, color);
    }



    fn draw_cell( 
            &mut self, 
            coord: Point2<usize>, 
            color: SemanticColor, 
        ) {

        let coord = coord.to_vec().cast::<u32>().unwrap();
        let this = (coord + Vector2::new(0,1)).mul_element_wise(self.dims).div_element_wise(Self::CELL_COUNT);
        let next = (coord + Vector2::new(1,0)).mul_element_wise(self.dims).div_element_wise(Self::CELL_COUNT);
        let cell_rect = Rect::new(
            self.origin.x + this.x as i32,
            self.origin.y - this.y as i32,
            next.x - this.x,
            this.y - next.y,
        );
        self.canvas.set_draw_color(color.screen_color());
        self.canvas.fill_rect(cell_rect).unwrap();
    }
    
}