use chrono::Datelike;
use digits::{ALTERNATE_SEPARATORS, DIGITS, SPACE};
use eras::{ERAS, SHORTENED_ERAS};

mod digits;
mod eras;

pub fn parse(japanese: &str) -> Option<chrono::NaiveDate> {
    if let Some(date) = parse_gregorian(japanese) {
        return Some(date);
    }

    if let Some(date) = parse_era(japanese) {
        return Some(date);
    }

    parse_shortened(japanese)
}

fn parse_gregorian(japanese: &str) -> Option<chrono::NaiveDate> {
    let mut iter = japanese.chars().peekable();

    if !DIGITS.contains_key(iter.peek()?) {
        return None;
    }

    let year = iter
        .by_ref()
        .take_while(|c| c != &digits::YEAR_MARKER)
        .collect::<String>();

    if year.chars().count() == 0 || year.chars().count() > 4 {
        return None;
    }

    let month = iter
        .by_ref()
        .take_while(|c| c != &digits::MONTH_MARKER)
        .collect::<String>();

    if month.chars().count() == 0 || month.chars().count() > 2 {
        return None;
    }

    let day = iter
        .by_ref()
        .take_while(|c| c != &digits::DAY_MARKER)
        .collect::<String>();

    if day.chars().count() == 0 || day.chars().count() > 2 {
        return None;
    }

    if iter.peek().is_some() {
        let mut iter = iter.skip_while(|c| SPACE.contains(&c));
        let next = iter.next()?;
        if !digits::OPEN_PARENS.contains(&next) {
            return None;
        }

        if iter.clone().count() != 4 && iter.clone().count() != 2 {
            return None;
        }

        let weekday = iter
            .by_ref()
            .take_while(|c| !digits::CLOSE_PARENS.contains(&c))
            .collect::<String>();

        if !digits::WEEKDAYS.contains(&weekday.as_str()) {
            return None;
        }
        if iter.next().is_some() {
            return None;
        }
    } else {
        if iter.next().is_some() {
            return None;
        }
    }

    let year_value = parse_number_string(&year)?.try_into().ok()?;
    let month_value = parse_number_string(&month)?.try_into().ok()?;
    let day_value = parse_number_string(&day)?.try_into().ok()?;

    chrono::NaiveDate::from_ymd_opt(year_value, month_value, day_value)
}

fn parse_era(japanese: &str) -> Option<chrono::NaiveDate> {
    let iter = japanese.chars().peekable();
    let era_str = iter
        .clone()
        .take_while(|c| !digits::DIGITS.contains_key(&c))
        .collect::<String>();

    let era_offset = *ERAS.get(&era_str.as_str())? - 1;

    let remaining = iter.skip(era_str.chars().count()).collect::<String>();

    let base_date = parse_gregorian(&remaining)?;
    let years = base_date.year() + era_offset;
    let months = base_date.month();
    let days = base_date.day();
    chrono::NaiveDate::from_ymd_opt(years, months, days)
}

fn parse_shortened(japanese: &str) -> Option<chrono::NaiveDate> {
    let mut iter = japanese.chars().peekable();

    let first = iter.peek()?;
    let era_offset = if SHORTENED_ERAS.contains_key(&first) {
        let era = *SHORTENED_ERAS.get(&first)?;
        iter.next();
        era - 1
    } else {
        2000
    };

    let year = iter
        .by_ref()
        .take_while(|c| !ALTERNATE_SEPARATORS.contains(&c))
        .collect::<String>();
    if year.chars().count() == 0 || year.chars().count() > 2 {
        return None;
    }

    let month = iter
        .by_ref()
        .take_while(|c| !ALTERNATE_SEPARATORS.contains(&c))
        .collect::<String>();
    if month.chars().count() == 0 || month.chars().count() > 2 {
        return None;
    }

    let day = iter
        .by_ref()
        .take_while(|c| !ALTERNATE_SEPARATORS.contains(&c))
        .collect::<String>();
    if day.chars().count() == 0 || day.chars().count() > 2 {
        return None;
    }

    if iter.next().is_some() {
        return None;
    }

    let year_value = parse_number_string(&year)?;
    let month_value = parse_number_string(&month)?.try_into().ok()?;
    let day_value = parse_number_string(&day)?.try_into().ok()?;

    let year_value = (year_value + era_offset).try_into().ok()?;

    chrono::NaiveDate::from_ymd_opt(year_value, month_value, day_value)
}

fn parse_number_string(string: &str) -> Option<u32> {
    string
        .chars()
        .map(|c| DIGITS.get(&c))
        .fold(Some(0), |acc, x| Some(acc? * 10 + x?))
}
