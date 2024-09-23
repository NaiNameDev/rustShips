use rand::Rng;

use std::io;

#[derive(Copy, Clone, PartialEq)]
struct Vec2 {x: i8, y: i8}
impl Vec2{
    fn new(x: i8, y: i8) -> Vec2{
        return Vec2{x: x, y: y};
    }
    fn def() -> Vec2{ return Vec2{x: 0, y: 0}; }
    fn print(&self){
        println!("{},{}",self.x, self.y);
    }
}

const WORLD_X: usize = 8;
const WORLD_Y: usize = 8;

struct World {
    arr: [(char, Vec2); WORLD_X * WORLD_Y]
}
impl World{
    fn new(sym: char) -> World {
        let mut tmp_world: World = World{ arr: [('x',Vec2::def()); WORLD_X * WORLD_Y] };
        let mut tmp_pos = Vec2::new(0,0);

        for i in 0..WORLD_X * WORLD_Y{
            if tmp_pos.x < (WORLD_X - 1) as i8{
                tmp_world.arr[i].1 = Vec2::new(tmp_pos.x, tmp_pos.y);
                tmp_world.arr[i].0 = sym;

                tmp_pos.x += 1;
            }
            else{
                tmp_world.arr[i].1 = Vec2::new(tmp_pos.x, tmp_pos.y);
                tmp_world.arr[i].0 = sym;

                tmp_pos.y += 1;
                tmp_pos.x = 0;
            }
        }
        return tmp_world;
    }

    fn read(&self, colums: bool){
        let mut tmp_pos: Vec2 = Vec2::new(0,0);
        let mut tmp_string: String = "".to_string();
        
        if colums == true{
            println!("  0 1 2 3 4 5 6 7");
            tmp_string += "0";
        } // еу это рубрика кастыли
        
        for i in 0..WORLD_X * WORLD_Y{
            if tmp_pos.x < (WORLD_X - 1) as i8{
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
                if colums == true{
                    tmp_string += &tmp_pos.y.to_string();
                }
            }
        }
    println!("{}",tmp_string); 
    }
    
    fn num_of_syms(&self, sym: char) -> i8{
        let mut counter: i8 = 0;

        for i in 0..self.arr.len(){
            if self.arr[i].0 == sym{
                counter += 1;
            }
        }
        return counter;
    }

    fn set_sym_by_pos(&mut self, pos: Vec2, sym: char){
        for i in 0..self.arr.len(){
            if pos == self.arr[i].1{
                self.arr[i].0 = sym;
            }
        }
    }

    fn get_sym_by_pos(&self, pos: Vec2) -> char{
        for i in 0..self.arr.len(){
            if pos == self.arr[i].1{
                return self.arr[i].0;
            }
        }
        return 'e';
    }
}

fn read(text: &str) -> String{
    let mut input = String::new();
    
    println!("{}", text.to_string());
    io::stdin().read_line(&mut input);

    return input;
}

fn to_int(text: &str) -> i8{
    let num: i8 = text.trim().parse()
        .expect("ERROR!!");
    
    return num;
}

fn main(){
    let mut rng = rand::thread_rng();

    let mut bot_world = World::new('~');

    for i in 0..3{
        bot_world.set_sym_by_pos(Vec2::new(rng.gen_range(0..WORLD_X as i8),rng.gen_range(0..WORLD_Y as i8)), '$');
    }

    let mut attack_world = World::new('~');
    attack_world.read(true);
    
    let mut own_world = World::new('~');
    let mut ships_left: i8 = 3;
    println!("select cells for your ships");

    //ready loop
    loop {    
        if ships_left != 0{
            println!("ships to set left: {}\n", ships_left);
        }
        else{
            println!("all ships ready for the battle!!");
            break;
        }

        let tmpx: i8 = to_int(&read("enter x: "));
        let tmpy: i8 = to_int(&read("enter y: "));
    
        let tmp_pos = Vec2::new(tmpx, tmpy);

        if own_world.get_sym_by_pos(tmp_pos) != 'K' && tmp_pos.x <= WORLD_X as i8 && tmp_pos.y <= WORLD_Y as i8{
            own_world.set_sym_by_pos(tmp_pos, 'K');
            ships_left -= 1;
        }
        else{
            println!("incroect position");
        }
        println!("your field -->");
        own_world.read(true);
    }
    
    println!("");
    println!("#######################");
    println!("###THE GAME STARTS!!###");
    println!("#######################");
    println!("");

    println!("choose cell to attack!!");

    //attck loop
    loop {
        //info
        let your_ships_left = own_world.num_of_syms('K');
        let ships_left = bot_world.num_of_syms('$');
        if ships_left != 0{
            println!("enemy ships left: {}", ships_left);
            println!("your ships left: {}\n", your_ships_left)
        }
        else{
            println!("YOU WIN!!!");
            println!("enemy ships left: {}", ships_left);
            println!("your ships left: {}\n", your_ships_left);
            break;
        }

        if your_ships_left == 0{
            println!("YOU LOST!!!");
            println!("enemy ships left: {}", ships_left);
            println!("your ships left: {}\n", your_ships_left);
            break;
        }

        //your attack

        let tmpx: i8 = to_int(&read("enter x: "));
        let tmpy: i8 = to_int(&read("enter y: "));
    
        let tmp_pos = Vec2::new(tmpx, tmpy);

        if bot_world.get_sym_by_pos(tmp_pos) == '$'{
            attack_world.set_sym_by_pos(tmp_pos, 'X');
            bot_world.set_sym_by_pos(tmp_pos, 'X');
        }
        else{
            attack_world.set_sym_by_pos(tmp_pos, 'O');
        }
        println!("your enemy field -->");
        attack_world.read(true);
        
        //bot attack
        let bot_attack = loop {
            let tmp = Vec2::new(rng.gen_range(0..WORLD_X as i8),rng.gen_range(0..WORLD_Y as i8));
            if own_world.get_sym_by_pos(tmp) != 'X' && own_world.get_sym_by_pos(tmp) != 'O'{
                break tmp;
            }
        };
        println!("bot attack: x:{}, y:{}",bot_attack.x, bot_attack.y);
        
        if own_world.get_sym_by_pos(bot_attack) == 'K'{
            own_world.set_sym_by_pos(bot_attack, 'X');
            println!("BOT KILLS YOUR SHIP");
        }
        else{
            println!("bot miss his attack!");
            own_world.set_sym_by_pos(bot_attack, 'O');
        }
        
        println!("your field -->");
        own_world.read(true);
    }
}
