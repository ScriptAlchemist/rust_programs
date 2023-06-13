use std::fs::File;
use std::io::Write;
use std::process::Command;
use chrono::{Utc, Datelike, Timelike};

fn main() -> Result<(), std::io::Error> {
    let now = Utc::now();
    let filename = format!("{}{}{}{}{}{}.md",
        now.year(),
        format!("{:02}", now.month()),
        format!("{:02}", now.day()),
        format!("{:02}", now.hour()),
        format!("{:02}", now.minute()),
        format!("{:02}", now.second()));

    let mut file = File::create(filename.clone())?;
    let formatted_now = now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    let content = format!(r#"---
title: ''
excerpt: ''
coverImage: '/assets/blog/img_bin/'
date: '{}'
author:
  name: 'Justin Bender'
  picture: '/assets/blog/authors/bender.png'
ogImage:
  url: '/assets/blog/img_bin/'
---

## Title

    "#, formatted_now);

    file.write_all(content.as_bytes())?;

    // Open Vim
    let mut child = Command::new("vim.bat")
        .arg(&filename)
        .spawn()
        .expect("Failed to open file in Vim");

    let _ = child.wait();
    Ok(())
}
