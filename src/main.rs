// Is this a comment?

use bevy::prelude::*;
use tts::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

struct GreetTimer {
    timer: Timer,
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer {
            timer: Timer::from_seconds(2.0, true),
        })
        .add_startup_system(add_people)
        .add_system(greet_people);
    }
}

fn main() {
    let my_string = "Hemlo I am String";
    println!("Hello, world! {}", my_string);

    App::new()
        .init_resource::<Mytts>()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_system(keyboard_tts)
        .run();
}

fn hello_world() {
    println!("hello world 2!");
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Kevin".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Bob".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Sherlock".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.timer.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
    }
}

fn keyboard_tts(mut tts: ResMut<Mytts>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");

        tts.0.speak("You Win!", true).unwrap();
    }
}

impl FromWorld for Mytts {
    fn from_world(world: &mut World) -> Self {
        Mytts(Tts::default().unwrap())
    }
}

struct Mytts(Tts);
