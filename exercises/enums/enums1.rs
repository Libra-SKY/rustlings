// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!



#[derive(Debug)]
enum Message {   Quit,
                 Echo,
                 Move,
                 ChangeColor
    // TODO: define a few types of messages as used below
}

impl Default for Message {
    fn default() -> Self {
        Self::ChangeColor
    }
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
