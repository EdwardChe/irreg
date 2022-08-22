use std::io;
use rand::distributions::{Distribution, Uniform};
use ansi_term::Colour;

use irreg::*;

fn main() {
    println!("
Modes:
    1 - random checking one from three a form each verb
    2 - none
    3 - none
Enter the mode number.");

    let mut mode_selection = String::new();
    io::stdin()
        .read_line(&mut mode_selection)
        .expect(" ");
    let mode_selection = mode_selection.trim().parse();  
    
    match mode_selection {
        Ok(1) => loop {
            let list_verbs = list_verbs();
            let result_set_verbs = choose_set_verbs(list_verbs);

        // receive index verb for question
            let mut index_form = rand::thread_rng();
            let die_2 = Uniform::from(0..3);
            let index_verbs_form = die_2.sample(&mut index_form); 

        // send question
            choose_form_verb(result_set_verbs, index_verbs_form);

        // receive response
            let mut result_input = String::new();
            io::stdin()
            .read_line(&mut result_input)
            .expect(" ");

        // quit from app
            if result_input.trim() == "exit" {
                println!("-------------------------------------------------");
                println!("Goodbye!");
                break;
            } else {
                analysis_of_response(&result_input, result_set_verbs, index_verbs_form); 
            }
        },
        _ => {
            println!("{}", Colour::Red.paint(String::from("--------------------------------------")));
            println!("{}", Colour::Red.paint(String::from("| You have entered an invalid value! |")));
            println!("{}", Colour::Red.paint(String::from("--------------------------------------")));
        },
    }  
}

