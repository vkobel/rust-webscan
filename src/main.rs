// for later use maybe
//extern crate getopts;
//use getopts::Options;

extern crate hyper;
extern crate regex;
extern crate sha1;

use hyper::Client;
use hyper::header::Connection;
use regex::Regex;
use std::env;
use std::io::Read;

struct WebCrawler {
    client: hyper::Client,
    base_host: String,
    href_regex: regex::Regex,
    sha1: sha1::Sha1,

    links: Vec<String>
}
impl WebCrawler {
    fn new(host: &str) -> WebCrawler {
        WebCrawler {
            client: Client::new(),
            base_host: host.to_string(),
            href_regex: Regex::new("href=[\'\"]?([^\'\" >]+)").unwrap(),
            sha1: sha1::Sha1::new(),
            links: Vec::new()
        }
    }

    fn get_web_page(&self, uri: &str) -> String {
        let url = self.base_host.clone() + uri;
        let mut res = self.client.get(&url).header(Connection::close()).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }

    fn extract_links(&mut self, html: &str) {
        for link in self.href_regex.captures_iter(html){
            let link_val = link.at(1).unwrap();
            self.links.push(link_val.to_string());
        }
    }

    fn do_sha1(&mut self, text: &str) -> String {
        self.sha1.reset();
        self.sha1.update(text.as_bytes());
        self.sha1.hexdigest()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "Not enough args!");

    let ref host = args[1];
    println!("======== Host is {} ========", host);

    let mut wc = WebCrawler::new(host);
    let ref body = wc.get_web_page("/");
    let sha1 = wc.do_sha1(body);

    wc.extract_links(body);

    println!("{:?}", sha1);
    println!("{:?}", wc.links);

}
