use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use cairo::Context;
use gtk4::gdk::RGBA;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::DrawingArea;
use gtk4::Gesture;
use gtk4::GestureClick;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::glib;
use maze_game::Maze;

const HEADER_HEIGHT: i32 = 37;
const NODE_SIZE: i32 = 50;
const MARGIN: i32 = 2;

fn main() -> glib::ExitCode {
    let maze = Rc::new(RefCell::new(Maze::new()));
    let app = Application::builder()
            .application_id("com.senengoding.MazeGame")
            .build();

    app.connect_activate(build_maze(Rc::clone(&maze)));

    app.run()
}


fn build_maze(maze: Rc<RefCell<Maze>>) -> impl Fn(&Application)
{
    move |app: &Application| {
        let inner_maze = maze.borrow();
        let maze_draw = Rc::new(DrawingArea::new());
    
        
            
        maze_draw.set_draw_func(draw_maze(Rc::clone(&maze)));

        maze_draw.queue_draw();

        let left_click = GestureClick::new();
        left_click.set_button(1);
        left_click.connect_pressed(left_click_controller(maze_draw.clone(), maze.clone()));

        let right_click = GestureClick::new();
        right_click.set_button(3);
        right_click.connect_pressed(right_click_controller(Rc::clone(&maze_draw), Rc::clone(&maze)));
        
        maze_draw.add_controller(left_click);
        maze_draw.add_controller(right_click);

        let height_size = inner_maze.height() as i32 * (NODE_SIZE + MARGIN) + MARGIN + HEADER_HEIGHT;
        let width_size = inner_maze.width() as i32 * (NODE_SIZE + MARGIN) + MARGIN;

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(width_size)
            .default_height(height_size)
            .title("Hello World")
            .child(&*maze_draw)
            .build();
        
        window.set_resizable(false);
        window.present();

    }
}

fn draw_maze(maze: Rc<RefCell<Maze>>) -> impl Fn(&DrawingArea, &Context, i32, i32) {

    move |area: &DrawingArea, cr: &Context, width: i32, height: i32| {
        let gray = (0.0, 0.7, 0.7, 1.);
        let red = (1., 0., 0., 1.);
        let green = (0., 1., 0., 1.);
        let black = (1., 1., 1., 1.);

        let maze = maze.borrow();

        for y in 1..(maze.height() + 1) {
            for x in 1..(maze.width() + 1) {
                let idx = maze.get_index(x, y);


                let color;

                if let Ok(neigh) = maze.get_node_relation(idx) {
                    if idx == maze.start() {
                        color = green;
                    } else if idx == maze.end() {
                        color = red;
                    } else if neigh.len() == 0 {
                        color = black;
                    } else {
                        color = gray;
                    }


                    let (r, g, b, a) = color;
                    cr.set_source_color(&RGBA::new(r, g, b, a));
                    cr.rectangle(((MARGIN + NODE_SIZE) * (x-1) as i32) as f64, ((MARGIN + NODE_SIZE) * (y-1) as i32) as f64, NODE_SIZE as f64, NODE_SIZE as f64);
                    cr.fill().unwrap();
                }
            }
        }
        
    
        println!("h: {}\nw: {}", height, width);
    }
}


fn left_click_controller(maze_draw: Rc<DrawingArea>, maze: Rc<RefCell<Maze>>) -> impl Fn(&GestureClick, i32, f64, f64) {
    move |_gesture, _num, x, y| {
        let mut inner_maze = maze.borrow_mut();

        let x_coord = x / maze_draw.width() as f64 * inner_maze.width() as f64;
        let y_coord = y / maze_draw.height() as f64 * inner_maze.height() as f64;

        let x_coord = x_coord.ceil() as usize;
        let y_coord = y_coord.ceil() as usize;
        println!("x: {}, y: {}", x, y);
        println!("x_coord: {}, y_coord: {}", x_coord, y_coord);

        let end_idx = inner_maze.get_index(x_coord, y_coord);
        inner_maze.set_end(end_idx);
        maze_draw.queue_draw();

        let bfs_result = inner_maze.get_bfs();

        println!("{:?}", bfs_result);

        for idx in bfs_result {
            inner_maze.set_start(idx);
        }
        
        maze_draw.queue_draw();
    }
}

fn right_click_controller(maze_draw: Rc<DrawingArea>, maze: Rc<RefCell<Maze>>) -> impl Fn(&GestureClick, i32, f64, f64) {
    move |_gesture, _num, x, y| {
        let mut inner_maze = maze.borrow_mut();
        let x_coord = x / maze_draw.width() as f64 * inner_maze.width() as f64;
        let y_coord = y / maze_draw.height() as f64 * inner_maze.height() as f64;

        let x_coord = x_coord.ceil() as usize;
        let y_coord = y_coord.ceil() as usize;

        let idx = inner_maze.get_index(x_coord, y_coord);
        inner_maze.toggle(idx);
        
        maze_draw.queue_draw();
    }
}