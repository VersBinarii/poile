extern crate serde_json;

use self::serde_json::Value;
use std::io::{self, Write};

enum QueryType {
    Pair(String),
    Historical(String),
}

pub struct CurrencyQuery<'a> {
    from: &'a str,
    to: &'a str,
    timestamp: Option<&'a str>,
    query_type: QueryType,
}

impl<'a> CurrencyQuery<'a> {
    pub fn new(from: &'a str, to: &'a str, timestamp: Option<&'a str>) -> Self {
        let query_type = match timestamp {
            Some(_) => QueryType::Historical("archive".to_owned()),
            None => QueryType::Pair("pair".to_owned()),
        };

        CurrencyQuery {
            from,
            to,
            timestamp,
            query_type,
        }
    }

    pub fn prepare_query_string(&self) -> String {
        let api_key = dotenv::var("CURR_EXCHANGE_API_KEY").unwrap();
        let url = match self.query_type {
            QueryType::Pair(ref s) => format!(
                "https://v3.exchangerate-api.com/{}/{}/{}/{}",
                s, api_key, self.from, self.to
            ),
            QueryType::Historical(ref s) => format!(
                "https://v3.exchangerate-api.com/{}/{}/{}/{}",
                s,
                api_key,
                self.to,
                self.timestamp.unwrap()
            ),
        };
        url
    }
}

pub struct CurrencyResponse(pub String);

impl CurrencyResponse {
    pub fn display(&self, qty: u32) {
        let v: Value = serde_json::from_slice(self.0.as_bytes()).unwrap();
        if qty == 1 {
            io::stdout()
                .write_fmt(format_args!(
                    "{} costs {} {}",
                    v["from"], v["rate"], v["to"]
                ))
                .unwrap();
        } else {
            let rate = v["rate"].as_f64().unwrap();
            io::stdout()
                .write_fmt(format_args!(
                    "{} {} costs {} {}",
                    qty,
                    v["from"],
                    rate * qty as f64,
                    v["to"]
                ))
                .unwrap();
        }
    }
}
