
extern crate rand;
extern crate ansi_term;

use std::io;
use rand::distributions::{Distribution, Uniform};
use ansi_term::Colour;


pub fn choose_set_verbs (list_verbs: Vec<[&'static str; 3]>) -> [&'static str; 3] {   
    let mut index_set = rand::thread_rng();
    let die_1 = Uniform::from(0..117);
    let index_verbs_set: usize = die_1.sample(&mut index_set); 
    let result_set_verbs = list_verbs[index_verbs_set];
    result_set_verbs

}

pub fn choose_form_verb(set_verb:[&'static str; 3]) {
    let mut index_form = rand::thread_rng();
    let die_2 = Uniform::from(0..3);
    let index_verbs_form = die_2.sample(&mut index_form); 

    println!("=================================================");

    let pointer = "-> ? <-";

    match index_verbs_form {
        0 => println!("         {}  --  {}  --  {}",Colour::Red.italic().paint(pointer), Colour::Cyan.italic().paint(set_verb[1]), Colour::Cyan.italic().paint(set_verb[2])),
        1 => println!("         {}  --  {}  --  {}", Colour::Cyan.italic().paint(set_verb[0]), Colour::Red.italic().paint(pointer), Colour::Cyan.italic().paint(set_verb[2])),
        2 => println!("         {}  --  {}  --  {}", Colour::Cyan.italic().paint(set_verb[0]), Colour::Cyan.italic().paint(set_verb[1]), Colour::Red.italic().paint(pointer)),
        _ => println!("fuck"),

    }
    println!("{}", Colour::Cyan.paint(String::from("-------------------------------------------------")));
    
    let mut result_input = String::new();
    let yes = "Yes!";
    let no = "No!";
    
    io::stdin()
        .read_line(&mut result_input)
        .expect(" ");

    if result_input.to_lowercase().trim() == set_verb[index_verbs_form] {
        println!("{}", Colour::Green.paint(String::from("-------------------------------------------------")));
        println!("{}     {}  --  {}  --  {}", Colour::Green.bold().paint(yes), Colour::Green.italic().paint(set_verb[0]), Colour::Green.italic().paint(set_verb[1]), Colour::Green.italic().paint(set_verb[2]));
    } else if result_input.trim() == String::from("") {
        println!("{}", Colour::Yellow.paint(String::from("-------------------------------------------------")));
        println!("{}", Colour::Yellow.paint(String::from("Where is the answer?")));
    } else {
        println!("{}", Colour::Red.paint(String::from("-------------------------------------------------")));
        println!("{}      {}  --  {}  --  {}",Colour::Red.bold().paint(no), Colour::Green.italic().paint(set_verb[0]), Colour::Green.italic().paint(set_verb[1]), Colour::Green.italic().paint(set_verb[2]));
    }
}
    
pub fn list_verbs() -> Vec<[&'static str; 3]> {
    vec![
        ["begin", "began", "begun"],
        ["drink", "drank", "drunk"],
        ["ring", "rang", "rung"],
        ["sing", "sang", "sung"],
        ["swim", "swam", "swum"],
        ["sink", "sank", "sunk"],
        ["blow", "blew", "blown"],
        ["draw", "drew", "drawn"],
        ["grow", "grew", "grown"],
        ["know", "knew", "known"],
        ["throw", "threw", "thrown"],
        ["fly", "flew", "flown"],
        ["drive", "drove", "driven"],
        ["hide", "hid", "hidden"],
        ["bite", "bit", "bitten"],
        ["ride", "rode", "ridden"],
        ["rise", "rose", "risen"],
        ["write", "wrote", "written"],
        ["eat", "ate", "eaten"],
        ["break", "broke", "broken"],
        ["speak", "spoke", "spoken"],
        ["steal", "stole", "stolen"],
        ["freeze", "froze", "frozen"],
        ["choose", "chose", "chosen"],
        ["wake", "woke", "woken"],
        ["fall", "fell", "fallen"],
        ["shake", "shook", "shaken"],
        ["take", "took", "taken"],
        ["forget", "forgot", "forgotten"],
        ["get", "got", "gotten"],
        ["forgive", "forgave", "forgiven"],
        ["give", "gave", "given"],
        ["bring", "brought", "brought"],
        ["buy", "bought", "bought"],
        ["catch", "caught", "caught"],
        ["fight", "fought", "fought"],
        ["teach", "taught", "taught"],
        ["think", "thought", "thought"],
        ["seek", "sought", "sought"],
        ["cost", "cost", "cost"],
        ["cut", "cut", "cut"],
        ["fit", "fit", "fit"],
        ["hit", "hit", "hit"],
        ["hurt", "hurt", "hurt"],
        ["let", "let", "let"],
        ["put", "put", "put"],
        ["quit", "quit", "quit"],
        ["set", "set", "set"],
        ["shut", "shut", "shut"],
        ["split", "split", "split"],
        ["upset", "upset", "upset"],
        ["knit", "knit", "knit"],
        ["spread", "spread", "spread"],
        ["broadcast", "broadcast", "broadcast"],
        ["burst", "burst", "burst"],
        ["cast", "cast", "cast"],
        ["lay", "laid", "laid"],
        ["pay", "paid", "paid"],
        ["make", "made", "made"],
        ["say", "said", "said"],
        ["become", "became", "become"],
        ["come", "came", "come"],
        ["hang", "hung", "hung"],
        ["dig", "dug", "dug"],
        ["stick", "stuck", "stuck"],
        ["sting", "stung", "stung"],
        ["read", "read", "read"],
        ["bleed", "bled", "bled"],
        ["feed", "fed", "fed"],
        ["lead", "led", "led"],
        ["feel", "felt", "felt"],
        ["deal", "dealt", "dealt"],
        ["build", "built", "built"],
        ["hold", "held", "held"],
        ["keep", "kept", "kept"],
        ["leave", "left", "left"],
        ["lend", "lent", "lent"],
        ["mean", "meant", "meant"],
        ["meet", "met", "met"],
        ["send", "sent", "sent"],
        ["sleep", "slept", "slept"],
        ["spend", "spent", "spent"],
        ["sweep", "swept", "swept"],
        ["bend", "bent", "bent"],
        ["burn", "burst", "burst"],
        ["dream", "dreamt", "dreamt"],
        ["lean", "leant", "leant"],
        ["learn", "learnt", "learnt"],
        ["smell", "smelt", "smelt"],
        ["hear", "heard", "heard"],
        ["flee", "fled", "fled"],
        ["sell", "sold", "sold"],
        ["tell", "told", "told"],
        ["find", "found", "found"],
        ["bind", "bound", "bound"],
        ["wind", "wound", "wound"],
        ["be", "was/were", "been"],
        ["see", "saw", "seen"],
        ["sew", "sewed", "sewn"],
        ["show", "showed", "shown"],
        ["swear", "swore", "sworn"],
        ["tear", "tore", "torn"],
        ["wear", "wore", "worn"],
        ["bear", "bore", "borne"],
        ["stand", "stood", "stood"],
        ["understand", "understood", "understood"],
        ["lose", "lost", "lost"],
        ["shoot", "shot", "shot"],
        ["win", "won", "won"],
        ["go", "went", "gone"],
        ["do", "did", "done"],
        ["run", "ran", "run"],
        ["have", "had", "had"],
        ["sit", "sat", "sat"],
        ["lie", "lay", "lain"],
        ["shine", "shone", "shone"],
        ["light", "lit", "lit"],
    ]
}

