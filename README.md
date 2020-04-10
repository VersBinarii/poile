# poile
Simple currency exchange CLI tool using [ExchangeRate-API][1] for current rates

## Config
It looks for a `.poilenv` file in your `$HOME` directory.
The file should contain `CURR_EXCHANGE_API_KEY` env variable with the API key.

## Installation
```
git clone https://github.com/VersBinarii/poile.git
cd poile/
cargo install --path .
```

## Example
```
poile --help
poile 0.1.0
Show current exchange rates

USAGE:
    poile [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --date <date>    Currency value at given date in YYYY/MM/DD format
    -t, --to <to>        Your country currency. Default PLN

ARGS:
    <from>    Currency to check. Default EUR
    <qty>     Quantitiy to exchange
```

Convert 128 Dollars to Euro:
```
poile USD 128 -t EUR 
```

Convert 128 Euro to Polish Zloty
```
poile USD 128
```

[1]: https://www.exchangerate-api.com/
