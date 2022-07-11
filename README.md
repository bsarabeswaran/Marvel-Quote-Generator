# Marvel-Quote-Generator

### About the Project
A small project that includes web scraping for all marvel projects using an IMDB list, and then generating quotes using Markov Chains inspired by https://rosettacode.org/wiki/Markov_chain_text_generator#Rust.

To run the API:

```
$ cd generate_quote/
$ cargo build
$ cd ..
$ ./generate_quote/target/release/generate_quote
```

You can then query against the API as such:

```
$ http GET http://127.0.0.1:5000/quote length=<enter desired quote length as a number here>
```

If you would like to fetch new quotes from IMDB, simply run:

```
$ python3 collect_quotes.py
```