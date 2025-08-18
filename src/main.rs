// Plan 1: Read in a RSS xml file and print the data to console or generate an html file to read
use rss::Channel;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("feed.xml").unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();
    println!("{channel}");
}
