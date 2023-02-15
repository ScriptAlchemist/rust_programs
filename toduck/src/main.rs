use std::fs;
use regex::Regex;

fn main() -> std::io::Result<()> {

  // Read input from the file
  let input = fs::read_to_string(std::env::args().nth(1).unwrap())?;

  // Replace all occurrences of [word thing]() with [word thing]("https://lite.duckduckgo.com/lite?kd=-1&kp=-1&q=word%20thing")
  let re = Regex::new(r"\[([^\]]+)\]\(\)").unwrap();
  let output = re.replace_all(&input, |caps: &regex::Captures| {
      let word = caps.get(1).unwrap().as_str();
      format!("[{}](https://lite.duckduckgo.com/lite?kd=-1&kp=-1&q={})", word, word.replace(" ", "%20"))
  });

  // Write the modified text back to the file
  fs::write(std::env::args().nth(1).unwrap(), output.to_string())?;

  Ok(())
}

