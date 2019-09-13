# exchage

A commandline tool converting currencies.


## Installation

    $ cargo install exchage


## Usage

Obtain an API key from <https://www.currencyconverterapi.com/> and `export` it
to the `API_KEY` environment variable.


```
$  export API_KEY=...

$ exchage 1 AUD_USD
Given Amount: "1"
Base Amount: $1.0
Rate: 0.686816
Target Amount: $0.686816
```
