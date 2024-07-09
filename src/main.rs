use bevy::prelude::*;
use rand::Rng;

use values::*;

mod values;


fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_dice));

    app.run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

fn spawn_dice(){
    let results = roll_dice(4, 6);
    
    for die in results {
        println!("Die: {}", die)
    }
}


pub fn roll_dice(mut num_dice: u32, die: u32) -> Vec<u32> {
    let mut dice_totals: Vec<u32> = vec![];

    while num_dice > 0 {
        let result = rand::thread_rng().gen_range(1..=die);
        dice_totals.push(result);
        num_dice -= 1;

}

dice_totals
}
