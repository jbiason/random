use console::Style;
use console::Term;

fn main() {
    let term = Term::stdout();
    let (_rows, columns) = term.size();
    let success = Style::new().green().underlined();
    let error = Style::new().red().underlined();
    println!("Total width: {}", columns);

    println!(
        "{value:>width$}",
        value = success.apply_to("Some value"),
        width = columns as usize
    );
    println!(
        "{value:>width$}",
        value = error.apply_to("Invalid"),
        width = (columns / 2) as usize
    )
}
