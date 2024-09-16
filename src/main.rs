use std::{io::{self, Read}, path::{Path, StripPrefixError}};
// STRUCTS
struct Display { // EVERYTHING | the entire display with all sub-menus

}

struct DisplayBars { // TOP LEFT |bars showing health, migma and lups

} 

struct Map { // TOP CENTER | the Map
    y: usize, // y size
    x: usize, // x size
    fields: Vec<Vec<Object>> // the fields, accessible by: fields[y][x]. y,x = coordinates
}
impl Map { // functionality for the map
    fn new(y: usize, x: usize) -> Map {
        let fields = (0..y)
            .map(|_| {
                (0..x)
                    .map(|_| Object::new(ObjectKind::Path))
                    .collect::<Vec<Object>>()
            })
            .collect::<Vec<Vec<Object>>>();
        Map {
            y,
            x,
            fields,
        }
    }
    fn draw(&self, player: &Player) {
        for y in 0..self.y {
            for x in 0..self.x {
                if y == player.position_y && x == player.position_x {
                    print!("{}", player.character)
                } else {
                    print!("{}", self.fields[y][x].character)
                }
            }
            println!();
        }
    }
}

struct Object { // Objects (1x1 on the map) like Path, Wall and Chest
    kind: ObjectKind,
    character: String,
    walkable: bool,
}
#[derive(PartialEq)]
enum ObjectKind { // Objects on the Map
    Wall,
    Path,
    Chest
}
impl Object {
    fn new(kind: ObjectKind) -> Object {
        let walkable = match kind {
            ObjectKind::Path => true,
            ObjectKind::Wall => false,
            ObjectKind::Chest => false
        };
        let character = match kind {
            ObjectKind::Path => String::from(" "),
            ObjectKind::Wall => String::from("◼"),
            ObjectKind::Chest => String::from("📦")
        };
        Object {
            kind,
            character,
            walkable,
        }
    }
}

struct Inventory { // TOP RIGHT | the Inventory
    items: Vec<Item>
}
enum Item {
    Sword, //{damage: usize},
    Axe,
    Pickaxe
}

struct InfoBox { // BOTTOM | box that shows info like map/area name

}

struct Player { // the Player
    health: usize,
    character: String,
    position_y: usize,
    position_x: usize
}
impl Player {
    fn new(health: usize, character: String) -> Player {
        Player {
            health,
            character,
            position_y: 5,
            position_x: 5
        }
    }
    fn movement(&mut self, map: &Map, direction: String) {
        match direction.as_str(){
            "W" | "w" | "up" => if self.position_y > 0 && map.fields[self.position_y - 1][self.position_x].kind == ObjectKind::Path {
                self.position_y -= 1
            },
            "A" | "a" | "right" => if self.position_y > 0 && map.fields[self.position_y][self.position_x - 1].kind == ObjectKind::Path {
                self.position_x -= 1
            }
            "S" | "s" | "down" => if self.position_y < (map.y - 1) && map.fields[self.position_y + 1][self.position_x].kind == ObjectKind::Path {
                self.position_y += 1
            },
            "D" | "d" | "left" => if self.position_x < (map.x - 1) && map.fields[self.position_y][self.position_x + 1].kind == ObjectKind::Path {
                self.position_x += 1
            }
            _ => return
        }
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_input(map: &Map) -> String {
    //let mut input = &mut String::new();
    //io::stdin().read_line(&mut input).expect("Too long.");
    print!("\x1B[{};0H", map.y + 1);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    String::from(input) // return input
}

fn main() {
    clear_terminal();
    let map = Map::new(9,9); // initialize the map
    let mut player = Player::new(10, String::from("P"));
    map.draw(&player);
    loop {
        player.movement(&map, get_input(&map));
        map.draw(&player)
    }
}
