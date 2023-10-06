extern crate itertools;
use itertools::Itertools;
use std::collections::HashMap;
use lambda_http::{service_fn, Request, Response, IntoResponse, Body, Error, RequestExt};
use serde::Serialize;
use http::StatusCode;

struct ApiGatewayResponse {
    status_code: u16,
    body: CustomOutput
}

impl ApiGatewayResponse {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::from_u16(self.status_code).unwrap())
            .body(Body::Text(serde_json::to_string(&self.body).unwrap()))
            .unwrap()
    }
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
    results: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(func)).await?;
    Ok(())
}

async fn func(request: Request) -> Result<impl IntoResponse, Error> {
    let qsp = request
        .query_string_parameters();
    let word = qsp.first("word").unwrap_or("");
    
    
    if word == "" || word.chars().count() > 128 {
        let err_resp = ApiGatewayResponse {
            status_code: 400,
            body: CustomOutput {
                message: String::from("failure"),
                results: Vec::new()
            }
        };
        
        return Ok(err_resp.into_response());
    }
    

    let scramblings = scrambler(word);

    let resp = ApiGatewayResponse {
        status_code: 200,
        body: CustomOutput {
            message: String::from("success"),
            results: scramblings,
        }
    };

    Ok(resp.into_response())
}

pub fn scrambler(starting_string: &str) -> Vec<String> {
    let hashmap_starting_string = string_to_hashmap(starting_string);
    
    let file_data = include_str!("../words.txt");
    let mut results = Vec::new();

    for word in file_data.lines() {
        let mut valid = true;
        let comparison = string_to_hashmap(word);
    
        'key_loop: for key in comparison.keys() {
            match hashmap_starting_string.get(key) {
                Some(letter_counts) => {
                    if comparison.get(key).unwrap() > letter_counts {
                        valid = false;
                        break 'key_loop;
                    }
                },
                None => {
                    valid = false;
                    break 'key_loop;
                },
            }
        }

        if valid {
            results.push(String::from(word));
        }
    }
    
    results
        .into_iter()
        .sorted()
        .collect()
}

fn string_to_hashmap(input_string: &str) -> HashMap<char,i32> {
    let mut letter_counts: HashMap<char,i32> = HashMap::new();

    let char_vec: Vec<char> = input_string.to_lowercase().chars().collect();
    for c in char_vec {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    
    letter_counts
}

#[test]
fn scrambler_test() {
    assert_eq!(scrambler("restaurant"), ["a", "area", "arena", "arrest", "art", "as", "at", "aunt", "ear",
        "earn", "east", "eat", "era", "nature", "near", "neat", "nest", "net", "nurse", "nut", "rare",
        "rat", "rate", "rear", "rent", "rest", "restaurant", "return", "run", "sea", "seat", "set", "star",
        "stare", "start", "starter", "state", "statue", "sue", "sun", "sure", "taste", "tea", "tear", "ten",
        "tent", "test", "treat", "true", "trust", "tune", "turn", "us", "use", "user"]);
    assert_eq!(scrambler("teacher"), ["a", "ace", "act", "ah", "arc", "arch", "art", "at", "car", "care",
        "cart", "cat", "chart", "cheat", "cheer", "create", "each", "ear", "earth", "eat", "era", "ha",
        "hat", "hate", "he", "hear", "heart", "heat", "her", "here", "race", "rat", "rate", "reach",
        "react", "tea", "teach", "teacher", "tear", "the", "there", "three", "trace", "tree"]);
    assert_eq!(scrambler("certificate"), ["a", "ace", "act", "after", "air", "arc", "art", "at", "car",
        "care", "cart", "cat", "cite", "craft", "create", "critic", "ear", "eat", "era", "face", "fact",
        "fair", "far", "fare", "fat", "fate", "fear", "fee", "fierce", "fire", "fit", "free", "i", "ice",
        "ie", "if", "it", "race", "rat", "rate", "react", "rice", "tactic", "tea", "tear", "tie", "tire",
        "trace", "trait", "treat", "tree"]);
}