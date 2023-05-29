use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};


use rand::rngs::OsRng;
use rand::Rng;

pub fn generate_number(num: u8) -> u8 {
    let mut rng = OsRng;
    rng.gen_range(1..=num)
}

pub fn you_win(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("")
        .title("You win!")
        .button("Finish", |s| s.quit()));

}

pub fn you_lose(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("")
        .title("Sucker, you lose!")
        .button("Finish", |s| s.quit()));

}

pub fn show_rules(s: &mut cursive::Cursive) {

    s.add_layer(Dialog::info("The rules of this game are simple.\n\nThis is a guessing game.\n\nYou will have to select a difficulty (East, Medium, or Hard).\n\nEasy is a choice between 1-20\n\nMedium is a choice between 1-50\n\nHard is a choice between 1-100 with only 5 guesses possible").title("Game Directions"));
}

pub fn show_next(s: &mut Cursive) {
    s.pop_layer();
    let words = Dialog::text("Select a difficulty (East, Medium, or Hard).\n\nEasy is a choice between 1-20\n\nMedium is a choice between 1-50\n\nHard is a choice between 1-100 with only 5 guesses possible");

    let buttons = LinearLayout::vertical()
        .child(DummyView)
        .child(DummyView)
        .child(Button::new("Easy", |s| {
            try_easy_guess(s, None);
        }))
        .child(DummyView)
        .child(Button::new("Medium", |s| {
            try_medium_guess(s, None);
        }))
        .child(DummyView)
        .child(Button::new("Hard", |s| {
            try_hard_guess(s, None, 5);
        }));

    s.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(words)
            .child(DummyView)
            .child(buttons))
    .title("Pick a level"));
}

fn try_easy_guess(s: &mut Cursive, number: Option<u8>) {
    let random_number: u8 = match number {
        Some(num) => num,
        None => generate_number(20),
    };
    s.pop_layer();
    // s.add_layer(Dialog::text("Take your guess! Between 1 and 20")
        // .title("Playing Easy Mode")
        // .button("Guess", you_win)
        // .button("Quit", |s| s.quit()));

}

fn try_medium_guess(s: &mut Cursive, number: Option<u8>) {
    let random_number: u8 = match number {
        Some(num) => num,
        None => generate_number(50),
    };
    s.pop_layer();
    s.add_layer(Dialog::text("Take your guess! Between 1 and 50")
        .title("Playing Medium Mode")
        .button("Guess", you_win)
        .button("Finish", |s| s.quit()));

}

fn try_hard_guess(s: &mut Cursive, number: Option<u8>, tries_left: u8) {
    let random_number: u8 = match number {
        Some(num) => num,
        None => generate_number(100),
    };
    let tries_left_string = tries_left.to_string();

    s.pop_layer();
    s.add_layer(Dialog::text("Take your guess! Between 1 and 100\n\n".to_owned() + &tries_left_string + " tries left")
        .title("Playing Hard Mode")
        .button("Guess", you_win)
        .button("Finish", |s| s.quit()));

}
