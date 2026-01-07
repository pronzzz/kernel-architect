mod state;
mod tui;
mod net;
mod pcg;
mod ai;
mod gameplay;

use std::error::Error;



use std::time::Duration;
use state::App;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, widgets::{Block, Borders}};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--server" {
        net::ssh_server::run_server(2026).await?;
        return Ok(());
    }

    println!("Welcome to The Kernel Architect (Local Mode).");
    let mut terminal = tui::init()?;
    
    // Create Application state
    let mut app = App::new();

    // Main loop
    loop {
        terminal.draw(|f| render_ui(f, &app))?;

        // Handle events
        if event::poll(Duration::from_millis(app.tick_rate))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Left => app.move_player(-1, 0),
                    KeyCode::Right => app.move_player(1, 0),
                    KeyCode::Up => app.move_player(0, -1),
                    KeyCode::Down => app.move_player(0, 1),
                    KeyCode::Enter => {
                        if let Some(msg) = app.hack_current_node() {
                            app.log(msg);
                        }
                    }
                    _ => {}
                }
            }
        }
        if app.should_quit {
            break;
        }

        app.on_tick();
    }

    // Restore terminal
    tui::restore()?;
    Ok(())
}

mod ui;

use ratatui::layout::{Constraint, Direction, Layout};

fn render_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header/Status
            Constraint::Min(0),    // Main Content
            Constraint::Length(10), // Console Log
        ])
        .split(f.size());

    // Status Bar
    let status_block = Block::default().borders(Borders::ALL).title("Status");
    f.render_widget(status_block, chunks[0]);
    let status = ui::components::StatusWidget { hp: 100, turn: 1 }; // Pull from app state
    f.render_widget(status, chunks[0].inner(&ratatui::layout::Margin { vertical: 1, horizontal: 1 }));

    // Main Area (Map)
    let map_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[1]);
    
    f.render_widget(ui::components::MapWidget { 
        _active_tick: app.animation_tick,
        player_x: app.player_x,
        player_y: app.player_y,
        cluster: &app.cluster,
    }, map_chunks[0]);
    
    f.render_widget(ui::components::InfoWidget { info: &app.get_current_node_info() }, map_chunks[1]);

    // Console
    let console = ui::components::ConsoleWidget { logs: &app.logs };
    f.render_widget(console, chunks[2]);
}
