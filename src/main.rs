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

    if long {
        return str;
    }
    str.replace("and ", "")
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
        .replace("1 year", "1y")
}

fn main() -> Result<(), ParseError> {
    let options = Options::from_args();

    match DateTime::parse_from_rfc2822(&options.datetime) {
        Ok(dt) => {
            let str = humanize(dt, options.long, options.precise);
            println!("{}", str.replace(" ", &options.separator));
            std::process::exit(0);
        }
        Err(_) => std::process::exit(1),
    }
}

#[cfg(test)]
mod tests {

    use test_case::test_case;

    use super::*;

    use chrono::Duration;

    fn create_dt(duration: Duration) -> DateTime<FixedOffset> {
        let _dt = chrono::Local::now() - duration;

        _dt.with_timezone(_dt.offset())
    }

    #[test_case(chrono::Duration::seconds(1), "now"; "1 second")]
    #[test_case(chrono::Duration::seconds(30), "30s"; "30 seconds")]
    #[test_case(chrono::Duration::minutes(1), "1m"; "1 minute")]
    #[test_case(chrono::Duration::minutes(30), "30m"; "30 minutes")]
    #[test_case(chrono::Duration::hours(1), "1h"; "1 hour")]
    #[test_case(chrono::Duration::hours(10), "10h"; "10 hours")]
    #[test_case(chrono::Duration::hours(30), "1d"; "1 day")]
    #[test_case(chrono::Duration::hours(50), "2d"; "2 days")]
    #[test_case(chrono::Duration::hours(24 * 7), "1w"; "1 week")]
    #[test_case(chrono::Duration::hours(24 * 7 * 3), "3w"; "3 weeks")]
    #[test_case(chrono::Duration::hours(24 * 7 * 6), "1mo"; "1 month")]
    #[test_case(chrono::Duration::hours(24 * 7 * 40), "9mo"; "9 months")]
    fn short(duration: Duration, expected: &str) {
        let dt = create_dt(duration);
        let actual = humanize(dt, false, false);

        assert_eq!(expected, actual);
    }

    #[test_case(chrono::Duration::seconds(1), "now"; "1 second")]
    #[test_case(chrono::Duration::seconds(30), "30 seconds"; "30 seconds")]
    #[test_case(chrono::Duration::minutes(1), "a minute"; "1 minute")]
    #[test_case(chrono::Duration::minutes(30), "30 minutes"; "30 minutes")]
    #[test_case(chrono::Duration::hours(1), "an hour"; "1 hour")]
    #[test_case(chrono::Duration::hours(10), "10 hours"; "10 hours")]
    #[test_case(chrono::Duration::hours(30), "a day"; "1 day")]
    #[test_case(chrono::Duration::hours(50), "2 days"; "2 days")]
    #[test_case(chrono::Duration::hours(24 * 7), "a week"; "1 week")]
    #[test_case(chrono::Duration::hours(24 * 7 * 3), "3 weeks"; "3 weeks")]
    #[test_case(chrono::Duration::hours(24 * 7 * 6), "a month"; "1 month")]
    #[test_case(chrono::Duration::hours(24 * 7 * 40), "9 months"; "9 months")]
    fn long(duration: Duration, expected: &str) {
        let dt = create_dt(duration);
        let actual = humanize(dt, true, false);

        assert_eq!(expected, actual);
    }
    #[test_case(chrono::Duration::seconds(1), "1s"; "1 second")]
    #[test_case(chrono::Duration::seconds(30), "30s"; "30 seconds")]
    #[test_case(chrono::Duration::minutes(1), "1m"; "1 minute")]
    #[test_case(chrono::Duration::minutes(30), "30m"; "30 minutes")]
    #[test_case(chrono::Duration::hours(1), "1h"; "1 hour")]
    #[test_case(chrono::Duration::hours(10), "10h"; "10 hours")]
    #[test_case(chrono::Duration::hours(30), "1d 6h"; "1 day, 6hours")]
    #[test_case(chrono::Duration::hours(50), "2d 2h"; "2 days, 2hours")]
    #[test_case(chrono::Duration::hours(24 * 7), "1w"; "1 week")]
    #[test_case(chrono::Duration::hours(24 * 7 * 3), "3w"; "3 weeks")]
    #[test_case(chrono::Duration::hours(24 * 7 * 6), "1mo 1w 5d"; "1 month, 1week, 5 day")]
    #[test_case(chrono::Duration::hours(24 * 7 * 40), "9mo 1w 3d"; "9 months, 1 week, 3 days")]
    fn short_precise(duration: Duration, expected: &str) {
        let dt = create_dt(duration);
        let actual = humanize(dt, false, true);

        assert_eq!(expected, actual);
    }

    #[test_case(chrono::Duration::seconds(1), "1 second"; "1 second")]
    #[test_case(chrono::Duration::seconds(30), "30 seconds"; "30 seconds")]
    #[test_case(chrono::Duration::minutes(1), "1 minute"; "1 minute")]
    #[test_case(chrono::Duration::minutes(30), "30 minutes"; "30 minutes")]
    #[test_case(chrono::Duration::hours(1), "1 hour"; "1 hour")]
    #[test_case(chrono::Duration::hours(10), "10 hours"; "10 hours")]
    #[test_case(chrono::Duration::hours(30), "1 day and 6 hours"; "1 day, 6hours")]
    #[test_case(chrono::Duration::hours(50), "2 days and 2 hours"; "2 days, 2hours")]
    #[test_case(chrono::Duration::hours(24 * 7), "1 week"; "1 week")]
    #[test_case(chrono::Duration::hours(24 * 7 * 3), "3 weeks"; "3 weeks")]
    #[test_case(chrono::Duration::hours(24 * 7 * 6), "1 month 1 week and 5 days"; "1 month, 1week, 5 day")]
    #[test_case(chrono::Duration::hours(24 * 7 * 40), "9 months 1 week and 3 days"; "9 months, 1 week, 3 days")]
    fn long_precise(duration: Duration, expected: &str) {
        let dt = create_dt(duration);
        let actual = humanize(dt, true, true);

        assert_eq!(expected, actual);
    }
}
