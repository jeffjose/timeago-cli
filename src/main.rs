use chrono::DateTime;
use chrono::ParseError;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "A simple tool to humanize datetime")]
struct Options {
    datetime: String,
}

fn main() -> Result<(), ParseError> {
    let options = Options::from_args();

    let dt = DateTime::parse_from_rfc2822(&options.datetime.to_string()).unwrap();
    let ht = HumanTime::from(dt);
    let str = ht
        .to_text_en(Accuracy::Rough, Tense::Past)
        .replace(",", "")
        .replace(" ago", "")
        .replace("and ", "")
        .replace(" days", "d")
        .replace(" hours", "h")
        .replace(" minutes", "m")
        .replace(" seconds", "s")
        .replace(" weeks", "w")
        .replace("a month", "1mo")
        .replace("a minute", "1m")
        .replace(" months", "mo");

    println!("{}", str);

    Ok(())
}