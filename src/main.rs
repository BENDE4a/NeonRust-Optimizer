mod cleaner;
mod localization;
mod processes;
mod ram_cleaner;
mod rust_tweaks;

#[derive(Clone, Copy)]
pub enum Language {
    English,
    Ukrainian,
    Russian,
}

impl Language {
    pub fn next(&self) -> Self {
        match self {
            Self::English => Self::Ukrainian,
            Self::Ukrainian => Self::Russian,
            Self::Russian => Self::English,
        }
    }
}

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use localization::Localization;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::{error::Error, io, time::{Duration, Instant}};

struct App {
    language: Language,
    loc: Localization,
    should_quit: bool,
    state: ListState,
    items: Vec<String>,
    logs: Vec<String>,
    tick_count: usize,
}

impl App {
    fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            language: Language::Russian, // Default to Russian as requested by user's prompt
            loc: Localization::new(),
            should_quit: false,
            state,
            items: vec![
                "item_clean_pc".to_string(),
                "item_kill_bloat".to_string(),
                "item_opt_rust".to_string(),
                "item_prio_rust".to_string(),
                "item_clean_ram".to_string(),
                "item_lossless_scaling".to_string(),
                "item_network_opt".to_string(),
                "item_power_plan".to_string(),
                "item_disable_gamedvr".to_string(),
                "item_timer_resolution".to_string(),
                "item_quit".to_string(),
            ],
            logs: vec!["welcome".to_string()],
            tick_count: 0,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn execute_selected(&mut self) {
        if let Some(i) = self.state.selected() {
            match i {
                0 => {
                    let (count, size) = cleaner::clean_temp_and_prefetch();
                    self.logs.insert(0, format!("{} {} {} {} {}", self.loc.get(&self.language, "deleted"), count, self.loc.get(&self.language, "files"), size, self.loc.get(&self.language, "bytes")));
                }
                1 => {
                    let (killed, _msg) = processes::kill_bloatware();
                    self.logs.insert(0, format!("{} {}", self.loc.get(&self.language, "killed_processes"), killed));
                }
                2 => {
                    match rust_tweaks::optimize_client_cfg() {
                        Ok(msg) => self.logs.insert(0, msg),
                        Err(e) => self.logs.insert(0, format!("Ошибка: {}", e)),
                    }
                }
                3 => {
                    match rust_tweaks::prioritize_rust_client() {
                        Ok(msg) => self.logs.insert(0, msg),
                        Err(e) => self.logs.insert(0, format!("Ошибка: {}", e)),
                    }
                }
                4 => {
                    match ram_cleaner::purge_standby_list() {
                        Ok(msg) => self.logs.insert(0, msg),
                        Err(e) => self.logs.insert(0, format!("Ошибка: {}", e)),
                    }
                }
                5 => {
                    match rust_tweaks::apply_lossless_scaling() {
                        Ok(msg) => self.logs.insert(0, msg),
                        Err(e) => self.logs.insert(0, format!("Ошибка: {}", e)),
                    }
                }
                6 => {
                    match rust_tweaks::optimize_network() {
                        Ok(msg) => self.logs.insert(0, msg),
                        Err(e) => self.logs.insert(0, format!("Ошибка: {}", e)),
                    }
                }
                7 => {
                    match rust_tweaks::set_ultimate_power_plan() {
                        Ok(msg) => self.logs.insert(0, msg),
                        Err(e) => self.logs.insert(0, format!("Ошибка: {}", e)),
                    }
                }
                8 => {
                    match rust_tweaks::disable_game_dvr() {
                        Ok(msg) => self.logs.insert(0, msg),
                        Err(e) => self.logs.insert(0, format!("Ошибка: {}", e)),
                    }
                }
                9 => {
                    match rust_tweaks::set_timer_resolution() {
                        Ok(msg) => self.logs.insert(0, msg),
                        Err(e) => self.logs.insert(0, format!("Ошибка: {}", e)),
                    }
                }
                10 => {
                    self.should_quit = true;
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();
    let mut last_scroll = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Down | KeyCode::Char('j') => {
                            if last_scroll.elapsed() >= Duration::from_millis(150) {
                                app.next();
                                last_scroll = Instant::now();
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if last_scroll.elapsed() >= Duration::from_millis(150) {
                                app.previous();
                                last_scroll = Instant::now();
                            }
                        }
                        KeyCode::Enter | KeyCode::Char(' ') => app.execute_selected(),
                        KeyCode::Char('l') => app.language = app.language.next(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick_count = app.tick_count.wrapping_add(1);
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(16), // ASCII Art / Title
                Constraint::Min(8),     // Menu
                Constraint::Length(8),  // Log
                Constraint::Length(1),  // Watermark
            ]
            .as_ref(),
        )
        .split(f.size());

    let ascii_art_lines = vec![
        "***************************",
        "***************************",
        "***********#%%%%#**********",
        "************%%%%***********",
        "************%%%%***********",
        "************#%%%***********",
        "****%%%%%###***************",
        "****%%%%%#%%*#%*#**********",
        "****###*******#%%###*******",
        "****************#%%%%#*****",
        "*****************#%%%%*****",
        "******************##*******",
        "***************************",
        "***************************",
    ];
    
    let mut spans = vec![];
    for (y, line) in ascii_art_lines.iter().enumerate() {
        let mut line_spans = vec![];
        for (x, ch) in line.chars().enumerate() {
            let distance = (x as f32) * 0.3 + (y as f32) * 0.5;
            let offset = (app.tick_count as f32) * 0.3;
            let wave = ((distance - offset).sin() + 1.0) / 2.0;
            
            let r = (90.0 + wave * 165.0) as u8;
            let g = (10.0 + wave * 40.0) as u8;
            let b = (150.0 + wave * 105.0) as u8;
            
            let char_str = if ch == '*' { " ".to_string() } else { ch.to_string() };
            line_spans.push(Span::styled(char_str, Style::default().fg(Color::Rgb(r, g, b)).add_modifier(Modifier::BOLD)));
        }
        spans.push(Line::from(line_spans));
    }

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Rgb(180, 0, 255)));
    let title_paragraph = Paragraph::new(spans)
        .block(title_block)
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title_paragraph, chunks[0]);

    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = Some(i) == app.state.selected();
            let prefix = if is_selected { "▶  " } else { "   " };
            let icon = match item.as_str() {
                "item_clean_pc" => "🧹",
                "item_kill_bloat" => "💀",
                "item_opt_rust" => "⚙️ ",
                "item_prio_rust" => "🔥",
                "item_clean_ram" => "🧠",
                "item_lossless_scaling" => "🔲",
                "item_network_opt" => "🌐",
                "item_power_plan" => "⚡",
                "item_disable_gamedvr" => "🚫",
                "item_timer_resolution" => "⏱️ ",
                "item_quit" => "🚪",
                _ => "▪️ ",
            };
            let localized_item = app.loc.get(&app.language, item);
            
            let prefix_color = if is_selected { Color::Rgb(255, 255, 100) } else { Color::Rgb(150, 80, 200) };
            let line = Line::from(vec![
                Span::styled(prefix, Style::default().fg(prefix_color).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{} ", icon), Style::default().fg(Color::Reset)),
                Span::raw(format!("{}", localized_item)),
            ]);

            let bg_color = if is_selected {
                let pulse = ((app.tick_count as f32 * 0.15).sin() * 25.0 + 45.0) as u8;
                Color::Rgb(pulse, 0, pulse + 50)
            } else { 
                Color::Reset 
            };
            let fg_color = if is_selected { Color::White } else { Color::Rgb(220, 180, 255) };
            
            ListItem::new(line).style(Style::default().fg(fg_color).bg(bg_color))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default()
               .borders(Borders::ALL)
               .border_type(BorderType::Rounded)
               .border_style(Style::default().fg(Color::Rgb(140, 0, 200)))
               .title(app.loc.get(&app.language, "menu_title")))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Rgb(255, 255, 255)).bg(Color::Rgb(60, 0, 120)));
    f.render_stateful_widget(list, chunks[1], &mut app.state);
    
    let logs_items: Vec<ListItem> = app.logs.iter().take(6).map(|l| ListItem::new(Line::from(
        if l == "welcome" { app.loc.get(&app.language, "welcome").to_string() } else { l.clone() }
    ))).collect();
    let logs_list = List::new(logs_items)
        .block(Block::default()
               .borders(Borders::ALL)
               .border_type(BorderType::Rounded)
               .border_style(Style::default().fg(Color::Rgb(100, 0, 150)))
               .title(app.loc.get(&app.language, "logs_title")))
        .style(Style::default().fg(Color::Rgb(200, 100, 255)));
    f.render_widget(logs_list, chunks[2]);

    let watermark = Paragraph::new(Span::styled("Киевский BOB", Style::default().fg(Color::Rgb(150, 50, 255)).add_modifier(Modifier::BOLD | Modifier::ITALIC)))
        .alignment(ratatui::layout::Alignment::Right);
    f.render_widget(watermark, chunks[3]);
}
