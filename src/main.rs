// Plan 1: Read in a RSS xml file and generate an html file to read
use rss::Channel;
use rss::validation::Validate;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("feed.xml").unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();

    channel.validate().expect("Invalid RSS file.");

    let output_file = File::create("output.xml").unwrap();
    channel
        .write_to(output_file)
        .expect("Failed to write to output file.");
    println!("Wrote to output file")
}
