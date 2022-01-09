use std::io::{self, BufRead};
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::{Widget,Block,Borders};
use tui::layout::{Layout, Constraint, Direction,Rect};
use std::time::{SystemTime,Duration};
use std::thread::sleep;

fn main()  -> Result<(),io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let start_time = SystemTime::now();

    terminal.clear()?;

    loop{
        let elapsed_string = start_time
            .elapsed()
            .unwrap()
            .as_secs()
            .to_string();

        terminal.draw(|f|{
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10)
                    ].as_ref()
                )
                .split(f.size());

            let block = Block::default()
                .title(elapsed_string)
                .borders(Borders::ALL);

            f.render_widget(block, chunks[1]);

            sleep(Duration::new(1,0));
        });
    }
}
