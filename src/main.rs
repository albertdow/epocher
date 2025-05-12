use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // Date to convert
    #[arg(short, long)]
    date: String,
    // Unit (seconds or milliseconds)
    #[arg(short, long, default_value = "s")]
    unit: String,
}

/// Convert date string to Unix epoch
fn unix_converter(date: &str, unit: &str) -> Result<u64, String> {
    let split_date: Vec<_> = date.split("-").collect();
    if split_date.len() != 3 {
        return Err("Date format must match: YYYY-MM-DD".to_string());
    }
    let year = split_date[0]
        .parse::<u64>()
        .map_err(|_| "Invalid year format".to_string())?;

    let month = split_date[1]
        .parse::<u64>()
        .map_err(|_| "Invalid month format".to_string())?;

    let day = split_date[2]
        .parse::<u64>()
        .map_err(|_| "Invalid day format".to_string())?;

    if !(1..=12).contains(&month) {
        return Err("Month must be within 1 and 12.".to_string());
    }

    let days_in_month = days_in_month(year, month);
    if day < 1 || day > days_in_month {
        return Err(format!("Day must be within 1 and {days_in_month}."));
    }

    let mut timestamp = calculate_epoch(year, month, day)?;

    if unit == "ms" {
        timestamp *= 1000;
    }
    if !["s", "ms"].contains(&unit) {
        return Err("Unit not in `s` or `ms`.".to_string());
    }

    Ok(timestamp)
}

/// Get days in month
fn days_in_month(year: u64, month: u64) -> u64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Check if leap year: divisible by 4 and not 100, or divisible by 400.
fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Calculate epoch timestamp
fn calculate_epoch(year: u64, month: u64, day: u64) -> Result<u64, String> {
    if year < 1970 {
        return Err("Date must be after Jan 1970 for valid Unix epoch.".to_string());
    }

    let mut days_since_epoch = day_of_year(year, month, day);
    for y in 1970..year {
        days_since_epoch += if is_leap_year(y) { 366 } else { 365 };
    }
    // 1 Jan 1970 should be 0. Currently would be 1.
    days_since_epoch -= 1;

    let seconds = days_since_epoch * 86_400;

    Ok(seconds)
}

/// Calculate day of year:
/// use current day of month and add days in month from previous months
fn day_of_year(year: u64, month: u64, day: u64) -> u64 {
    let mut doy = day;
    for m in 1..month {
        doy += days_in_month(year, m);
    }

    doy
}

fn main() {
    let args = Args::parse();
    match unix_converter(&args.date, &args.unit) {
        Ok(timestamp) => println!("{timestamp}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
