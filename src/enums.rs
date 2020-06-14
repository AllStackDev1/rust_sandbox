/*
Enums - Are types which has a few definite values
*/

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avater(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avater moving up"),
        Movement::Down => println!("Avater moving down"),
        Movement::Left => println!("Avater moving left"),
        Movement::Right => println!("Avater moving right"),
    }
}

pub fn run() {
    let avater1 = Movement::Left;
    let avater2 = Movement::Up;
    let avater3 = Movement::Right;
    let avater4 = Movement::Down;

    move_avater(avater1);
    move_avater(avater2);
    move_avater(avater3);
    move_avater(avater4);
}
