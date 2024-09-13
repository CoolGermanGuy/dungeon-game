// STRUCTS
struct Display { // EVERYTHING | the entire display with all sub-menus

}

struct DisplayBars { // TOP LEFT |bars showing health, migma and lups

} 

struct Map { // TOP CENTER | the Map
    size_y: usize,
    size_x: usize,
}
enum Object { // Objects on the Map
    Wall, //{walkable: bool},
    Path,
    Chest
}

struct Inventory { // TOP RIGHT | the Inventory
    
}
enum Item {
    Sword, //{damage: usize},
    Axe,
    Pickaxe
}

struct InfoBox { // BOTTOM | box that shows info like map/area name

}

struct Player { // the Player

}

fn main() {
    println!("Hello, world!");
}
