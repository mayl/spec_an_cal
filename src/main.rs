use std::io::{self, prelude::*};
use std::net::TcpStream;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::{Spans, Span};

fn main() -> std::io::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
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
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block,chunks[0]);
        let block = Block::default()
            .title("Block 2")
            .borders(Borders::ALL);
        f.render_widget(block,chunks[1]);

        let block = Block::default()
            .title("Block 3")
            .borders(Borders::ALL);
        let text = vec![
            Spans::from("This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text This is some test text "),
        ];
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap {trim: true});
        f.render_widget(paragraph, chunks[2]);
        //f.render_widget(block,chunks[2]);
    })?;

    let mut stream = TcpStream::connect("localhost:8080")?;
    stream.write_all(b"Hello, World!")?; 
    let mut buf = [0; 10];
    let _n = stream.read(&mut buf[..]);
    let rx_val = String::from_utf8_lossy(&buf);
    println!("Received the bytes: {:?}", rx_val);
    Ok(())
}
