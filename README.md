# pngme

Rust implementation of [PNGME](https://picklenerd.github.io/pngme_book/introduction.html)

A command line tool to encode/decode secret message in a standard png file.

## usage
`pngme encode ./dice.png ruSt "This is a secret message!`

`pngme decode ./dice.png ruSt`

`pngme remove ./dice.png ruSt`

`pngme print ./dice.png`

## dependencies
* `anyhow 1.0` and `thiserror 1.0` for error handling
* `crc 3.0` for CRC checksum
* `clap 3.1`(derive API) for parsing CLI arguments
