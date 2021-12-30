extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::Duration;
mod vars;

fn main() {
    //initialize storage for strenght of pheromons
    let mut p_food = vec![vec![0; vars::SIZEY as usize]; vars::SIZEX as usize];
    let mut p_home = vec![vec![0; vars::SIZEY as usize]; vars::SIZEX as usize];

    //positions of stone
    let mut stones = vec![vec![0; vars::SIZEY as usize]; vars::SIZEX as usize];

    //Vector of all pixels that have changed during loop
    let mut changed_pixels :Vec<[i32; 2]> = Vec::new();

    //just for testing
    p_food[0][2] = 1;
    p_food[0][0] = 1;
    p_food[1][3] = 1;
    p_food[1][3] = 1;
    p_home[1][0] = 1;
    p_home[3][3] = 1;
    p_home[1][3] = 1;
    printarr(&p_food);

    //start visual
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", vars::WINDOWRES[0] as u32, vars::WINDOWRES[1] as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_draw_color(Color::RGB(27, 28, 30));
    canvas.clear();

    let mut first = true;
    let mut mouse_pressed = false;
    //the main loop
    'running: loop {
        //backgroung color

        if first {
            //looping through x, y
            for x in 0..p_food.len(){
                for y in 0..p_food[0].len(){
                    let food = p_food[x][y] != 0;
                    let home = p_home[x][y] != 0;
                    let tstone = stones[x][y] == 1;

                    if tstone {
                        canvas.set_draw_color(Color::RGB(60, 56, 54));
                        canvas.fill_rect(Rect::new(x as i32 * vars::BSX, y as i32 * vars::BSY, vars::BSX as u32, vars::BSY as u32));
                    } else {
                        if food && !home{
                            canvas.set_draw_color(Color::RGB(152, 151, 26));
                            canvas.fill_rect(Rect::new(x as i32 * vars::BSX, y as i32 * vars::BSY, vars::BSX as u32, vars::BSY as u32));
                        } else if !food && home {
                            canvas.set_draw_color(Color::RGB(204, 36, 29));
                            canvas.fill_rect(Rect::new(x as i32 * vars::BSX, y as i32 * vars::BSY, vars::BSX as u32, vars::BSY as u32));
                        } else if food && home {
                            canvas.set_draw_color(Color::RGB(250, 189, 47));
                            canvas.fill_rect(Rect::new(x as i32 * vars::BSX, y as i32 * vars::BSY, vars::BSX as u32, vars::BSY as u32));
                        }
                    }

                }
            }
            first = false;
        } else {
            for i in 0..changed_pixels.len() {
                let x :usize = changed_pixels[i][0] as usize;
                let y :usize = changed_pixels[i][1] as usize;

                let food = p_food[x][y] != 0;
                let home = p_home[x][y] != 0;
                let tstone = stones[x][y] == 1;

                if tstone {
                    canvas.set_draw_color(Color::RGB(60, 56, 54));
                    canvas.fill_rect(Rect::new(x as i32 * vars::BSX, y as i32 * vars::BSY, vars::BSX as u32, vars::BSY as u32));
                } else {
                    if food && !home{
                        canvas.set_draw_color(Color::RGB(152, 151, 26));
                        canvas.fill_rect(Rect::new(x as i32 * vars::BSX, y as i32 * vars::BSY, vars::BSX as u32, vars::BSY as u32));
                    } else if !food && home {
                        canvas.set_draw_color(Color::RGB(204, 36, 29));
                        canvas.fill_rect(Rect::new(x as i32 * vars::BSX, y as i32 * vars::BSY, vars::BSX as u32, vars::BSY as u32));
                    } else if food && home {
                        canvas.set_draw_color(Color::RGB(250, 189, 47));
                        canvas.fill_rect(Rect::new(x as i32 * vars::BSX, y as i32 * vars::BSY, vars::BSX as u32, vars::BSY as u32));
                    }
                }
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}  => {
                    break 'running
                },
                Event::MouseButtonDown{x, y, ..} => {
                    mouse_pressed = true;
                    let stone_new = make_stones((x / vars::BSX) as i32, (y / vars::BSY) as i32, stones);
                    stones = stone_new.0;
                    changed_pixels = stone_new.1;
                },
                Event::MouseMotion{x, y, ..} => {
                    println!("{} {}", x, y);
                    if mouse_pressed {
                        let stone_new = make_stones((x / vars::BSX) as i32, (y / vars::BSY) as i32, stones);
                        stones = stone_new.0;
                        changed_pixels = stone_new.1;
                    }
                },
                Event::MouseButtonUp{..} => {
                    mouse_pressed = false;
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn printarr(arr: &Vec<Vec<i32>>) {
    for x in 0..arr.len() {
        let mut strx = String::from("");
        for y in 0..arr[0].len() {
            strx.push_str(&arr[x][y].to_string());
        }
        println!("{}", strx);
    }
}

fn make_stones(x :i32, y :i32, mut arr: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<[i32; 2]>){
    let mut changed_new :Vec<[i32; 2]> = Vec::new();
    for xc in x - vars::RADIUS..x + vars::RADIUS {
        if 0 <= xc && xc < vars::SIZEX{
            for yc in y - vars::RADIUS..y + vars::RADIUS {
                if 0 <= yc && yc < vars::SIZEY{
                    if (xc - x).pow(2) + (yc - y).pow(2) <= vars::RADIUS.pow(2) {
                        arr[xc as usize][yc as usize] = 1;
                        changed_new.push([xc, yc])
                    }
                }
            }
        }
    }
    return (arr, changed_new);

}
