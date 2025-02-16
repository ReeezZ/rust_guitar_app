

#[derive(Debug)]
enum Notes {
    C,
    CisOrDes,
    D,
    // DisOrEs,
    // E,
    // F,
    // FisOrGes,
    // G,
    // GisOrAs,
}

impl Notes {
    fn iterator() -> impl Iterator<Item = Notes> {
        [Notes::C, Notes::CisOrDes, Notes::D].into_iter()
    }

    fn as_str(&self) -> &'static str {
        match self {
            Notes::C => "C",
            Notes::CisOrDes => "C♯/D♭",
            Notes::D => "D",
        }
    }
}

fn main() {
    for variant in Notes::iterator() {
        println!("{}", variant.as_str());
    }
}
