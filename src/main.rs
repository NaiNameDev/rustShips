use rand::Rng;
use std::thread;

mod tilemap;

static mut gravity_x: i16 = 1;
static mut gravity_y: i16 = 1;

//default 250
static delta: u32 = 25;
pub static mut phys: Vec<tilemap::Object> = vec![];

pub fn update(){
    let mut tmp_tile_map: tilemap::TileMap = tilemap::TileMap::new('~');
    unsafe{
        for i in 0..phys.len(){
            tmp_tile_map.set_sym(phys[i].position, phys[i].sym);
        }

        for i in 0..phys.len(){
            let mut tmp = &mut phys[i];

            if tmp.is_static == false {
                if tmp_tile_map.get_sym(tilemap::Vec2::new(tmp.position.x, tmp.position.y + gravity_y)) == '~'{
                    tmp_tile_map.set_sym(phys[i].position, '~');
                    tmp.moveto(tilemap::Vec2::new(tmp.position.x, tmp.position.y + gravity_y));
                    tmp_tile_map.set_sym(phys[i].position, phys[i].sym);
                }
                if tmp_tile_map.get_sym(tilemap::Vec2::new(tmp.position.x + gravity_x, tmp.position.y)) == '~'{
                    tmp_tile_map.set_sym(phys[i].position, '~');
                    tmp.moveto(tilemap::Vec2::new(tmp.position.x + gravity_x, tmp.position.y));
                    tmp_tile_map.set_sym(phys[i].position, phys[i].sym);
                }
            }
        }
    }
    tmp_tile_map.read();
    //println!("{}", tmp_tile_map.count('#'));
}

fn main(){
    let mut frame_count: i32 = 0;

    let mut rng = rand::thread_rng();
    unsafe {
        let mut added_count: i16 = 0;
        loop {
            let tmp = tilemap::Object::new(tilemap::Vec2::new(rng.gen_range(0..32), rng.gen_range(0..32)), '#', false);
            let mut count: i16 = 0;

            for j in 0..phys.len(){
                if tmp.position != phys[j].position{ count += 1; }
            }
            if count as usize == phys.len(){
                phys.push(tmp);
                added_count += 1;
                if added_count == 300 { break; }
            }
        }
    }
    loop {
        frame_count += 1;
        //for i in 0..36{ println!("\n"); }
        
        update();
        
        println!("now frame: {}", frame_count);
        thread::sleep_ms(delta);
        
        unsafe{
            if frame_count % 41 == 0 { gravity_x *= -1; gravity_y *= -1}
        }
    }
}
