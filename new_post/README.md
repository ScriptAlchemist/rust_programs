# New Post Generator

A Rust-based command-line tool for generating blog post markdown files with interactive GUI prompts for title, excerpt, and image selection.

## Features

- Interactive Terminal User Interface (TUI) for all inputs
- Vim-like navigation for image selection
- Automatic timestamp-based filename generation
- Environment variable configuration for post and image directories
- Opens generated file directly in Zed editor
- Cross-platform support (macOS, Linux, Windows)

## Prerequisites

- Rust and Cargo installed
- Zed editor installed and accessible via command line
- Environment variables configured (see setup below)
- Terminal that supports ANSI colors and cursor positioning

## Installation

1. Clone or download this project
2. Install using cargo:
   ```bash
   cargo install --path .
   ```
   This will install the binary to `~/.cargo/bin/new_post` which should already be in your PATH if Rust is properly configured.

## Environment Setup

Before using the tool, set these required environment variables in your shell profile:

```bash
export JUST_IN_POSTS="/path/to/your/blog/posts"
export JUST_IN_PICTURES="/path/to/your/blog/images"
```

Example for a typical blog structure:
```bash
export JUST_IN_POSTS="$HOME/projects/myblog/_posts"
export JUST_IN_PICTURES="$HOME/projects/myblog/public/assets/blog/img_bin"
```

## Usage

Simply run the command from anywhere:
```bash
new_post
```

The tool will guide you through a beautiful TUI interface:

1. **Title Input**: Type your blog post title directly in the terminal
2. **Excerpt Input**: Type a brief excerpt for your post
3. **Cover Image Selection**: Browse and select from existing images with vim-like navigation (j/k or ↑↓)
4. **OG Image Selection**: Choose a different image or use the same one (s to skip/use same)
5. **Confirmation Screen**: Review all details before creating the post
6. **File Generation**: Creates a markdown file with timestamp-based filename
7. **Auto-open**: Opens the generated file in Zed editor

## Generated File Structure

The tool creates markdown files with this front matter structure:

```yaml
---
title: 'Your Post Title'
excerpt: 'Your post excerpt'
coverImage: '/assets/blog/img_bin/cover-image.jpg'
date: '2024-01-15T10:30:45.123Z'
author:
  name: 'Justin Bender'
  picture: '/assets/blog/authors/bender.png'
ogImage:
  url: '/assets/blog/img_bin/og-image.jpg'
---

## Your Post Title

```

## File Naming

Files are automatically named using the current UTC timestamp in the format:
`YYYYMMDDHHMMSS.md`

Example: `20240115103045.md`

## Directory Structure

The tool expects and maintains this structure:
```
$JUST_IN_POSTS/           # Blog posts directory
├── 20240115103045.md     # Generated post files
└── ...

$JUST_IN_PICTURES/        # Images directory  
├── cover-image.jpg       # Copied images
├── og-image.png
└── ...
```

## Customization

To modify the author information or image paths, edit the template in `src/main.rs`:

```rust
let content = format!(r#"---
title: '{}'
excerpt: '{}'
coverImage: '{}'
date: '{}'
author:
  name: 'Your Name'           // Modify this
  picture: '/path/to/pic'     // Modify this
ogImage:
  url: '{}'
---
```

## TUI Navigation

The interface provides intuitive navigation:

**Text Input Fields:**
- Type directly to enter text
- Use arrow keys or Home/End to navigate within text
- Press Enter to submit and move to next field
- Press Esc or q to quit at any time

**Image Selection:**
- Use ↑↓ arrow keys or j/k (vim-style) to navigate
- Press Enter to select highlighted image
- Press s to skip and use default path
- Images are automatically discovered from `$JUST_IN_PICTURES`

**General Navigation:**
- F1: Toggle help screen
- Esc or q: Quit/Cancel
- Enter: Confirm/Select
- y/n: Yes/No confirmations

## Troubleshooting

**Environment variables not set**: Ensure `JUST_IN_POSTS` and `JUST_IN_PICTURES` are exported in your shell profile and restart your terminal.

**Zed not found**: Ensure Zed is installed and the `zed` command is available in your PATH.

**TUI not displaying correctly**: Ensure your terminal supports ANSI colors and has sufficient size (minimum 80x24 recommended).

**Images not showing**: Verify that `$JUST_IN_PICTURES` contains image files with supported extensions (jpg, jpeg, png, gif, webp, svg).

**Permission denied**: Make sure the directories specified in environment variables exist and are writable.

**Build fails**: Ensure you have Rust 1.70+ installed with `rustc --version`.

## Development

To build for development:
```bash
cargo build --release
```

To install locally:
```bash
cargo install --path .
```

The installed binary will be available at `~/.cargo/bin/new_post`.