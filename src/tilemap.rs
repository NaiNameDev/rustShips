#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[derive(Copy, Clone, PartialEq)]
pub struct Vec2 {pub x: i16, pub y: i16}
impl Vec2 {
    pub fn new(x: i16, y: i16) -> Vec2{
        return Vec2{x: x, y: y};
    }
    pub fn def() -> Vec2{ return Vec2{x: 0, y: 0}; }
    pub fn print(&self){
        println!("{},{}",self.x, self.y);
    }
}

#[derive(Copy,Clone)]
pub struct Object {
    pub position: Vec2,
    pub sym: char,
    pub is_static: bool,
    pub is_on_floor: bool,
}

impl Object {
    pub fn new(pos: Vec2, sym: char, stat: bool/*, toadd: &mut Vec<Object>*/) -> Object {
        let mut newObj = Object{position: pos, sym: sym, is_static: stat, is_on_floor: false};
        //toadd.push(self);
        return newObj;
    }
    pub fn moveto(&mut self, new_pos: Vec2) {
        self.position = new_pos;
    }
    pub fn rename(&mut self, new_sym: char) {
        self.sym = new_sym;
    }
}

const GLOBAL_X: usize = 32;
const GLOBAL_Y: usize = 32;

pub struct TileMap {
    arr: [(char, Vec2); GLOBAL_X * GLOBAL_Y],
}
impl TileMap{
    pub fn new(sym: char) -> TileMap {
        let mut tmp_world: TileMap = TileMap{ arr: [(sym,Vec2::def()); (GLOBAL_X * GLOBAL_Y) as usize]};
        let mut tmp_pos = Vec2::new(0,0);

        for i in 0..GLOBAL_X * GLOBAL_Y{
            if tmp_pos.x < (GLOBAL_X - 1) as i16{
                tmp_world.arr[i].1 = Vec2::new(tmp_pos.x, tmp_pos.y);
                tmp_pos.x += 1;
            }
            else{
                tmp_world.arr[i].1 = Vec2::new(tmp_pos.x, tmp_pos.y);

                tmp_pos.y += 1;
                tmp_pos.x = 0;
            }
        }
        return tmp_world;
    }

    pub fn read(&self){
        let mut tmp_pos: Vec2 = Vec2::new(0,0);
        let mut tmp_string: String = "".to_string();
        
        for i in 0..GLOBAL_X * GLOBAL_Y{
            if tmp_pos.x < (GLOBAL_X - 1) as i16{
                tmp_pos.x += 1;
                
                tmp_string += " ";
                tmp_string += &(self.arr[i].0).to_string();
            }
            else{
                tmp_pos.y += 1;
                tmp_pos.x = 0;
                
                tmp_string += " ";
                tmp_string += &(self.arr[i].0).to_string();
                tmp_string += "\n";
            }
        }
    println!("{}",tmp_string); 
    }
    
    pub fn count(&self, sym: char) -> i16{
        let mut counter: i16 = 0;

        for i in 0..self.arr.len(){
            if self.arr[i].0 == sym{
                counter += 1;
            }
        }
        return counter;
    }

    pub fn set_sym(&mut self, pos: Vec2, sym: char){
        for i in 0..self.arr.len(){
            if pos == self.arr[i].1{
                self.arr[i].0 = sym;
            }
        }
    }

    pub fn get_sym(&self, pos: Vec2) -> char{
        for i in 0..self.arr.len(){
            if pos == self.arr[i].1{
                return self.arr[i].0;
            }
        }
        return 'e';
    }
}
