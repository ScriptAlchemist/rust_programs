use std::collections::HashMap;
use std::fs;

fn main() -> std::io::Result<()> {
    // Create a hash map mapping keywords to emoji characters
    let mut emoji = HashMap::new();
    emoji.insert("smile", "😃");
    emoji.insert("bear", "🐻");
    emoji.insert("hamburger", "🍔");
    emoji.insert("lightbulb", "💡");
    emoji.insert("idea", "💡");
    emoji.insert("comment", "💬");
    emoji.insert("chat", "💬");
    emoji.insert("pomo", "🍅");
    emoji.insert("stop", "🛑");
    emoji.insert("warning", "⚠️");
    emoji.insert("rant", "🤬");
    emoji.insert("tv", "📺");
    emoji.insert("update", "📰");
    emoji.insert("tux", "🐧");
    emoji.insert("facepalm", "🤦");
    emoji.insert("puke", "🤢");
    emoji.insert("skull", "💀");
    emoji.insert("wizard", "🧙");
    emoji.insert("redX", "❌");
    emoji.insert("checkmark", "✅");
    emoji.insert("lock", "🔐");
    emoji.insert("bluedot", "🔵");

    // Read input from the file
    let input = fs::read_to_string(std::env::args().nth(1).unwrap())?;

    // Replace all occurrences of the keywords with the corresponding emoji characters
    let mut output = input;
    for (keyword, emoji_char) in &emoji {
        output = output.replace(&format!(":{}:", keyword), emoji_char);
    }

    // Write the modified text back to the file
    fs::write(std::env::args().nth(1).unwrap(), output)?;

    Ok(())
}

