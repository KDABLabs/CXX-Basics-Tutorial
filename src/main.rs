#![allow(dead_code)]

#[cxx::bridge]
mod ffi {
    pub enum Language {
        English,
        German,
        French,
    }

    unsafe extern "C++" {
        // #include
        include!("cxx-basics/include/greeter.h");

        type Greeter;

        fn greet(self: &Greeter, greeting: &Greeting);

        #[rust_name = "make_greeting"]
        fn makeGreeting() -> UniquePtr<Greeter>;
    }

    extern "Rust" {
        type Greeting;

        fn translate(self: &Greeting, language: Language) -> String;
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
            _ => "🤯",
        }
        .to_string()
    }
}

fn main() {
    println!("Hello, world!");
}
