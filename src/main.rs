use irreg::*;

fn main() {
    loop {
        let list_verbs = list_verbs();
        let result_set_verbs = choose_set_verbs(list_verbs);
    
        choose_form_verb(result_set_verbs);
    }
    
}

