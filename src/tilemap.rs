#![allow(incomplete_features)]

#[derive(Copy, Clone, PartialEq)]
pub struct Vec2 {pub x: i16, pub y: i16}
impl Vec2 {
    pub fn new(x: i16, y: i16) -> Vec2{
        return Vec2{x: x, y: y};
    }
    pub fn def() -> Vec2{ return Vec2{x: 0, y: 0}; }
    pub fn print(&self){
        println!("({},{})",self.x, self.y);
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
        let new_obj = Object{position: pos, sym: sym, is_static: stat, is_on_floor: false};
        //toadd.push(self);
        return new_obj;
    }
    pub fn moveto(&mut self, new_pos: Vec2) {
        self.position = new_pos;
    }
    pub fn rename(&mut self, new_sym: char) {
        self.sym = new_sym;
    }
}

pub struct PolygonObject {
    pub points: Vec<Vec2>,
    pub sym: char,
    pub is_static: bool
}
impl PolygonObject {
    pub fn new(sym: char, stat: bool) -> PolygonObject {
        let new_obj = PolygonObject{ points: vec![] , sym: sym, is_static: stat };
        return new_obj;
    }
    pub fn add(&mut self, pos: Vec2) {
        self.points.push(pos);
    }
    pub fn move_point(&mut self, index: i16, new_pos: Vec2) {
        self.points[index as usize] = new_pos;
    }
    pub fn rename(&mut self, new_sym: char) {
        self.sym = new_sym;
    }
}

pub struct TileMap {
    arr: Vec<(char, Vec2)>,
    size: Vec2
}
impl TileMap{
    pub fn new(sym: char, x: i16, y: i16) -> TileMap {
        //let mut tmp_world: TileMap = TileMap{ arr: [(sym,Vec2::def()); (x * y) as usize]};
        let mut tmp_world: TileMap = TileMap { arr: vec![(sym,Vec2::def()); (x * y) as usize], size: Vec2::new(x, y)};
        let mut tmp_pos = Vec2::new(0,0);

        for i in 0..x * y{
            if tmp_pos.x < (x - 1) as i16{
                tmp_world.arr[i as usize].1 = Vec2::new(tmp_pos.x, tmp_pos.y);
                tmp_pos.x += 1;
            }
            else{
                tmp_world.arr[i as usize].1 = Vec2::new(tmp_pos.x, tmp_pos.y);

                tmp_pos.y += 1;
                tmp_pos.x = 0;
            }
        }
        return tmp_world;
    }

    pub fn read(&self){
        let mut tmp_pos: Vec2 = Vec2::new(0,0);
        let mut tmp_string: String = "".to_string();
        
        for i in 0..self.size.x * self.size.y{
            if tmp_pos.x < (self.size.x - 1) as i16{
                tmp_pos.x += 1;
                
                tmp_string += " ";
                tmp_string += &(self.arr[i as usize].0).to_string();
            }
            else{
                tmp_pos.y += 1;
                tmp_pos.x = 0;
                
                tmp_string += " ";
                tmp_string += &(self.arr[i as usize].0).to_string();
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
