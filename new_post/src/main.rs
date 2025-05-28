use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use chrono::{Utc, Datelike, Timelike};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rfd::FileDialog;

#[derive(Debug, Clone)]
struct BlogPost {
    title: String,
    excerpt: String,
    cover_image: String,
    og_image: String,
}

impl Default for BlogPost {
    fn default() -> Self {
        Self {
            title: String::new(),
            excerpt: String::new(),
            cover_image: String::from("/assets/blog/img_bin/"),
            og_image: String::from("/assets/blog/img_bin/"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum AppState {
    TitleInput,
    ExcerptInput,
    ImageSelectionMenu,
    ImageSelection,
    OgImageSelectionMenu,
    OgImageSelection,
    Confirmation,
    Error(String),
}

#[derive(Debug, PartialEq)]
enum ImageMenuOption {
    BrowseExternal,
    SelectFromFolder,
    Skip,
}

#[derive(Debug)]
struct App {
    state: AppState,
    blog_post: BlogPost,
    input_buffer: String,
    cursor_position: usize,
    posts_dir: PathBuf,
    pictures_dir: PathBuf,
    image_files: Vec<PathBuf>,
    image_list_state: ListState,
    og_image_files: Vec<PathBuf>,
    og_image_list_state: ListState,
    image_menu_selection: usize,
    og_image_menu_selection: usize,
    show_help: bool,
}

impl App {
    fn new(posts_dir: PathBuf, pictures_dir: PathBuf) -> io::Result<Self> {
        let image_files = get_image_files(&pictures_dir)?;
        let og_image_files = image_files.clone();
        
        Ok(Self {
            state: AppState::TitleInput,
            blog_post: BlogPost::default(),
            input_buffer: String::new(),
            cursor_position: 0,
            posts_dir,
            pictures_dir,
            image_files,
            image_list_state: ListState::default(),
            og_image_files,
            og_image_list_state: ListState::default(),
            image_menu_selection: 0,
            og_image_menu_selection: 0,
            show_help: false,
        })
    }

    fn handle_key_event(&mut self, key: KeyCode) -> io::Result<bool> {
        match self.state {
            AppState::TitleInput | AppState::ExcerptInput => {
                self.handle_text_input(key)
            }
            AppState::ImageSelectionMenu => {
                self.handle_image_menu(key, true)
            }
            AppState::ImageSelection => {
                self.handle_image_selection(key)
            }
            AppState::OgImageSelectionMenu => {
                self.handle_image_menu(key, false)
            }
            AppState::OgImageSelection => {
                self.handle_og_image_selection(key)
            }
            AppState::Confirmation => {
                self.handle_confirmation(key)
            }
            AppState::Error(_) => {
                if matches!(key, KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q')) {
                    return Ok(true);
                }
                Ok(false)
            }
        }
    }

    fn handle_text_input(&mut self, key: KeyCode) -> io::Result<bool> {
        match key {
            KeyCode::Enter => {
                if !self.input_buffer.trim().is_empty() {
                    match self.state {
                        AppState::TitleInput => {
                            self.blog_post.title = self.input_buffer.trim().to_string();
                            self.input_buffer.clear();
                            self.cursor_position = 0;
                            self.state = AppState::ExcerptInput;
                        }
                        AppState::ExcerptInput => {
                            self.blog_post.excerpt = self.input_buffer.trim().to_string();
                            self.input_buffer.clear();
                            self.cursor_position = 0;
                            self.state = AppState::ImageSelectionMenu;
                        }
                        _ => {}
                    }
                }
                Ok(false)
            }
            KeyCode::Char(c) if c != 'q' => {
                self.input_buffer.insert(self.cursor_position, c);
                self.cursor_position += 1;
                Ok(false)
            }
            KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    self.input_buffer.remove(self.cursor_position - 1);
                    self.cursor_position -= 1;
                }
                Ok(false)
            }
            KeyCode::Delete => {
                if self.cursor_position < self.input_buffer.len() {
                    self.input_buffer.remove(self.cursor_position);
                }
                Ok(false)
            }
            KeyCode::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
                Ok(false)
            }
            KeyCode::Right => {
                if self.cursor_position < self.input_buffer.len() {
                    self.cursor_position += 1;
                }
                Ok(false)
            }
            KeyCode::Home => {
                self.cursor_position = 0;
                Ok(false)
            }
            KeyCode::End => {
                self.cursor_position = self.input_buffer.len();
                Ok(false)
            }
            KeyCode::Esc | KeyCode::Char('q') => Ok(true),
            KeyCode::F(1) => {
                self.show_help = !self.show_help;
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_image_menu(&mut self, key: KeyCode, is_cover_image: bool) -> io::Result<bool> {
        let menu_selection = if is_cover_image { &mut self.image_menu_selection } else { &mut self.og_image_menu_selection };
        
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                if *menu_selection > 0 {
                    *menu_selection -= 1;
                }
                Ok(false)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if *menu_selection < 2 {
                    *menu_selection += 1;
                }
                Ok(false)
            }
            KeyCode::Enter => {
                match *menu_selection {
                    0 => { // Browse External
                        if let Some(selected_file) = browse_for_image() {
                            let copied_path = copy_image_to_pictures(&selected_file, &self.pictures_dir)?;
                            if is_cover_image {
                                self.blog_post.cover_image = copied_path;
                                self.state = AppState::OgImageSelectionMenu;
                            } else {
                                self.blog_post.og_image = copied_path;
                                self.state = AppState::Confirmation;
                            }
                        }
                        Ok(false)
                    }
                    1 => { // Select from Folder
                        if is_cover_image {
                            if !self.image_files.is_empty() {
                                self.image_list_state.select(Some(0));
                                self.state = AppState::ImageSelection;
                            } else {
                                self.state = AppState::OgImageSelectionMenu;
                            }
                        } else {
                            if !self.og_image_files.is_empty() {
                                self.og_image_list_state.select(Some(0));
                                self.state = AppState::OgImageSelection;
                            } else {
                                self.state = AppState::Confirmation;
                            }
                        }
                        Ok(false)
                    }
                    2 => { // Skip
                        if is_cover_image {
                            self.state = AppState::OgImageSelectionMenu;
                        } else {
                            self.blog_post.og_image = self.blog_post.cover_image.clone();
                            self.state = AppState::Confirmation;
                        }
                        Ok(false)
                    }
                    _ => Ok(false)
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => Ok(true),
            KeyCode::F(1) => {
                self.show_help = !self.show_help;
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_image_selection(&mut self, key: KeyCode) -> io::Result<bool> {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                let selected = self.image_list_state.selected().unwrap_or(0);
                if selected > 0 {
                    self.image_list_state.select(Some(selected - 1));
                }
                Ok(false)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let selected = self.image_list_state.selected().unwrap_or(0);
                if selected < self.image_files.len() - 1 {
                    self.image_list_state.select(Some(selected + 1));
                }
                Ok(false)
            }
            KeyCode::Enter => {
                if let Some(selected) = self.image_list_state.selected() {
                    if selected < self.image_files.len() {
                        let filename = self.image_files[selected]
                            .file_name()
                            .unwrap()
                            .to_string_lossy();
                        self.blog_post.cover_image = format!("/assets/blog/img_bin/{}", filename);
                        
                        self.og_image_list_state.select(Some(selected));
                        self.state = AppState::OgImageSelectionMenu;
                    }
                }
                Ok(false)
            }
            KeyCode::Char('s') => {
                self.state = AppState::OgImageSelectionMenu;
                Ok(false)
            }
            KeyCode::Esc | KeyCode::Char('q') => Ok(true),
            KeyCode::F(1) => {
                self.show_help = !self.show_help;
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_og_image_selection(&mut self, key: KeyCode) -> io::Result<bool> {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                let selected = self.og_image_list_state.selected().unwrap_or(0);
                if selected > 0 {
                    self.og_image_list_state.select(Some(selected - 1));
                }
                Ok(false)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let selected = self.og_image_list_state.selected().unwrap_or(0);
                if selected < self.og_image_files.len() - 1 {
                    self.og_image_list_state.select(Some(selected + 1));
                }
                Ok(false)
            }
            KeyCode::Enter => {
                if let Some(selected) = self.og_image_list_state.selected() {
                    if selected < self.og_image_files.len() {
                        let filename = self.og_image_files[selected]
                            .file_name()
                            .unwrap()
                            .to_string_lossy();
                        self.blog_post.og_image = format!("/assets/blog/img_bin/{}", filename);
                    }
                }
                self.state = AppState::Confirmation;
                Ok(false)
            }
            KeyCode::Char('s') => {
                self.blog_post.og_image = self.blog_post.cover_image.clone();
                self.state = AppState::Confirmation;
                Ok(false)
            }
            KeyCode::Esc | KeyCode::Char('q') => Ok(true),
            KeyCode::F(1) => {
                self.show_help = !self.show_help;
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_confirmation(&mut self, key: KeyCode) -> io::Result<bool> {
        match key {
            KeyCode::Char('y') | KeyCode::Enter => {
                match create_blog_post(&self.blog_post, &self.posts_dir) {
                    Ok(file_path) => {
                        open_in_zed(&file_path)?;
                        return Ok(true);
                    }
                    Err(e) => {
                        self.state = AppState::Error(format!("Failed to create blog post: {}", e));
                    }
                }
                Ok(false)
            }
            KeyCode::Char('n') | KeyCode::Esc | KeyCode::Char('q') => Ok(true),
            KeyCode::F(1) => {
                self.show_help = !self.show_help;
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check environment variables
    let posts_dir = env::var("JUST_IN_POSTS")
        .map(PathBuf::from)
        .map_err(|_| "Environment variable JUST_IN_POSTS not set")?;
    
    let pictures_dir = env::var("JUST_IN_PICTURES")
        .map(PathBuf::from)
        .map_err(|_| "Environment variable JUST_IN_PICTURES not set")?;

    // Ensure directories exist
    fs::create_dir_all(&posts_dir)?;
    fs::create_dir_all(&pictures_dir)?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new(posts_dir, pictures_dir)?;
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(result?)
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if app.handle_key_event(key.code)? {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("New Blog Post Generator")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content
    match app.state {
        AppState::TitleInput => render_text_input(f, chunks[1], app, "Title", "Enter your blog post title:"),
        AppState::ExcerptInput => render_text_input(f, chunks[1], app, "Excerpt", "Enter a brief excerpt for your post:"),
        AppState::ImageSelectionMenu => {
            render_image_menu(f, chunks[1], "Cover Image Selection", app.image_menu_selection, &app.image_files);
        }
        AppState::ImageSelection => {
            let files = &app.image_files;
            render_image_selection(f, chunks[1], "Cover Image", files, &mut app.image_list_state);
        }
        AppState::OgImageSelectionMenu => {
            render_image_menu(f, chunks[1], "OG Image Selection", app.og_image_menu_selection, &app.og_image_files);
        }
        AppState::OgImageSelection => {
            let files = &app.og_image_files;
            render_image_selection(f, chunks[1], "OG Image", files, &mut app.og_image_list_state);
        }
        AppState::Confirmation => render_confirmation(f, chunks[1], app),
        AppState::Error(ref msg) => render_error(f, chunks[1], msg),
    }

    // Help/Status bar
    let help_text = match app.state {
        AppState::TitleInput | AppState::ExcerptInput => {
            "Enter: Submit | Esc/q: Quit | F1: Help"
        }
        AppState::ImageSelectionMenu | AppState::OgImageSelectionMenu => {
            "↑↓/jk: Navigate | Enter: Select | Esc/q: Quit | F1: Help"
        }
        AppState::ImageSelection | AppState::OgImageSelection => {
            "↑↓/jk: Navigate | Enter: Select | s: Skip | Esc/q: Quit | F1: Help"
        }
        AppState::Confirmation => {
            "y/Enter: Create Post | n/Esc/q: Cancel | F1: Help"
        }
        AppState::Error(_) => {
            "Press any key to exit"
        }
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);

    if app.show_help {
        render_help_popup(f);
    }
}

fn render_text_input(f: &mut Frame, area: Rect, app: &App, field_name: &str, prompt: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .split(area);

    let prompt_block = Paragraph::new(prompt)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title(field_name));
    f.render_widget(prompt_block, chunks[0]);

    let input_text = if app.cursor_position <= app.input_buffer.len() {
        let mut display_text = app.input_buffer.clone();
        display_text.insert(app.cursor_position, '│');
        display_text
    } else {
        format!("{}│", app.input_buffer)
    };

    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
}

fn render_image_selection(f: &mut Frame, area: Rect, title: &str, files: &[PathBuf], list_state: &mut ListState) {
    let items: Vec<ListItem> = files
        .iter()
        .map(|file| {
            let filename = file.file_name().unwrap_or_default().to_string_lossy().to_string();
            ListItem::new(filename)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow))
        .highlight_symbol("► ");

    f.render_stateful_widget(list, area, list_state);
}

fn render_image_menu(f: &mut Frame, area: Rect, title: &str, selection: usize, existing_files: &[PathBuf]) {
    let file_count = existing_files.len();
    let menu_items = vec![
        format!("📁 Browse for new image..."),
        format!("🖼️  Select from existing ({} files)", file_count),
        format!("⏭️  Skip (use default)"),
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .map(|item| ListItem::new(item.as_str()))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow))
        .highlight_symbol("► ");

    let mut list_state = ListState::default();
    list_state.select(Some(selection));
    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_confirmation(f: &mut Frame, area: Rect, app: &App) {
    let text = vec![
        Line::from(vec![
            Span::styled("Title: ", Style::default().fg(Color::Yellow)),
            Span::raw(&app.blog_post.title),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Excerpt: ", Style::default().fg(Color::Yellow)),
            Span::raw(&app.blog_post.excerpt),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Cover Image: ", Style::default().fg(Color::Yellow)),
            Span::raw(&app.blog_post.cover_image),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("OG Image: ", Style::default().fg(Color::Yellow)),
            Span::raw(&app.blog_post.og_image),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("Create this blog post? (y/n): ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Confirmation"))
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

fn render_error(f: &mut Frame, area: Rect, message: &str) {
    let paragraph = Paragraph::new(message)
        .style(Style::default().fg(Color::Red))
        .block(Block::default().borders(Borders::ALL).title("Error"))
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

fn render_help_popup(f: &mut Frame) {
    let area = centered_rect(60, 70, f.area());
    f.render_widget(Clear, area);

    let help_text = vec![
        Line::from(vec![Span::styled("Help", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  ↑↓ or j/k  - Move up/down in lists"),
        Line::from("  Enter      - Select/Submit"),
        Line::from("  Esc/q      - Quit/Cancel"),
        Line::from("  s          - Skip image selection"),
        Line::from(""),
        Line::from("Text Input:"),
        Line::from("  Type       - Enter text"),
        Line::from("  ←→         - Move cursor"),
        Line::from("  Home/End   - Jump to start/end"),
        Line::from("  Backspace  - Delete character"),
        Line::from(""),
        Line::from("Environment Variables Required:"),
        Line::from("  JUST_IN_POSTS    - Posts directory"),
        Line::from("  JUST_IN_PICTURES - Images directory"),
        Line::from(""),
        Line::from("Press F1 to close help"),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn get_image_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if dir.exists() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg") {
                        files.push(path);
                    }
                }
            }
        }
    }
    
    files.sort();
    Ok(files)
}

fn create_blog_post(blog_post: &BlogPost, posts_dir: &Path) -> io::Result<PathBuf> {
    let now = Utc::now();
    let filename = format!("{}{}{}{}{}{}.md",
        now.year(),
        format!("{:02}", now.month()),
        format!("{:02}", now.day()),
        format!("{:02}", now.hour()),
        format!("{:02}", now.minute()),
        format!("{:02}", now.second()));

    let file_path = posts_dir.join(&filename);
    let formatted_now = now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    let content = format!(r#"---
title: '{}'
excerpt: '{}'
coverImage: '{}'
date: '{}'
author:
  name: 'Justin Bender'
  picture: '/assets/blog/authors/bender.png'
ogImage:
  url: '{}'
---

## {}

"#, 
        blog_post.title.replace("'", "\\'"),
        blog_post.excerpt.replace("'", "\\'"),
        blog_post.cover_image,
        formatted_now,
        blog_post.og_image,
        blog_post.title);

    let mut file = File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(file_path)
}

fn open_in_zed(file_path: &Path) -> io::Result<()> {
    #[cfg(windows)]
    let mut child = Command::new("cmd")
        .args(&["/C", "zed", file_path.to_str().unwrap()])
        .spawn()?;

    #[cfg(unix)]
    let mut child = Command::new("zed")
        .arg(file_path.to_str().unwrap())
        .spawn()?;

    let _ = child.wait();
    Ok(())
}

fn browse_for_image() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Select Image")
        .add_filter("Images", &["jpg", "jpeg", "png", "gif", "webp", "svg"])
        .pick_file()
}

fn copy_image_to_pictures(source: &Path, pictures_dir: &Path) -> io::Result<String> {
    let filename = source.file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid filename"))?
        .to_string_lossy();
    
    let dest_path = pictures_dir.join(filename.as_ref());
    
    // Copy the image to the pictures directory
    fs::copy(source, &dest_path)?;
    
    // Return the relative path for use in markdown
    Ok(format!("/assets/blog/img_bin/{}", filename))
}