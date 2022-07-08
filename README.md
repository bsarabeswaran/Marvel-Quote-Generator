# Marvel-Quote-Generator

### About the Project
A small project that includes web scraping for all marvel projects using an IMDB list, and then generating quotes using Markov Chains inspired by https://rosettacode.org/wiki/Markov_chain_text_generator#Rust.

To run:

```
$ cd generate_quote/
$ cargo build
$ cd ..
$ ./generate_quote/target/release/generate_quote all_quotes.txt 2 20
```

If you would like to fetch new quotes, simply run:

```
$ python3 collect_quotes.py
```