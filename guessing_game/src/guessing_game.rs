use cursive::views::Dialog;
use cursive::Cursive;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(Dialog::text("This is a guessing game!\nPress <Next> when you're ready.\n\nThe rules of this game are simple.\n\nYou will have to select a difficulty (East, Medium, or Hard).\n\nEasy is a choice between 1-20\n\nMedium is a choice between 1-50\n\nHard is a choice between 1-100 with only 5 guesses possible")
        .title("Guessing Game")
        .button("Quit",  |s| s.quit())
        .button("Next", show_next));

    // siv.add_layer(TextView::new("Hello Cursive! Press <q> to quit."));

    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Select a difficulty (East, Medium, or Hard).\n\nEasy is a choice between 1-20\n\nMedium is a choice between 1-50\n\nHard is a choice between 1-100 with only 5 guesses possible")
        .title("Pick a level")
        .button("Easy", try_easy_guess)
        .button("Medium", try_medium_guess)
        .button("Hard", try_hard_guess));
}

fn try_easy_guess(s: &mut Cursive) {
    s.add_layer(Dialog::text("Playing Easy Mode")
        .title("Take your guess!")
        .button("Guess", you_win)
        .button("Cancel", |s| {
            s.pop_layer();
        })
        .button("Quit", |s| s.quit()));

}

fn try_medium_guess(s: &mut Cursive) {
    s.add_layer(Dialog::text("Playing Medium Mode")
        .title("Take your guess!")
        .button("Guess", you_win)
        .button("Cancel", |s| {
            s.pop_layer();
        })
        .button("Finish", |s| s.quit()));

}

fn try_hard_guess(s: &mut Cursive) {
    s.add_layer(Dialog::text("Playing Hard Mode")
        .title("Take your guess!")
        .button("Guess", you_win)
        .button("Cancel", |s| {
            s.pop_layer();
        })
        .button("Finish", |s| s.quit()));

}
fn you_win(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("")
        .title("You win!")
        .button("Finish", |s| s.quit()));

}
