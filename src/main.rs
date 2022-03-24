fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://google.com/")?;
    let date_string = resp.headers().get("date").unwrap();
    let date = chrono::DateTime::parse_from_rfc2822(date_string.to_str()?)?;
    let date = date.with_timezone(&chrono_tz::Europe::Berlin);

    // let connection = sqlite::open("./db")?;

    let connection = sqlite::open("./db")?;
    connection.execute("create table clock (time text);")?;
    let mut stmt = connection.prepare("insert into clock(time) values(?)")?;
    let datestr = date.to_rfc3339();
    stmt.bind(1, datestr.as_str())?;
    stmt.next()?;

    println!("Success: {}", date);

    Ok(())
}

// v-im:tw=4:sw=4:et
// vim:sw=4:et:ts=4
