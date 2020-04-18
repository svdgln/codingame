use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const NUM_PLAYERS : usize =2;
const NUM_MOLECULE_TYPES : usize = 5;
const NUM_EXPERTISE_TYPES : usize =5;

#[derive(Debug,Copy,Clone)]
enum Module{
    Diagnosis,
    Molecules,
    Laboratory,
    StartPos,
}

impl Default for Module{
    fn default() -> Self{
        Module::Diagnosis
    }
}

#[derive(Debug,Copy,Clone,Default)]
struct Sample{
    id:i32,
    health:i32,
    cost: [i32; NUM_MOLECULE_TYPES],
}

impl Sample{
}

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
            let mut player = &mut self.players[i];
            player.samples.clear();

            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();

            player.target = match inputs[0].trim() {
                "DIAGNOSIS" => { Module::Diagnosis},
                "MOLECULES" => {Module::Molecules},
                "LABORATORY"=> {Module::Laboratory},
                "START_POS" => {Module::StartPos},
                module => {panic! ("Unknown Module: {}", module);},
            };
            let eta = parse_input!(inputs[1], i32);
            player.score = parse_input!(inputs[2], i32);

            for i in 0..NUM_MOLECULE_TYPES{
                player.store[i] = parse_input!(inputs[3+i], i32);
            }

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

            let mut sample = Sample::default() ;
            sample.id = parse_input!(inputs[0], i32);
            sample.health = parse_input!(inputs[4], i32);


            //let sample_id = parse_input!(inputs[0], i32);
            let carried_by = parse_input!(inputs[1], i32);
            let rank = parse_input!(inputs[2], i32);
            let expertise_gain = inputs[3].trim().to_string();

            for i in 0..NUM_MOLECULE_TYPES{
                sample.cost[i] = parse_input!(inputs[5+i], i32);
            }

            if carried_by == -1 {
                self.samples.push(sample);
            } else {
                self.players[carried_by as usize].samples.push(sample);
            }

        }
    }

    fn take_turn(&self) {
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

    fn stepDiagnosis(&self) {
        let us = &self.players[0];
        if us.samples.is_empty() {
            println!("CONNECT {}", self.samples[0].id);
        }
        else{
            println!("GOTO MOLECULES")
        }
    }

    fn stepLabrotory(&self) {
        let us  = &self.players[0];
        if us.samples.is_empty() {
            self.step_lost();
            return;
        }

        println!("CONNECT {}", self.players[0].samples[0].id);

    }

    fn stepMolecules(&self) {
        let us= &self.players[0];
        if us.samples.is_empty() {
            self.step_lost();
            return;
        }
        let sample = &us.samples[0];

        for i in 0..NUM_MOLECULE_TYPES{
            if us.store[i] < sample.cost[i] {
                println!("CONNECT {}", (('A' as u8)+ (i as u8)) as char);
                return;
            }
        }
        println!("GOTO LABORATORY");
    }

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