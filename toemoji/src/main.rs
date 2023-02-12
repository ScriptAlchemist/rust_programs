use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Create a hash map mapping keywords to emoji characters
    let mut emoji = std::collections::HashMap::new();
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

    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Replace all occurrences of the keywords with the corresponding emoji characters
    let mut output = input;
    for (keyword, emoji_char) in &emoji {
        output = output.replace(&format!(":{}:", keyword), emoji_char);
    }

    // Write the modified text to standard output
    io::stdout().write_all(output.as_bytes())?;

    Ok(())
}

