use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const NUM_PLAYERS : usize =2; //Number of players, we have 2 players
const NUM_MOLECULE_TYPES : usize = 5; //Number of molecule types we have 5 molecules
const NUM_EXPERTISE_TYPES : usize =5; //Number of expertise type it is not important for this game

#[derive(Debug,Copy,Clone)] //derive(something) means that you call something.
enum Module{
    Diagnosis,
    Molecules,
    Laboratory,
    StartPos,
}
//enum cannot derive Default that's why we implemented it
impl Default for Module{
    fn default() -> Self{
        Module::Diagnosis
    }
}
//Rust is not like OO languages.
//For this reason we have to create struct it is like "class" in Java.
//Also we derive Copy and Clone because, we will start cloning these states.
//It is sample struct. We created it because, we will use its items. Also we will call them.
#[derive(Debug,Copy,Clone,Default)]
struct Sample{
    id:i32,
    health:i32,
    cost: [i32; NUM_MOLECULE_TYPES],
}
//There is implement code.
//impl is can you implement a trait for a struct.
//We didn't do because we dont have to do it.
impl Sample{
}

//There is Player struct. We created it cuz, we will use its items.
//Also its item will be change so we had to store them.
#[derive(Debug,Clone,Default)]
struct Player{
    target : Module,
    score : i32,
    store : [i32; NUM_MOLECULE_TYPES],
    expertise : [i32; NUM_EXPERTISE_TYPES],
    samples: Vec<Sample>,
}

impl Player{

}

#[derive(Debug,Clone,Default)]
struct State{
    players: [Player; NUM_PLAYERS],
    avaible: [i32; NUM_MOLECULE_TYPES],
    samples: Vec<Sample>,
}

impl State{

    fn load_stdin() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let project_count = parse_input!(input_line, i32);
        for i in 0..project_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let a = parse_input!(inputs[0], i32);
            let b = parse_input!(inputs[1], i32);
            let c = parse_input!(inputs[2], i32);
            let d = parse_input!(inputs[3], i32);
            let e = parse_input!(inputs[4], i32);
        }
        State::default()

    }

    fn read_players(&mut self){

        for i in 0..NUM_PLAYERS as usize {
            //We create all player in players[] like an object
            let mut player = &mut self.players[i];
            player.samples.clear();

            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();

            //There is "target". Target is player's item.
            //Also target is a Module.
            //In here, we put target instance player.
            //And we check target's name
            //Cuz, the input may wrong name (we wrote panic for this.)
            player.target = match inputs[0].trim() {
                "DIAGNOSIS" => {Module::Diagnosis},
                "MOLECULES" => {Module::Molecules},
                "LABORATORY"=> {Module::Laboratory},
                "START_POS" => {Module::StartPos},
                module => {panic! ("Unknown Module: {}", module);},
            };
            let eta = parse_input!(inputs[1], i32);

            //We put score in instance player
            player.score = parse_input!(inputs[2], i32);

            //We put storage_a ..e in instance player
            //we stored storage in this loop.
            // This loop takes input's 3 to up number of molecules. (3+i)
            for i in 0..NUM_MOLECULE_TYPES{
                player.store[i] = parse_input!(inputs[3+i], i32);
            }

            //We stored expertise but it is not necessary for this game.
            for i in 0..NUM_EXPERTISE_TYPES{
                player.expertise[i] = parse_input!(inputs[8+i], i32);
            }


        }
    }

    fn read_turn(&mut self){

        self.read_players();
        self.samples.clear();


        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        for i in 0..NUM_MOLECULE_TYPES{
            self.avaible[i] = parse_input!(inputs[0+i], i32);
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let sample_count = parse_input!(input_line, i32);
        for i in 0..sample_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();

            //We have to do mut sample the reason is we need much sample.
            //I commented a line cuz it is not necessary
            let mut sample = Sample::default() ;
            sample.id = parse_input!(inputs[0], i32);
            sample.health = parse_input!(inputs[4], i32);
            //let sample_id = parse_input!(inputs[0], i32);

            let carried_by = parse_input!(inputs[1], i32);
            let rank = parse_input!(inputs[2], i32);
            let expertise_gain = inputs[3].trim().to_string();

            //We need cost that is inside sample struct.
            //So we take them.
            for i in 0..NUM_MOLECULE_TYPES{
                sample.cost[i] = parse_input!(inputs[5+i], i32);
            }

            //It is rule for this game.
            if carried_by == -1 {
                self.samples.push(sample);
            } else {
                self.players[carried_by as usize].samples.push(sample);
            }

        }
    }

    fn take_turn(&self) {
        //In this method, we have 4 step.
        // player's target can equal StartPos, Diagnosis, Laboratory and Molecules.
        // We can achieve these Module enum.
        match self.players[0].target{
            Module::StartPos => {
                self.step_lost();
            },

            Module::Diagnosis => {
                self.stepDiagnosis();
            },

            Module::Laboratory => {
                self.stepLabrotory();
            },

            Module::Molecules => {
                self.stepMolecules();
            },
        };
    }

    //If the player's target equal to diagnosis:
    // if we have no samples then Connect id
    // otherwise go to molecule to take sample.
    //This is rule for this game
    fn stepDiagnosis(&self) {
        let us = &self.players[0];
        if us.samples.is_empty() {
            println!("CONNECT {}", self.samples[0].id);
        }
        else{
            println!("GOTO MOLECULES")
        }
    }

    //If the player's target equal to laboratory:
    // id there is no sample then go to diagnosis.
    // otherwise connect id.
    //This is rule for this game
    fn stepLabrotory(&self) {
        let us  = &self.players[0];
        if us.samples.is_empty() {
            self.step_lost();
            return;
        }

        println!("CONNECT {}", self.players[0].samples[0].id);

    }

    //If player's target is equal to Molecules:
    // if there is no samples then go to diagnosis.
    // otherwise, that means we have some molecules. (A, B, C, D ,E)
    // and if the player's storage < sample's cost , then Connect the molecule.
    //  if not, go to molecule.
    fn stepMolecules(&self) {
        let us= &self.players[0];
        if us.samples.is_empty() {
            self.step_lost();
            return;
        }
        let sample = &us.samples[0];

        for i in 0..NUM_MOLECULE_TYPES{
            if us.store[i] < sample.cost[i] {
                //It is hard to implement in Rust. Cuz, there is no some data structure.
                // in here, if 'i' is 0 than "(('A' as u8)+ (i as u8)) as char"
                // it will be return A + 0 = A
                // Also,
                // A + 1 = B
                // A + 2 = C
                // A + 3 = D
                // A + 4 = E
                println!("CONNECT {}", (('A' as u8)+ (i as u8)) as char);
                return;
            }
        }
        println!("GOTO LABORATORY");
    }

    //If the player's target equal Step post:
    // then goto diagnosis.
    fn step_lost(&self){
        println!("GOTO DIAGNOSIS");
    }

}



/**
 * Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
 **/
fn main() {
    let mut state = State::load_stdin();

    // game loop
    loop {

        state.read_turn();


        state.take_turn();
        // println!("GOTO DIAGNOSIS");
    }
}