// Playable roguelike game

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use mandelrust::roguelike::{GameState, Position};
use rand::thread_rng;
use std::io::{self, stdout, Write};

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    let mut rng = thread_rng();

    // Setup terminal
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    // Create game
    let mut game = GameState::new(&mut rng);

    // Game loop
    let result = game_loop(&mut game, &mut stdout);

    // Cleanup terminal
    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
        ResetColor
    )?;
    terminal::disable_raw_mode()?;

    result
}

fn game_loop<W: Write>(game: &mut GameState, stdout: &mut W) -> io::Result<()> {
    loop {
        // Draw the game
        draw_game(game, stdout)?;

        // Check game over
        if game.is_game_over() {
            queue!(
                stdout,
                cursor::MoveTo(0, 20),
                SetForegroundColor(Color::Red),
                Print("GAME OVER! You have died."),
                ResetColor,
                cursor::MoveTo(0, 21),
                Print("Press 'q' to quit."),
            )?;
            stdout.flush()?;

            // Wait for quit
            loop {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
            break;
        }

        // Handle input
        if let Event::Key(key) = event::read()? {
            let mut moved = false;

            match key.code {
                // Movement with arrow keys or numpad
                KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('8') => {
                    game.try_move_player(&Position::new(0, -1));
                    moved = true;
                }
                KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('2') => {
                    game.try_move_player(&Position::new(0, 1));
                    moved = true;
                }
                KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('4') => {
                    game.try_move_player(&Position::new(-1, 0));
                    moved = true;
                }
                KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('6') => {
                    game.try_move_player(&Position::new(1, 0));
                    moved = true;
                }
                // Diagonal movement
                KeyCode::Char('y') | KeyCode::Char('7') => {
                    game.try_move_player(&Position::new(-1, -1));
                    moved = true;
                }
                KeyCode::Char('u') | KeyCode::Char('9') => {
                    game.try_move_player(&Position::new(1, -1));
                    moved = true;
                }
                KeyCode::Char('b') | KeyCode::Char('1') => {
                    game.try_move_player(&Position::new(-1, 1));
                    moved = true;
                }
                KeyCode::Char('n') | KeyCode::Char('3') => {
                    game.try_move_player(&Position::new(1, 1));
                    moved = true;
                }
                // Wait/skip turn
                KeyCode::Char('.') | KeyCode::Char('5') => {
                    moved = true;
                }
                // Quit
                KeyCode::Char('q') | KeyCode::Esc => {
                    break;
                }
                _ => {}
            }

            // Enemy turns after player moves
            if moved {
                game.enemy_turns();
            }
        }
    }

    Ok(())
}

fn draw_game<W: Write>(game: &GameState, stdout: &mut W) -> io::Result<()> {
    queue!(stdout, terminal::Clear(ClearType::All))?;

    // Draw title
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        SetForegroundColor(Color::Yellow),
        Print("=== DUNGEON OF THE FALLEN KING ==="),
        ResetColor,
    )?;

    // Draw map
    let view_width = 60;
    let view_height = 35;
    let start_x = (game.player.pos.x - view_width / 2).max(0);
    let start_y = (game.player.pos.y - view_height / 2).max(0);

    for y in 0..view_height {
        for x in 0..view_width {
            let world_x = start_x + x;
            let world_y = start_y + y;
            let pos = Position::new(world_x, world_y);

            queue!(stdout, cursor::MoveTo(x as u16, (y + 2) as u16))?;

            // Check if player is at this position
            if pos == game.player.pos {
                queue!(
                    stdout,
                    SetForegroundColor(Color::White),
                    Print(game.player.glyph),
                    ResetColor
                )?;
                continue;
            }

            // Check if enemy is at this position
            if let Some(enemy) = game.enemies.iter().find(|e| e.pos == pos && e.is_alive()) {
                let color = match enemy.glyph {
                    'g' => Color::Green,
                    'o' => Color::DarkYellow,
                    'T' => Color::DarkRed,
                    _ => Color::Red,
                };
                queue!(stdout, SetForegroundColor(color), Print(enemy.glyph), ResetColor)?;
                continue;
            }

            // Draw tile
            if let Some(tile) = game.map.get_tile(&pos) {
                let ch = tile.to_char();
                let color = match ch {
                    '#' => Color::DarkGrey,
                    '.' => Color::Grey,
                    '>' => Color::Cyan,
                    _ => Color::White,
                };
                queue!(stdout, SetForegroundColor(color), Print(ch), ResetColor)?;
            }
        }
    }

    // Draw UI
    let ui_y = view_height + 3;
    queue!(
        stdout,
        cursor::MoveTo(0, ui_y as u16),
        Print("─".repeat(60)),
        cursor::MoveTo(0, (ui_y + 1) as u16),
        SetForegroundColor(Color::White),
        Print(format!(
            "HP: {}/{} | Level: {} | Enemies: {}",
            game.player.hp,
            game.player.max_hp,
            game.dungeon_level,
            game.enemies.len()
        )),
        ResetColor,
    )?;

    // Draw messages (last 3)
    let msg_start = (ui_y + 2) as u16;
    for (i, msg) in game.messages.iter().rev().take(3).enumerate() {
        queue!(
            stdout,
            cursor::MoveTo(0, msg_start + i as u16),
            Print(msg),
        )?;
    }

    // Draw controls
    queue!(
        stdout,
        cursor::MoveTo(0, (ui_y + 6) as u16),
        SetForegroundColor(Color::DarkGrey),
        Print("Controls: Arrow keys/hjkl/numpad to move | q to quit"),
        ResetColor,
    )?;

    stdout.flush()?;
    Ok(())
}
