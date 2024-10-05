use rand::Rng;
use std::thread;
use round::round_down;

mod tilemap;

const AIR_SYM: char = '.';

static mut GRAVITY_X: i16 = 1;
static mut GRAVITY_Y: i16 = 1;

//default 250
static DELTA: u32 = 50;
pub static mut objects: Vec<tilemap::Object> = vec![];
pub static mut polygon_objects: Vec<tilemap::PolygonObject> = vec![];

pub fn update(){
    let mut tmp_tile_map: tilemap::TileMap = tilemap::TileMap::new(AIR_SYM, 64, 64);
    unsafe{

        // POLYGONS
        for i in 0..polygon_objects.len() {
            for j in 0..polygon_objects[i].points.len() {
                tmp_tile_map.set_sym(polygon_objects[i].points[j], polygon_objects[i].sym);
            }

            let mut float_plus: f64 = 0.0;
            for j in 0..polygon_objects[i].points.len() {
                let mut tmp1: tilemap::Vec2 = tilemap::Vec2::def();
                let mut tmp2: tilemap::Vec2 = tilemap::Vec2::def();

                if j != polygon_objects[i].points.len() - 1 {
                    tmp1 = polygon_objects[i].points[j];
                    tmp2 = polygon_objects[i].points[j + 1];
                }
                else {
                    tmp1 = polygon_objects[i].points[polygon_objects[i].points.len() - 1];
                    tmp2 = polygon_objects[i].points[0];
                }


                let vector: tilemap::Vec2 = tilemap::Vec2::new(tmp1.x - tmp2.x, tmp1.y - tmp2.y);
                //vector.print();
                let mut oper_x: i16 = 0;
                let mut oper_y: i16 = 0;

                if vector.x >= 0 { oper_x = -1; }
                else { oper_x = 1 }

                if vector.y >= 0 { oper_y = -1; }
                else { oper_y = 1}

                let mut dir: f64 = vector.y as f64 / vector.x as f64;
                if dir < 0.0 { dir *= -1.0;  }

                //println!("{}", dir);
                
                let mut tmp: tilemap::Vec2 = tilemap::Vec2::def();
                for o in 0..64 {
                    float_plus += dir - round_down(dir, 0);
                    let mut clear_dir: i16 = round_down(dir, 0) as i16;
                    
                    let mut now_pos: tilemap::Vec2 = tilemap::Vec2::def();
                    if vector.x == 0 {
                        let mut opr: i16 = 1;
                        if vector.y < 0 { opr *= -1; }

                        for l in 0..vector.y * opr as i16 {
                            now_pos = tilemap::Vec2::new(polygon_objects[i].points[j].x + (tmp.x * oper_x), polygon_objects[i].points[j].y + (l * oper_y));
                            tmp_tile_map.set_sym(now_pos, polygon_objects[i].sym);
                        }
                        break;
                    }
                    for l in 0..clear_dir + round_down(float_plus, 0) as i16 {
                        now_pos = tilemap::Vec2::new(polygon_objects[i].points[j].x + (tmp.x * oper_x), polygon_objects[i].points[j].y + (tmp.y * oper_y));
                        tmp_tile_map.set_sym(now_pos, polygon_objects[i].sym);
                        tmp.y += 1;
                    }
                    tmp.x += 1;
                    //if (clear_dir + round_down(float_plus, 0) as i16) < 1 {
                        now_pos = tilemap::Vec2::new(polygon_objects[i].points[j].x + (tmp.x * oper_x), polygon_objects[i].points[j].y + (tmp.y * oper_y));
                        tmp_tile_map.set_sym(now_pos, polygon_objects[i].sym);
                    //}
                    if now_pos == tmp2 { break; }

                    if float_plus >= 1.0 {
                        float_plus -= 1.0;
                    }
                }
            }
        }
        
        //SINGLE PIXELS
        for i in 0..objects.len(){
            tmp_tile_map.set_sym(objects[i].position, objects[i].sym);
        }

        for i in 0..objects.len(){
            let tmp = &mut objects[i];

            if tmp.is_static == false {
                if tmp_tile_map.get_sym(tilemap::Vec2::new(tmp.position.x, tmp.position.y + GRAVITY_Y)) == AIR_SYM {
                    tmp_tile_map.set_sym(objects[i].position, AIR_SYM);
                    tmp.moveto(tilemap::Vec2::new(tmp.position.x, tmp.position.y + GRAVITY_Y));
                    tmp_tile_map.set_sym(objects[i].position, objects[i].sym);
                }
                if tmp_tile_map.get_sym(tilemap::Vec2::new(tmp.position.x + GRAVITY_X, tmp.position.y)) == AIR_SYM {
                    tmp_tile_map.set_sym(objects[i].position, AIR_SYM);
                    tmp.moveto(tilemap::Vec2::new(tmp.position.x + GRAVITY_X, tmp.position.y));
                    tmp_tile_map.set_sym(objects[i].position, objects[i].sym);
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

    /*unsafe {
        let mut trg = tilemap::PolygonObject::new('#', true);
        trg.add(tilemap::Vec2::new(7, 2));
        trg.add(tilemap::Vec2::new(2, 2));

        polygon_objects.push(trg);
    }*/

    /*unsafe {
        let mut added_count: i16 = 0;
        loop {
            let tmp = tilemap::Object::new(tilemap::Vec2::new(rng.gen_range(0..32), rng.gen_range(0..32)), '#', false);
            let mut count: i16 = 0;

            for j in 0..objects.len(){
                if tmp.position != objects[j].position{ count += 1; }
            }
            if count as usize == objects.len(){
                objects.push(tmp);
                added_count += 1;
                if added_count == 300 { break; }
            }
        }
        let mut added_count: i16 = 0;
        loop {
            let tmp = tilemap::Object::new(tilemap::Vec2::new(rng.gen_range(0..32), rng.gen_range(0..32)), 'K', true);
            let mut count: i16 = 0;

            for j in 0..objects.len(){
                if tmp.position != objects[j].position{ count += 1; }
            }
            if count as usize == objects.len(){
                objects.push(tmp);
                added_count += 1;
                if added_count == 32 { break; }
            }
        }
    }*/
    loop {
        frame_count += 1;
        //for i in 0..36{ println!("\n"); }
        
        //update();
        
        thread::sleep_ms(DELTA);
        unsafe {
            //if frame_count % 1 == 0 {
                let tmp1 = tilemap::Object::new(tilemap::Vec2::new(rng.gen_range(0..64), rng.gen_range(0..64)), '1', true);
                let tmp2 = tilemap::Object::new(tilemap::Vec2::new(rng.gen_range(0..64), rng.gen_range(0..64)), '2', true);
                let tmp3 = tilemap::Object::new(tilemap::Vec2::new(rng.gen_range(0..64), rng.gen_range(0..64)), '3', true);
                objects.push(tmp1);
                objects.push(tmp2);
                objects.push(tmp3);
                update();
                thread::sleep_ms(1500);

                let mut trg = tilemap::PolygonObject::new('#', true);
                trg.add(tilemap::Vec2::new(tmp1.position.x, tmp1.position.y));
                trg.add(tilemap::Vec2::new(tmp2.position.x, tmp2.position.y));
                trg.add(tilemap::Vec2::new(tmp3.position.x, tmp3.position.y));

                polygon_objects.push(trg);
                update();
                thread::sleep_ms(1500);
                polygon_objects.remove(0);
                objects.remove(objects.len() - 1);
                objects.remove(objects.len() - 1);
                objects.remove(objects.len() - 1);
                println!("now frame: {}", frame_count);
            //}
        }
    }
}
