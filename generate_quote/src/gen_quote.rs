use actix_web::web::Json;
use actix_web::{HttpResponse, get};
use serde::{Deserialize, Serialize};
use unidecode::unidecode;

use std::env;
use std::fs;
use std::collections;
use std::cmp;
use rand::Rng;

#[derive(Debug, Deserialize)]
pub struct Request {
    length: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    quote: String
}

#[get("/quote")]
pub async fn gen_quote_endpoint(length: Json<Request>) -> HttpResponse{
    let quote = gen_quote(("all_quotes.txt").to_string(), 2, length.into_inner().length.parse::<usize>().unwrap());
    HttpResponse::Ok()
        .content_type("application/json")
        .json(Response {quote: quote})
}

fn gen_quote(args_file: String, args_context: usize, args_quote: usize) -> String{
    //let (args_file, args_context, args_quote) = collect_args();
    let mut words: String = fs::read_to_string(args_file).expect("File error");
    let (speakermap, quotemap) = create_hashmap(&mut words, args_context);
    let quote = generate_str(speakermap, quotemap, args_quote);
    quote
    // inspired by https://rosettacode.org/wiki/Markov_chain_text_generator#Rust
}

fn create_hashmap(s: &mut str, context_len: usize) -> (collections::HashMap<String, Vec<String>>, collections::HashMap<String, Vec<String>>) {
    if context_len == 0 {
        panic!("context_len cannot be less than one")
    }

    let lines = s.split("\n");
    let mut speaker: collections::HashMap<String, Vec<String>> = collections::HashMap::new();
    let mut hashmap: collections::HashMap<String, Vec<String>> = collections::HashMap::new();
    for line in lines {
        let line_converted = unidecode(line);
        let mut words: Vec<&str> = line_converted.trim().split(" ").collect();
        if words[words.len() - 1] == "" {
            words.pop();
        }
        
        let mut num_prefixes: usize = 0;
        if words.len() >= context_len {
            num_prefixes = words.len() - context_len;
        }

        let mut quote_header_end = 0;
        for i in 0..words.len() {
            if words[i].contains(":") {
                quote_header_end = i + 1;
                break;
            }
        }

        let quote_starter = &words[0..quote_header_end].join(" ").to_string();
        let mut comb_suff = String::from("");
        for i in quote_header_end..cmp::min(quote_header_end + context_len, words.len()){
            comb_suff.push_str(words[i]);
            comb_suff.push_str(" ");
        }
        if !comb_suff.is_empty() {
            comb_suff.pop();
        }
        match speaker.get_mut(quote_starter){
            Some(suffixes) => {
                if !comb_suff.is_empty() {
                    suffixes.push(comb_suff.replace(&[',', '\"', ';', '!', '?', '*', '.', ':'][..], ""));
                };
            },
            None => {
                if !comb_suff.is_empty() {
                    speaker.insert(quote_starter.to_string(), vec![comb_suff.replace(&[',', '\"', ';', '!', '?', '*', '.', ':'][..], "")]);
                };
            }
        };


        for i in quote_header_end..num_prefixes {
            let mut word_comb = words[i].to_string();
            
            let words_iter = words.iter().take(i + context_len).skip(i + 1);
            for word in words_iter {
                word_comb.push_str(" ");
                word_comb.push_str(word);
            }
            
            match hashmap.get_mut(&word_comb) {
                Some(suffixes) => {suffixes.push(String::from(words[i + context_len].replace(&[',', '\"', ';', '!', '?', '*', '.', ':'][..], "")));},
                None => {hashmap.insert(word_comb, vec![String::from(words[i + context_len].replace(&[',', '\"', ';', '!', '?', '*', '.', ':'][..], ""))]);}
            };
        }
    }
    (speaker, hashmap)

}

fn generate_str(speakermap: collections::HashMap<String, Vec<String>>, quotemap: collections::HashMap<String, Vec<String>>, quote_len: usize) -> String{
    let mut quote = String::from("");
    // get random prefix
    let speaker_len = speakermap.len();
    let mut rng = rand::thread_rng();
    let mut rand_prefix = speakermap.keys().collect::<Vec<&String>>()[rng.gen_range(0..speaker_len)].to_string();
    quote.push_str(&rand_prefix);
    let mut len_pref_vec = speakermap.get(&rand_prefix).unwrap().len();
    let mut rand_suffix = &speakermap.get(&rand_prefix).unwrap()[rng.gen_range(0..len_pref_vec)];
    quote.push_str(" ");
    quote.push_str(rand_suffix);
    rand_prefix = rand_suffix.to_string();

    let mut i = 1;
    while i < quote_len || ![".", "?", "!"].contains(&&quote[quote.len() - 1..]) {
    //for _ in 1..quote_len {
        len_pref_vec = match quotemap.get(&rand_prefix) {
            Some(suffixes) => suffixes.len(),
            None => {
                if i >= quote_len {
                    quote.push_str(".");
                    break;
                }
                quote.push_str(". ");
                let num_quotes = quotemap.len();
                rand_prefix = quotemap.keys().collect::<Vec<&String>>()[rng.gen_range(0..num_quotes)].to_string();
                quote.push_str(&rand_prefix);
                quotemap.get(&rand_prefix).unwrap().len()
            }
        };
        
        rand_suffix = &quotemap.get(&rand_prefix).unwrap()[rng.gen_range(0..len_pref_vec)];
        quote.push_str(" ");
        quote.push_str(rand_suffix);
        let remain_pref_words = rand_prefix.split(" ").collect::<Vec<&str>>();
        let mut remain_prefix = String::from("");
        for wrd in &remain_pref_words[1..] {
            remain_prefix.push_str(wrd);
            remain_prefix.push_str(" ");
        }
        remain_prefix.push_str(rand_suffix);
        rand_prefix = remain_prefix.to_string();
        i+=1;
    }

    quote
}

fn _collect_args() -> (String, usize, usize) {
    let args: Vec<String> = env::args().collect();
    let file = match args.get(1) {
        Some(val) => val,
        None => panic!("No File specified")
    };
    let context_len = match args.get(2) {
        Some(val) => val.parse::<usize>().expect("Context Length must be usize"),
        None => panic!("No context length specified")
    };
    let quote_len = match args.get(3) {
        Some(val) => val.parse::<usize>().expect("Quote Length must be usize"),
        None => panic!("No quote length specified")
    };
    (String::from(file), context_len, quote_len)
}
