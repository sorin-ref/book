use std::io::stdout;
use std::error::Error;
use std::time::Duration;

const MAX_PAGE: u32 = 10;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, size, Clear, ClearType},
    cursor::{MoveTo, Hide, Show},
    style::{Print, Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    event::{poll, read, Event, KeyCode},
};

fn print_page_0(_width: u16, _height: u16) -> Result<(), Box<dyn Error>> {
    execute!(
        stdout(),
        Print("Hello"),
    )?;
    Ok(())
}

fn print_page_1(_width: u16, _height: u16) -> Result<(), Box<dyn Error>> {
    execute!(
        stdout(),
        MoveTo(1, 1),
        SetForegroundColor(Color::Green),
        Print("world!"),
    )?;
    Ok(())
}

fn print(page: u32, width: u16, height: u16) -> Result<(), Box<dyn Error>> {
    match page {
        0 => print_page_0(width, height)?,
        1 => print_page_1(width, height)?,
        _ => ()
    }
    Ok(())
}

fn layout_print(page: u32, width: u16, height: u16) -> Result<(), Box<dyn Error>> {
    let size_indicator = format!("{}/{}", width, height);
    execute!(
        stdout(),
        MoveTo(page as u16, 0),
        Print("Page "),
        SetBackgroundColor(Color::Blue),
        SetForegroundColor(Color::Yellow),
        Print(format!("{}", page)),
        ResetColor,
        MoveTo(width - size_indicator.len() as u16, height - 1),
        Print(size_indicator),
        MoveTo(0, 1),
    )?;
    print(page, width, height)?;
    Ok(())
}

fn prepare_print(page: u32, width: u16, height: u16) -> Result<(), Box<dyn Error>> {
    execute!(
        stdout(),
        Clear(ClearType::All),
        ResetColor,
        MoveTo(0, 0),
    )?;
    layout_print(page, width, height)?;
    execute!(
        stdout(),
        ResetColor,
        MoveTo(0, 0),
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut page: u32 = 0;

    enable_raw_mode()?;
    
    let (mut width, mut height) = size()?;

    execute!(
        stdout(),
        Hide,
    )?;

    prepare_print(page, width, height)?;

    loop {
        if poll(Duration::from_millis(200))? {
            match read()? {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            execute!(
                                stdout(),
                                Clear(ClearType::All),
                                Show,
                            )?;
                            disable_raw_mode()?;
                            std::process::exit(0);
                        }
                        KeyCode::Left | KeyCode::Up | KeyCode::Char('p') => {
                            if page > 0 {
                                page -= 1
                            }
                        }
                        KeyCode::Right | KeyCode::Down | KeyCode::Char('n') => {
                            if page < MAX_PAGE {
                                page += 1;
                            }
                        }
                        _ => ()
                    }
                }
                Event::Resize(w, h) => {
                    width = w;
                    height = h;
                }
                _ => ()
            }

            prepare_print(page, width, height)?;
        }
    }
}
