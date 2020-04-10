use clap::{App, Arg};
use dotenv;
use std::env;
use std::path::PathBuf;

mod currency;
mod parse;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let my_path = env::var_os("HOME")
        .map(PathBuf::from)
        .and_then(|a| Some(a.join(".poilenv")))
        .unwrap();
    if let Err(err) = dotenv::from_path(my_path.as_path()) {
        eprintln!("Cannot find the .poilenv file with the API key: {}", err);
        std::process::exit(-1);
    }
    // Define command line arguments.
    let matches = App::new("poile")
        .version("0.1.0")
        .author("versbinarii@gmail.com")
        .about("Show current exchange rates")
        .arg(
            Arg::with_name("from")
                .takes_value(true)
                .index(1)
                .help("Currency to check. Default EUR"),
        )
        .arg(
            Arg::with_name("qty")
                .takes_value(true)
                .index(2)
                .help("Quantitiy to exchange"),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .long("to")
                .takes_value(true)
                .help("Your country currency. Default PLN"),
        )
        .arg(
            Arg::with_name("date")
                .short("d")
                .long("date")
                .takes_value(true)
                .help("Currency value at given date in YYYY/MM/DD format"),
        )
        .get_matches();

    let from: &str = matches.value_of("from").unwrap_or("EUR");
    let to = matches.value_of("to").unwrap_or("PLN");
    let qty = matches.value_of("qty").unwrap_or("1");
    let timestamp = matches.value_of("date");
    let currency_query = currency::CurrencyQuery::new(from, to, timestamp);

    let url = currency_query.prepare_query_string();
    let json_response = parse::do_get(&url).await?;
    json_response.display(qty.parse().unwrap());
    Ok(())
}
