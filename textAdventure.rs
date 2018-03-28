//use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Game {
    scenes: [Scene; 10],
    actual_scene: i32,
}

impl Game {
    fn set_actual_scene (&mut self, actual_scene: i32) {
        self.actual_scene = actual_scene;
    }

    fn get_actual_scene (&self) -> i32{
        return self.actual_scene;
    }
}

struct Scene {
    id: i32,
    title: String,
    description: String,
    itens: [Object; 100],
}

impl Scene {
    

}

struct Inventory {
    objects: [Object; 100],
}

impl Inventory {

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
    fn pick_object() {

    }
}

fn save_game (filename: &str) {
    println!("{}", filename);
}

fn load_saved_game (filename: &str) {
    println!("{}", filename);
}

fn new_game () {
    println!("Reiniciar jogo? (S/N):\n");
}

fn parse_user_command (command: &str) { //identificar qual comando foi digitado pelo jogador
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

    println!("With text:\n{}", contents);
}