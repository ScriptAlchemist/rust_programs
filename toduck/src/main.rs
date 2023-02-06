use std::io::{self, BufRead, BufReader, Write};

fn toduck() -> io::Result<()> {
        let mut out = io::stdout();
            let reader = BufReader::new(io::stdin());
                let duck = "https://lite.duckduckgo.com/lite?kd=-1&kp=-1&q=";
                    for line in reader.lines() {
                                let line = line?;
                                        if let Some(capture) = line.find("[") {
                                                        let end = line[capture..].find("]()").unwrap() + capture;
                                                                    let text = &line[capture + 1..end];
                                                                                write!(out, "{}[{}]({}{})", &line[..capture], text, duck, text)?;
                                                                                        } else {
                                                                                                        write!(out, "{}", line)?;
                                                                                                                }
                                                                                                                        writeln!(out)?;
                                                                                                                            }
                                                                                                                                Ok(())
}

fn main() {
        if let Err(err) = toduck() {
                    eprintln!("Error: {}", err);
                        }
}
