use chrono::ParseError;
use chrono::{DateTime, FixedOffset};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "A simple tool to humanize datetime")]
struct Options {
    datetime: String,

    #[structopt(short, long)]
    long: bool,

    #[structopt(short, long)]
    precise: bool,

    #[structopt(default_value = " ", short, long)]
    separator: String,
}

fn humanize(dt: DateTime<FixedOffset>, long: bool, precise: bool) -> String {
    let ht = HumanTime::from(dt);

    let str: String;
    if precise {
        str = ht
            .to_text_en(Accuracy::Precise, Tense::Past)
            .replace(",", "")
            .replace(" ago", "");
    } else {
        str = ht
            .to_text_en(Accuracy::Rough, Tense::Past)
            .replace(",", "")
            .replace(" ago", "");
    }

    if long == true {
        return str;
    }
    return str
        .replace("and ", "")
        .replace("a day", "1d")
        .replace("1 day", "1d")
        .replace(" days", "d")
        .replace("an hour", "1h")
        .replace("1 hour", "1h")
        .replace(" hours", "h")
        .replace("a minute", "1m")
        .replace(" minutes", "m")
        .replace("1 minute", "1m")
        .replace("a second", "1s")
        .replace(" seconds", "s")
        .replace("1 second", "1s")
        .replace("a week", "1w")
        .replace(" weeks", "w")
        .replace("1 week", "1w")
        .replace("a month", "1mo")
        .replace(" months", "mo")
        .replace("1 month", "1mo")
        .replace("a year", "1y")
        .replace(" years", "y")
        .replace("1 year", "1y");
}

fn main() -> Result<(), ParseError> {
    let options = Options::from_args();

    let dt = DateTime::parse_from_rfc2822(&options.datetime.to_string()).unwrap();

    let str = humanize(dt, options.long, options.precise);

    println!("{}", str.replace(" ", &options.separator.to_string()));

    Ok(())
}