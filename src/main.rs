#![allow(dead_code)]

mod ffi {
    pub enum Language {
        English,
        German,
        French,
    }
}

enum Greeting {
    Hello,
    Bye,
}

use ffi::*;

impl Greeting {
    fn translate(&self, language: Language) -> String {
        match (self, language) {
            (Greeting::Hello, Language::English) => "Hello, World!",
            (Greeting::Hello, Language::German) => "Hallo, Welt!",
            (Greeting::Hello, Language::French) => "Bonjour, le monde!",
            (Greeting::Bye, Language::English) => "Bye!",
            (Greeting::Bye, Language::German) => "Auf Wiedersehen!",
            (Greeting::Bye, Language::French) => "Au revoir!",
        }
        .to_string()
    }
}

fn main() {
    println!("Hello, world!");
}
