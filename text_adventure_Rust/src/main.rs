//#[allow(warnings)]
//#![warn(unused_imports)]
//#![warn(unused_variables)]

use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Write};




#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

//#[derive(Serialize, Deserialize, Debug)]
/*
struct Save{
    game:Game,
    inventory:Inventory,
}
impl Save{
    fn new(game:Game,inventory:Inventory)->Self{
        Self{game:game,inventory:inventory}
    }
    fn get_game(&self)->Game{
        return self.game;
    }
    fn get_inventory(&self)->Inventory{
        return self.inventory;
    }
}
*/


#[derive(Serialize, Deserialize, Debug)]

struct Game {
    actual_scene: i32,
    scenes:Vec<Scene>,
}


impl Game {
     fn new()->Self{
        Self{actual_scene:1,scenes:Vec::new()}
    }
    fn set_actual_scene(&mut self, actual_scene: i32) {
        self.actual_scene = actual_scene;
    }
    fn get_actual_scene(&self) -> i32 {
        return self.actual_scene;
    }
    fn get_scene(&mut self, position:i32) -> &mut Scene{
        let mut aux=position; 
        aux=aux-1;//cena começa em 1 vector indexa a partir de 0
        return &mut self.scenes[aux as usize];
    }
    #[allow(warnings)]
    fn add_scene(&mut self, mut scene:Scene) { //Considerando ids de 1 a 10
            self.scenes.push(scene);
    }
}

#[derive(Serialize, Deserialize, Debug)]

struct Scene {
    id: i32,
    title: String,
    description: String,
    itens:Vec<Object>
}

impl Scene {
    fn new(id:i32,title:&str,description:&str)->Self{
        Self{id:id,title:title.to_owned(),description:description.to_owned(),itens:Vec::new()}
    }
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
    fn add_objects(&mut self,object:Object) {
         self.itens.push(object)
    }
    fn get_objects(&mut self)->&mut Vec<Object>{
        return &mut self.itens;
    }
    fn aux_get_object(&mut self,id:i32)->Option<&mut Object>{
        for obj in self.get_objects(){
            if(obj.get_id()==id){
                return Some(obj);
            }       
        }
        return None;
    }
   fn get_object(&mut self,id:i32)->&mut Object{
        let result=self.aux_get_object(id);
        return result.unwrap();
    }

    fn list_objects(&mut self) {
        for i in &self.itens{
            print!("{} ",i.get_name());
        }
    }
    fn verify_obj(&mut self,nameObj:&str)->i32{
    let objectsScene= self.get_objects();
    for obj in objectsScene{ // #VERFICAR -> ACHO QUE NÃO PRECISA DE & PARA ITERAR OBJ ELE JA É &MUT -> da && erro se colocar
        if(obj.get_name()==nameObj){
            return obj.get_id();
        }
    }
    return -1; //"FALSE"
    }

     fn aux_get_object_clone(&mut self,id:i32)->Option<Object>{
        for obj in self.get_objects(){
            if(obj.get_id()==id){
                return Some(obj.clone());
            }       
        }
        return None;
    }
   fn get_object_clone(&mut self,id:i32)->Object{
        let result=self.aux_get_object_clone(id);
        return result.unwrap();
    }

}

#[derive(Serialize, Deserialize, Debug)]
struct Inventory {
    objects:Vec<Object>
}

impl Inventory {
    fn new()->Self{
        Self{objects:Vec::new()}
    }
    fn add_objects(&mut self,object:Object) { //função do jogador (get Object)  INSTANCIA DO OBJETO SERA *TRANSFERIDA* PARA O INVENTARIO
         self.objects.push(object)
    }
    fn list_objects(&mut self) {
        if(self.objects.len()==0){
            println!("Mochila está vazia.");
        }
        for i in &self.objects{
            print!("{} " ,i.get_name());
        }
        print!("\n");
    }
    fn get_objects(&mut self)->&mut Vec<Object>{
        return &mut self.objects;
    }

    fn verify_obj(&mut self,nameObj:&str)->i32{
        let objectsInvetory= self.get_objects();
        for obj in objectsInvetory{ //NÃO PRECISA DE & PARA ITERAR OBJ ELE JA É &MUT -> da && erro se colocar
            if(obj.get_name()==nameObj){
                return obj.get_id();
            }
       }
        return -1; //"FALSE"

    }
    fn aux_get_object(&mut self,id:i32)->Option<&Object>{
        for obj in self.get_objects(){
            if(obj.get_id()==id){
                return Some(obj);
            }       
        }
        return None;
    }
   fn get_object(&mut self,id:i32)->&Object{
        let result=self.aux_get_object(id);
        return result.unwrap();
    }

}

#[derive(Serialize, Deserialize, Debug)]

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
     fn new(id:i32,type_obj:&str,name:&str,description:&str,positive_result:&str,negative_result:&str,correct_command:&str,target_scene:i32,resolved: bool,obtained: bool) ->  Self
    {
        Self{id:id,type_obj:type_obj.to_owned(),name:name.to_owned(),description:description.to_owned(),positive_result:positive_result.to_owned(),negative_result:negative_result.to_owned(),correct_command:correct_command.to_owned(),target_scene:target_scene,resolved:resolved,obtained:obtained}
    }
    fn pick_object(&mut self) { //função comando do jogador
        self.obtained = true;
    }
    fn check_object(&self) { //função comando do jogador
        println!("\n{}\n", self.description);
    }
    fn use_object() { //função comando do jogador
        
    }
    fn set_id(&mut self, id: i32) {
        self.id = id;
    }
    fn get_id(&self) -> i32 {
        return self.id;
    }
    fn set_target_scene(&mut self, target: i32) {
        self.target_scene = target;
    }
    fn get_target_scene(&self) -> i32 {
        return self.target_scene;
    }
    fn set_correct_command(&mut self, command: String) {
        self.correct_command = command;
    }
    fn get_correct_command(&self) -> &str {
        let str: &str = &self.correct_command[..]; 
        return str;
    }
    fn set_positive_result(&mut self, result: String) {
        self.positive_result = result;
    }
    fn get_positive_result(&self) -> &String {
        return &self.positive_result;
    }
    fn set_negative_result(&mut self, result: String) {
        self.positive_result = result;
    }
    fn get_negative_result(&self) -> &String {
        return &self.negative_result;
    }
    fn set_description(&mut self, description: String) {
        self.description = description;
    }
    fn get_description(&self) -> &String {
        return &self.description;
    }
    fn set_type_obj(&mut self, type_obj: String) {
        self.type_obj = type_obj;
    }
    fn get_type_obj(&self) -> &String {
        return &self.type_obj;
    }
    fn set_resolved(&mut self) {
        self.resolved = true;
    }
    fn get_resolved(&self) -> bool {
        return self.resolved;
    }
    fn set_obtained(&mut self) {
        self.obtained = true;
    }
    fn get_obtained(&self) -> bool {
        return self.obtained;
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn get_name(&self) -> &str { //Como se passa uma referencia(emprestimo), tem que retornar uma referencia, para retornar apenas um String tem que fazer um .copy()
        let str: &str = &self.name[..]; 
        return str;          //
    }
    fn clone(&mut self) -> Object {
        return(Object::new(self.get_id(),self.get_type_obj(),self.get_name(),self.get_description(),self.get_positive_result(),self.get_negative_result(),self.get_correct_command(),self.get_target_scene(),self.get_resolved(),self.get_obtained()))
    }
    
}

#[allow(warnings)]
/*
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
*/

fn clean(){
    print!("{}[2J", 27 as char);

}
fn get_help() {
println!("\nmochila -> mostra itens na mochila\nuse OBJETO -> interagir com objeto da cena (abrir, usar, pressionar, ...)\nuse ITEM with OBJETO -> usar item do inventário em objeto da cena\ncheck OBJETO -> descreve objeto da cena\nget OBJETO -> coloca objeto na mochila\nexit -> sai do jogo\n");
}

fn use_inv_object_with_scene_object() { //função comando do jogador (mudar estado do objeto cena e inventario)

}


fn msg_delirium(){
    println!("Você está vendo coisas melhor ir para sombra...");
}

fn aux_parse_user_command(command: &str,inventory:&mut Inventory,sceneID:i32,scene:&mut Scene)->(bool,i32) { //identificar qual comando foi digitado pelo jogador
    let mut actualCommmand =command.trim(); //trim tira os espaços do começo fim e \n
    let commands: Vec<&str> = actualCommmand.split(" ").collect(); //vetoriza o split em elementos do tipo &str
    let mut nextScene=-1;
    if(commands[0]=="exit"){
        if(commands.len()!=1){
            println!("Digite />help para obter ajuda");
            return (false,nextScene);
        }
        return (true,nextScene);
    }

    if(commands[0]=="check"){
        if(commands.len()!=2){
            println!("tente:/>check OBJETO.");
            println!("Digite />help para obter ajuda.");
            return (false,nextScene);
        }
        let objectIDScene=scene.verify_obj(commands[1]); 
        let objectIDInventory=inventory.verify_obj(commands[1]);
        if(objectIDScene==-1 && objectIDInventory==-1){ // -1 -> não encontrou nem no inventario nem na cena
            msg_delirium();
            return (false,nextScene);
        }

        else if(objectIDInventory==-1){ // objeto está apenas na cena
            let obj=scene.get_object(objectIDScene);
            println!("{}",obj.get_description());
        }

        else{ // objeto está nos dois ou apenas no inventario-> certamente no inventario
           let obj=inventory.get_object(objectIDInventory);
           println!("{}",obj.get_description());
        }
    }


    else if(commands[0]=="get"){
        if(commands.len()!=2){
            println!("tente:/>get OBJETO.");
            println!("Digite />help para obter ajuda.");
            return (false,nextScene);
        }

        let objectID=scene.verify_obj(commands[1]); 

        if(objectID==-1){ ///n exite na cena
            msg_delirium();
            return (false,nextScene);
        }
        {
        let mut obj=scene.get_object(objectID);
        
        if(obj.get_type_obj()=="SCENE_OBJECT"){
            println!("Melhor não carregar isso ai...");
            return (false,nextScene);
        }


        if(obj.get_obtained()){
            println!("Você já pegou isso, sua memória está começando a falhar,melhor descansar um pouco...");
            return (false,nextScene);
        }
        obj.set_obtained();
        }
        /* É preciso definir um escopo de emprestimo... 
        let mut obj=scene.get_object(objectID);
    |                     ----- first mutable borrow occurs here

    |   inventory.add_objects(scene.get_object_clone(objectID));
    |                               ^^^^^ second mutable borrow occurs here
    */
        {
        inventory.add_objects(scene.get_object_clone(objectID));
        }

    }
    

    else if(commands[0]=="mochila"){
        if(commands.len()!=1){
            println!("Digite />help para obter ajuda.");
            return (false,nextScene);
        }

        inventory.list_objects();
    }




    else if(commands[0]=="use"){
        if(commands.len()==1){
            println!("Tente />use OBJECT");
            println!("Digite />help para obter ajuda.");
            return (false,nextScene);
        }

        // APENAS USE COMMAND
        if(commands.len()==2){  
            let objectIDScene=scene.verify_obj(commands[1]);
            if(objectIDScene==-1){
                msg_delirium();
                return (false,nextScene);
            }
            let obj=scene.get_object(objectIDScene);
            if(obj.get_correct_command()==actualCommmand && obj.get_type_obj()=="SCENE_OBJECT"){
                println!("{}",obj.get_positive_result());
                obj.set_resolved();
                nextScene=obj.get_target_scene();        
            }
            else{
                println!("{}",obj.get_negative_result());
            }

        }

        // USE + WITH
        else if(commands.len()==4 && commands[2]=="with"){
            let objectIDInventory=inventory.verify_obj(commands[1]);
            let objectIDScene=scene.verify_obj(commands[3]);
            if(objectIDInventory==-1 || objectIDScene==-1){ //verifica se está no inventario e cena
                msg_delirium();
                return(false,nextScene);
            }
            let objScene=scene.get_object(objectIDScene);

            if(objScene.get_correct_command()==actualCommmand){
                objScene.set_resolved();
                println!("{}",objScene.get_positive_result());
                nextScene=objScene.get_target_scene();
            }
            else{
                println!("{}",objScene.get_negative_result());
            }

        


        }        
        else{
            msg_delirium();
        }

    }


    else if(commands[0]=="help"){
         if(commands.len()!=1){
            println!("Digite />help para obter ajuda.");
            return (false,nextScene);
        }

        get_help();
    }

    else{
        msg_delirium();
    }

    
    return (false,nextScene);//Não entrou em =='exit' -> shell continua
}




fn pre_parser(command: &str,game:&mut Game,inventory:&mut Inventory)->(bool,i32) { //identificar qual comando foi digitado pelo jogador
    let(exit,nextScene)={
        let sceneID=game.get_actual_scene();
        let scene=game.get_scene(sceneID);
        aux_parse_user_command(&command.to_lowercase(),inventory,sceneID,scene)
    };

    if(exit==true){
        println!("\n(*) Deseja salvar jogo(S/N)?\n");
        print!("/>"); 
        std::io::stdout().flush();

        let mut string: String = String::new();
        std::io::stdin().read_line(&mut string);

        if(string.trim().to_lowercase()=="s"){
            println!("\n(*) Insira o nome do save\n");
            print!("/>"); 
            std::io::stdout().flush();
            let mut string: String = String::new();
            std::io::stdin().read_line(&mut string);
            save_game(game,inventory,&mut string);
            return (exit,nextScene);

        }
    
    }
    return (exit,nextScene);

}



fn parse_user_command(command: &str,game:&mut Game,inventory:&mut Inventory)->bool{// Existe? ->n existe heap global(tem como mas...),->get e set de borrowed mut game.
    let(exit,nextScene)=pre_parser(command,game,inventory);
    if(nextScene!=-1){
        game.set_actual_scene(nextScene);
        display(game,nextScene);
    }
    return exit;
}


fn display(game:&mut Game,sceneID:i32){
    let scene=game.get_scene(sceneID);
    clean();
    println!("{}\n",scene.get_title());
    println!("{}",scene.get_description());
    println!("\n\n\n\n\n");

}

fn save_game(game:&mut Game,inventory:&mut Inventory,saveName:&mut String){
    let delimiter="***";
    let mut save=(game,delimiter,inventory);
    let serialized = serde_json::to_string(&save).unwrap();

    let borrowed_string: &str = ".json";
    saveName.pop();// retira \n
    saveName.push_str(borrowed_string);
    let mut ofile = File::create(saveName).unwrap();
    ofile.write_all(serialized.as_bytes());
    ofile.flush();
    println!("\n(*) Save executado com exito\n");

    
    
}

fn new_game(file:&str)->(Game,Inventory){
    
    let mut game=Game::new();
    let mut inventory= Inventory::new();

    let mut ifile = File::open(file.trim()).expect("file not found");
    let mut contents = String::new();
    ifile.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.pop();// retira \n
    let aux=&contents[1..];
    let mut help: Vec<&str> = aux.split(",\"***\",").collect(); //vetoriza o split em elementos do tipo &str
    let mut content= String::new();

    content=help[0].to_owned();
    game=serde_json::from_str(&content).unwrap();
    content=help[1].to_owned();
    inventory=serde_json::from_str(&content).unwrap();
    return(game,inventory);

}


fn load_saved_game()->(Game,Inventory){
    println!("\n(*) Insira o nome do save a carregar\n");
    print!("/>"); 
    std::io::stdout().flush();

    let mut save: String = String::new();
    let borrowed_string: &str = ".json";
    std::io::stdin().read_line(&mut save);
    save.pop();// retira \n
    save.push_str(borrowed_string);
    return new_game(&save); 
    
    
}

fn init()->(Game,Inventory){
    clean();
    println!("<><><><>Aventura no deserto<><><><>");
    println!("\n(*) Iniciar novo jogo -> pressione 'n'\n");
    println!("\n(*) Carregar jogo -> pressione 'l'\n");

    print!("/>"); 
    std::io::stdout().flush();

    let mut string: String = String::new();
    std::io::stdin().read_line(&mut string);

    if(string.trim().to_lowercase()=="n"){
       return new_game("default.json");
    }
    else{
        return load_saved_game();
    }

}



fn main() {
    let mut inventory= Inventory::new();
    //"INVENTORY_OBJECT"
    //"SCENE_OBJECT"
    let mut feno = Object::new(1,"SCENE_OBJECT","feno","Uma planta estranha, que cobre a areia embaixo dela do sol","Você agora consegue se proteger um pouco sol, usando a planta como chapéu!","Há uma pequena cobra dentro, que so sairá morta!","use faca with feno",2,false,false);
    let mut pedra = Object::new(2,"INVENTORY_OBJECT","faca","Uma faca afiada comum","NENHUM","Não posso fazer isso com a faca","NENHUM",-1,false,false);
    let mut scene1= Scene::new(1,"No meio do nada [7:00 AM]","Na sua primeira noite de ferias, no deserto de Guban, você experimentou plantas locais, que provavelmente seriam barradas em um aeroporto. Uma caminhada pela madrugada e ai está você: perdido no deserto.O sol escaldante de Guban já nasceu, e está derretendo o que sobrou do seu cerebro, no horizente tudo que se vê são dunas de areia. Você tem uma mochila vazia nas costas e uma FACA proximo ao pé. Ao seu lado há uma planta que mais parece um FENO.");
    scene1.add_objects(feno);
    scene1.add_objects(pedra);


    let mut cacto = Object::new(3,"SCENE_OBJECT","cacto","Um animal com muita fome tentaria comer","Não foi a melhor refeição que você teve...","É preciso abrir esse cacto dos espinhos","use faca with cacto",-1,false,false);
    let mut cranio = Object::new(4,"INVENTORY_OBJECT","cranio","Um cranio qualquer","NENHUM","Não dá para fazer isso com um pedaço de osso","NENHUM",-1,false,false);
    let mut scene2= Scene::new(2,"No meio do nada [8:30 AM]","Já faz um bom tempo que você não come algo...Você ao olhar ao lado encontra um CRANIO de um camelo que já morreu há muito tempo... em meio a imensidão da areia você enxerga algumas plantas estranhas, dentre elas um pequeno CACTO redondo");
    scene2.add_objects(cacto);
    scene2.add_objects(cranio);

    
    
    
    let mut game=Game::new();
    game.add_scene(scene1);
    game.add_scene(scene2);
    

    
    let delimiter="***";
    let mut save=(game,delimiter,inventory);
    let serialized = serde_json::to_string(&save).unwrap();
    let mut ofile = File::create("default.json").unwrap();
    ofile.write_all(serialized.as_bytes());
    ofile.flush();
    


    
   
    

   
    let (game,inventory)=init();
    let mut game=game;
    let mut inventory=inventory;

    let mut sceneID=game.get_actual_scene();
    let finalSceneID=10;
    let mut end=true;

    display(&mut game,sceneID);


    while(end && sceneID!=finalSceneID){//controle do fluxo do jogo
            
            print!("/>"); 
            std::io::stdout().flush();
            let mut string: String = String::new();
            std::io::stdin().read_line(&mut string);
            let mut command: &str = &string[..];  // transformo string em &str
            let mut split =command.trim().split(" "); //trim tira os espaços do começo fim e \n,  dps separa por whitespace
            let commands: Vec<&str> = split.collect();
            if(parse_user_command(command,&mut game,&mut inventory)){//caso seja exit
                end=false;
            }   
            sceneID=game.get_actual_scene();
    }



}












//Mutability is inherited in Rust. That is, if you have a Foo stored in a mutable variable (let mut f: Foo), its fields are mutable;