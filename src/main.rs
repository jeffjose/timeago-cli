use chrono::DateTime;
use chrono::ParseError;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "A simple tool to humanize datetime")]
struct Options {
    datetime: String,
}

fn humanize(dt: DateTime<FixedOffset>) -> Result<String> {
    let ht = HumanTime::from(dt);
    let str = ht
        .to_text_en(Accuracy::Rough, Tense::Past)
        .replace(",", "")
        .replace(" ago", "")
        .replace("and ", "")
        .replace("a day", "1d")
        .replace(" days", "d")
        .replace("an hour", "1h")
        .replace(" hours", "h")
        .replace("a minute", "1m")
        .replace(" minutes", "m")
        .replace("a second", "1s")
        .replace(" seconds", "s")
        .replace("a week", "1w")
        .replace(" weeks", "w")
        .replace("a month", "1mo")
        .replace(" months", "mo")
        .replace("a year", "1y")
        .replace(" years", "y");

    return str;
}

fn main() -> Result<(), ParseError> {
    let options = Options::from_args();

    let dt = DateTime::parse_from_rfc2822(&options.datetime.to_string()).unwrap();

    let str = humanize(dt);

    println!("{}", str);

    Ok(())
}