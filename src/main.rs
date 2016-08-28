extern crate hyper;
extern crate oauthcli;
extern crate serde_json;
extern crate serde_yaml;

use std::fs::File;
use std::io::Read;
use hyper::Url;
use hyper::client::Client;
use hyper::header::Authorization;
use oauthcli::{SignatureMethod, authorization_header, nonce, timestamp};
use serde_json::Value;
use std::collections::HashMap;

fn load_config() -> HashMap<String, String> {
    let mut yaml_file = File::open("twitter_key.yaml").unwrap();
    let mut s = String::new();
    yaml_file.read_to_string(&mut s).unwrap();
    let config: HashMap<String, String> = serde_yaml::from_str(&s).unwrap();
    return config;
}

fn main() {

	let config: HashMap<String, String> = load_config();
    
    let base = Url::parse("https://api.twitter.com/1.1/statuses/user_timeline.json").unwrap();
    let params = vec![
        ("screen_name".to_owned(), "donuuuuts".to_owned()),
        ("count".to_owned(), "1".to_owned()),
        //("trum_user".to_owned(), "0".to_owned()),
        ("contributor_details".to_owned(), "false".to_owned()),
        ("include_rts".to_owned(), "false".to_owned()),
        ("trim_user".to_owned(), "false".to_owned()),
    ];
    let mut url = base.clone();
	//url.set_query_from_pairs(params.iter().map(|&(ref k, ref v)| {
    //    (&k[..], &v[..])
    //}));
    url.query_pairs_mut()
        .append_pair("screen_name", "donuuuuts");
        
    url.query_pairs_mut()
        .append_pair("count", "1");

    url.query_pairs_mut()
        .append_pair("trim_user", "false");
    
    url.query_pairs_mut()
        .append_pair("include_rts", "false");

    url.query_pairs_mut()
        .append_pair("contributor_details", "false");

	let auth = authorization_header(
        "GET", base, None,
        config["consumer_key"].as_str(), config["consumer_secret"].as_str(),
        Some(config["access_token"].as_str()), Some(config["access_token_secret"].as_str()),
        SignatureMethod::HmacSha1,
        &timestamp(), &nonce(),
        None, None, params.into_iter(),
    );
    
    let client = Client::new();
    let request = client.get(url).header(Authorization(auth));
    let mut response = request.send().unwrap();
    let mut result = String::new();
    response.read_to_string(&mut result).unwrap();
    let data: Value = serde_json::from_str(&result).unwrap(); 
    
    let arr = data.as_array();

    for tw in arr.unwrap() {
        let text = tw.find("text");
        println!("{}", text.unwrap());
    }
}
   
