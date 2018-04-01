#[allow(warnings)]
//#![warn(unused_imports)]
//#![warn(unused_variables)]

use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;

struct Game {
    scenes: [Scene; 10],
    actual_scene: i32,
}

impl Game {
//pub fn new() -> Game{ cant initiate empty struct ---------------------
    /*fn new() -> Game{
        return Game {
            scenes: {},
            actual_scene: 0,
        };
    }*/
    fn set_actual_scene(&mut self, actual_scene: i32) {
        self.actual_scene = actual_scene;
    }
    fn get_actual_scene(&self) -> i32 {
        return self.actual_scene;
    }
    fn get_scene(&self, position: usize) -> &Scene{
        return &self.scenes[position];
    }
    #[allow(warnings)]
    fn add_scene(&mut self, scene: Scene) { //
        //scenes.p
    }
}

struct Scene {
    id: i32,
    title: String,
    description: String,
    itens: [Object; 100],
}

impl Scene {
    fn set_id(&mut self, id: i32) {
        self.id = id;
    }
    fn get_id(&mut self) -> i32 {
        return self.id;
    }
    fn set_title(&mut self, title: String) {
        self.title = title;
    }
    fn get_title(&self) -> &String {
        return &self.title;
    }
    fn set_description(&mut self, description: String) {
        self.description = description;
    }
    fn get_description(&self) -> &String {
        return &self.description;
    }
    fn add_objects() {
        
    }
    fn recover_objects() {
        
    }
}

struct Inventory {
    id: i32,
    objects: [Object; 100],
}

impl Inventory {
    fn set_id(&mut self, id: i32) {
        self.id = id;
    }
    fn get_id(&self) -> i32 {
        return self.id;
    }
    fn add_objects() { //função do jogador (get Object)
        
    }
    fn recover_objects() {
        
    }
}

struct Object {
    id: i32,
    type_obj: String,
    name: String,
    description: String,
    positive_result: String,
    negative_result: String,
    correct_command: String,
    target_scene: i32,
    resolved: bool,
    obtained: bool,
}

impl Object {
    fn pick_object(&mut self) { //função comando do jogador
        self.obtained = true;
    }
    fn check_object(&self) { //função comando do jogador
        println!("\n{}\n", self.description);
    }
    fn use_object() { //função comando do jogador

    }
    fn set_resolved(&mut self, state: bool) {
        self.resolved = state;
    }
    fn get_resolved(&self) -> bool {
        return self.resolved;
    }
    fn set_obtained(&mut self, state: bool) {
        self.obtained = state;
    }
    fn get_obtained(&self) -> bool {
        return self.obtained;
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn get_name(&self) -> &String { //error: cannot move out of borrowed content
        return &self.name;          //solution: return a reference
    }
}

//let x = Game::new();
#[allow(warnings)]
fn list_inventory(inventory: Inventory) { //move to inventory impl
    //let inventory_objects: i32 = inventory.get_objects();
    //print
}

fn save_game(filename: &str) {
    println!("{}", filename);
}

fn load_saved_game(filename: &str) {
    println!("{}", filename);
}

fn new_game() {
    //let mut new_game = Game;
    println!("Reiniciar jogo? (S/N):\n");
}

fn get_help() {

}

fn use_inv_object_with_scene_object() { //função comando do jogador (mudar estado do objeto cena e inventario)

}

fn parse_user_command(command: &str) { //identificar qual comando foi digitado pelo jogador
    println!("{}", command);
}

fn main() {
    // --snip--
    let filename = "text.json";
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    //println!("With text:\n{}", contents);
    let mut n = 1;
    while n!=3 {
        let mut input = String::new();
        println!("/>");
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                //println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
        n = n + 1;
    }
}
