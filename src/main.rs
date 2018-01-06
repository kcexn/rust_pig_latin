extern crate pig_latin;

use std::env;
use std::process;

use pig_latin::OriginalWord;

fn main() {
    let word = OriginalWord::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    pig_latin::translate_to_pig_latin(word);
}
