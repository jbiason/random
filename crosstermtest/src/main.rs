use crossterm::{
    execute,
    terminal::{size, ScrollUp, SetSize, ScrollDown},
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let (cols, rows) = size()?;
    println!("Cols: {cols}, rows: {rows}");

    // Resize terminal and scroll up.
    execute!(io::stdout(), SetSize(10, 10), ScrollUp(5))?;
    for i in 0..11 {
        println!("Line {i}");
    }

    execute!(io::stdout(), SetSize(10, 10), ScrollDown(5))?;

    // Be a good citizen, cleanup
    execute!(io::stdout(), SetSize(cols, rows))?;
    Ok(())
}
