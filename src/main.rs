// for later use maybe
//extern crate getopts;
//use getopts::Options;

extern crate hyper;
extern crate regex;

use hyper::Client;
use hyper::header::Connection;
use regex::Regex;
use std::env;
use std::io::{self, Write, Read};
use std::fs::File;

struct WebCrawler {
    client: hyper::Client,
    base_host: String,
    href_regex: regex::Regex,
    links_to_visit: Vec<String>,
    links_visited: Vec<String>,
    files_found: Vec<String>
}
impl WebCrawler {
    fn new(host: &str) -> WebCrawler {
        WebCrawler {
            client: Client::new(),
            base_host: host.to_string(), // should remove all trailing slashes
            href_regex: Regex::new("href=[\'\"]?([^\'\" >]+)").unwrap(),
            links_to_visit: Vec::new(),
            links_visited: Vec::new(),
            files_found: Vec::new()
        }
    }

    fn get_web_page(&mut self, uri: &str) -> (String, String) {
        let url = self.base_host.clone() + "/" + uri;
        let mut res = self.client.get(&url).header(Connection::close()).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        self.links_visited.push(uri.to_string());
        (uri.to_string(), body)
    }

    fn extract_links(&mut self, html: &str, url: &str) {
        for link in self.href_regex.captures_iter(html){
            let link_ref = link.at(1).unwrap().to_string();

            // !!! Lazy checking here - to improve !!!
            // Drops get parameters (start with ?)...
            // Only folder check is ending by /
            if !self.links_visited.contains(&link_ref) && !link_ref.starts_with("http") && !link_ref.starts_with("?"){
                if link_ref.ends_with("/") {
                    self.links_to_visit.push(url.to_string() + &link_ref);
                } else {
                    self.files_found.push(url.to_string() + &link_ref);
                }
            }
        }
    }

    fn explore(&mut self) {
        println!("Starting...");

        loop {
            
            print!("\rRemaining: {} - Files found: {}             ", self.links_to_visit.len(), self.files_found.len());
            io::stdout().flush().unwrap();

            let val = self.links_to_visit.pop();
            if val == None {
                break;
            }

            let (url, body) = self.get_web_page(&val.unwrap());
            self.extract_links(&body, &url);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: webcan [host] [outfile_pattern]");
        return;
    }

    let ref host = args[1];
    let ref outfile = args[2];
    println!("======== ALPHA - only follows relative links! ========");
    println!("======== Host is {} ========\n", host);

    let mut wc = WebCrawler::new(host);
    let (url, body) = wc.get_web_page("/");
    wc.extract_links(&body, &url);
    wc.explore();

    let mut buffer = File::create(outfile).unwrap();
    for l in wc.files_found {
        buffer.write_fmt(format_args!("{}\n", l)).unwrap();
    }
    println!("\nDone! Results are in {}", outfile);

}
