use chrono::DateTime;
use chrono::SecondsFormat;
use chrono::TimeZone;
use chrono::Utc;

fn from_millis(millis: i64) -> DateTime<Utc> {
    Utc.timestamp_millis(millis)
}

fn candle_id(timestamp: &DateTime<Utc>, size_ms: i64) -> DateTime<Utc> {
    let millis = timestamp.timestamp_millis();
    println!(
        "{} -> {}",
        timestamp.to_rfc3339_opts(SecondsFormat::Millis, false),
        millis
    );
    let id = (millis / size_ms) * size_ms;
    println!("                                 {}", id);
    Utc.timestamp_millis(id)
}

fn display(name: &str, millis: i64) {
    let today = from_millis(millis);
    println!(
        "{:>10}: {}",
        name,
        today.to_rfc3339_opts(SecondsFormat::Millis, false)
    );
    println!(
        "{:>10}: {}",
        "ID",
        candle_id(&today, 10).to_rfc3339_opts(SecondsFormat::Millis, false)
    );
}

fn main() {
    let complete = (1648641824010i64 / 10) * 10;
    println!("Complete: {}", complete);

    display("0", 1648641824000);
    display("1", 1648641824001);
    display("9", 1648641824009);
    display("10", 1648641824010);
    display("99", 1648641824099);
    display("100", 1648641824100);
}
