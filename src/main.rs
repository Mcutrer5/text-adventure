use std::{thread, time};
use std::io::{stdout, stdin, Write};

mod graphics;

#[allow(dead_code)]

// Function that causes the text to be printed character by character
fn scroll_print(text : &str, wait_time: u64, new_line: bool) {
    for c in text.chars() {
        let mut lock = stdout().lock();
        write!(lock, "{}", c).unwrap();
        thread::sleep(time::Duration::from_millis(wait_time));
        lock.flush().unwrap();
    }
    if new_line {
        println!();
    }
}

// Actions :: Death
fn you_died(reason: &str) {
    let death_text = "You died because ";
    scroll_print(death_text, 50, false);
    scroll_print(reason, 50, true);
    graphics::print_game_over();
    // End the game
    std::process::exit(0);
}

// Characters :: Guard
fn guard() {
    graphics::print_guard();
    scroll_print("You attempt to sneak past the guard, he's still sleeping.", 50, true);
    scroll_print("Suddenly you knocked a wooden cask over with a mug on it.", 50, false);
    scroll_print("...", 1000, false);
    println!("CRASSH!");
    scroll_print("The guard wakes up and looks at you.", 50, true);
    scroll_print("Guard: Oi, what'r you doing 'ere?", 50, true);

    // Guard is not moving initally
    let mut guard_moved = false;

    // When a player dies, it calls you_died() and it exits the program
    // When a player escapes through the door, you return to the previous function which
    // called this function.

    loop {
        let next_action = input("[run | door] > ");
        if next_action == "run" && guard_moved {
            let reason = "the guard was faster than he looks and your world goes dark.";
            you_died(reason);
        } else if next_action == "run" && !guard_moved {
            scroll_print("The guard jumps and looks the other way, missing you entirely", 50, true);
            guard_moved = true;
        } else if next_action == "door" && guard_moved {
            scroll_print("You run to the door and open it, you're free!", 50, true);
            break;
        } else if next_action == "door" && !guard_moved {
            let reason = "the guard was faster than he looks and your world goes dark.";
            you_died(reason);
        } else {
            scroll_print("Not sure what that is but you can't do that.", 50, true);
        }
    }
}

// Rooms
fn blissful_ignorance_of_illusion_room() {
    graphics::print_chest();
    // The variable treasure_chest is an object type called a collection
    let treasure_chest = vec!["Diamonds", "Gold", "Silver", "Sword"];
    let treasure_chest_text = 
        "You see a room with a wooden treasure chest on the left and a sleeping guard on the right in front of the door";
    scroll_print(treasure_chest_text, 50, true);
    let action = input("What do you do? \n[Open Chest | Leave it alone ] > ");

    let chosen_option = parse_input(&action, 
            &[
                "open chest", 
                "leave it alone",
                "go left",
                "left",
                "treasure",
                "chest",
                "open",
                "dabloons"
        ]);

    if chosen_option { 
        open_chest(treasure_chest);
    } else {
        scroll_print("You decide to leave the chest alone and walk to the door.", 50, true);
        guard();
    }
}

fn open_chest(items: Vec<&str>) {
    scroll_print("You open the chest and see a bunch of shiny things.", 50, true);
    scroll_print("You take a closer look and see...", 50, true);
    for item in 0..items.len() {
        scroll_print(items[item], 50, true);
    }
    
    let choice = input("What do you want to do? > ");
    let options = [
        "take all",
        "take",
        "diamonds",
        "gold",
        "silver",
        "sword",
        "take sword",
        "take diamonds",
        "take gold"
    ];
    let parse_choice = parse_input(&choice, &options);

    if parse_choice {
        scroll_print("\tWoo! Bounty and a new sword!", 50, true);
        scroll_print("\tYou just received: ", 50, false);
        for item in 0..items.len() {
            
            if items[item] != items[items.len() - 1] && items[item] != items[items.len() - 2] { 
                scroll_print(items[item], 50, false);
                scroll_print(", ", 50, false); 
            } else if items[item] == items[items.len()-2] {
                scroll_print(items[item], 50, false);
                scroll_print(" and a ", 50, false);
            } else {
                scroll_print(items[item], 50, false);
                scroll_print("!", 50, false);
            }
        }
    } else {
        scroll_print("I sure hope it's still here when I get past this guard...", 50, true);
    }
    guard();
}

fn painful_truth_of_reality_room(name: String) {
    graphics::print_monster();
    scroll_print("There you see the great evil monster...", 50, true);
    scroll_print("He, it, whatever stares at you and you begin to go insane.", 40, true);
    let next_move = input("Do you flee or fight? > ");

    if next_move == "flee" {
        scroll_print("You flee and run back to the door. Smart choice, tbh.", 50, true);
        start_adventure(name);
    } else if next_move == "fight" {
        scroll_print("You fight and you die.", 50, true);
        scroll_print("I mean really what were you expecting? Actually hold on.", 50, true);
        let alt_ending = 
        "You fight and you die. But you're a hero and you die with honor.\n
        Your family is proud of you and you're remembered as a hero.\n
        The monster will soon be defeated by somebody who was smart enough to\n
        come in with a gun and shoot it in the head.\n
        I mean really what were you expecting? A bunch of sewer rats in a\n
        trench coat?\n
        Nevertheless, you died a hero and that's what matters.\n
        The end.";
        scroll_print(alt_ending, 10, true);
        let reason = "you became a delicious snack. \n\n...Not that kind of snack.";
        you_died(reason);
    } else {
        scroll_print("Not sure what that is but you can't do that.", 50, true);
        painful_truth_of_reality_room(name);
    }
}

fn start_adventure(name: String) {
    graphics::print_dungeon();
    let text = format!("Welcome {}! You enter a room and see a red door to your left that says 'Prison'
    and a blue door to your right that says 'Painful Truth to Reality'\n
    Which door do you choose? > ", name);
    let next_move = input(&text);

    let prison_door = parse_input(&next_move, &["prison", "red", "left"]);

    if prison_door {
        blissful_ignorance_of_illusion_room();
    } else if !prison_door {
        painful_truth_of_reality_room(name);
    } else {
        scroll_print("Not sure what that is but you can't do that.", 50, true);
        start_adventure(name);
    }
}

fn get_name() -> String {
    let name = input("What is your name? > ");
    let alt_name = "Furry Yiff :3";
    if name == "" {
        scroll_print("You didn't enter a name so I'm just going to call you Furry Yiff :3", 50, true);
        return alt_name.to_string();
    }
    return name;
}

fn parse_input(input : &str, options : &[&str]) -> bool {
    for option in options {
        if input.to_lowercase().contains(option) {
            return true;
        }
    }
    return false;
}

fn input(msg : &str) -> String {
    let mut input = String::new();
    print!("{}", msg);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    
    return input.trim().to_string();
}

fn main() {
    // this statement clears the terminal
    print!("{}[2J", 27 as char);
    let player_name = get_name();
    let _name = player_name.clone();
    start_adventure(player_name);
    scroll_print("The End", 100, true);
    let end_text = format!("You have reached the end of the game. Thanks {} for playing!", &_name);
    scroll_print(end_text.as_str(), 50, true);
}