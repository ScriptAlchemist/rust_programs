use cursive::theme::{BorderStyle, Palette};
use cursive::traits::With;
use cursive::views::Dialog;
use cursive::Cursive;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.set_theme(cursive::theme::Theme{
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;
            {
                use cursive::theme::Color::TerminalDefault;
                use cursive::theme::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Blue.light();
                palette[Highlight] = Blue.dark();
            }

            {
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Green.light()).combine(Bold);
            }

        }),
    });

    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(Dialog::text("This is a guessing game!\nPress <Next> when you're ready.\n\nCheck <Open Directions> to learn how to play the game\n\n")
        .title("Guessing Game")
        .button("Next", show_next)
        .button("Open Directions", show_dialog)
        .button("Quit",  |s| s.quit()));

    // siv.add_layer(TextView::new("Hello Cursive! Press <q> to quit."));

    siv.run();
}

fn show_dialog(s: &mut cursive::Cursive) {

    s.add_layer(Dialog::info("The rules of this game are simple.\n\nThis is a guessing game.\n\nYou will have to select a difficulty (East, Medium, or Hard).\n\nEasy is a choice between 1-20\n\nMedium is a choice between 1-50\n\nHard is a choice between 1-100 with only 5 guesses possible").title("Game Directions"));
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
    s.pop_layer();
    s.add_layer(Dialog::text("Playing Easy Mode")
        .title("Take your guess!")
        .button("Guess", you_win)
        .button("Quit", |s| s.quit()));

}

fn try_medium_guess(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Playing Medium Mode")
        .title("Take your guess!")
        .button("Guess", you_win)
        .button("Finish", |s| s.quit()));

}

fn try_hard_guess(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Playing Hard Mode")
        .title("Take your guess!")
        .button("Guess", you_win)
        .button("Finish", |s| s.quit()));

}
fn you_win(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("")
        .title("You win!")
        .button("Finish", |s| s.quit()));

}
