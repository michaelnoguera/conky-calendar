/*
    Conky-Calendar prints a simple calendar with conky format strings embedded.
    Copyright (c) 2020 Michael Noguera. Freely avaliable under the MIT License:

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

use chrono::{Datelike, Local, NaiveDate};
use clap::{App, Arg};

/// Returns true if specified year is a leap year.
fn is_leap_year(year: i32) -> bool {
    (year % 4) == 0 && ((year % 100) != 0 || (year % 400) == 0)
}

/// Returns the number of days in a given date's month.
fn days_in_month<T: Datelike>(date: T) -> u32 {
    match date.month() {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(date.year()) {
                29
            } else {
                28
            }
        }
        _ => panic!("invalid month: {}", date.month()),
    }
}

// Could work, but ballooned in complexity too fast. Wish there was a way to overload functions.
//fn print_stylized(output: &str, color: Option<&str>, x_coord: Option<&str>) {
//    let goto: &str = if let Some(x) = x_coord {format!("${{goto {x}}}}", x = x)} else {""};
//    let color_pre: &str = if let Some(x) = color {format!("${{color #{color}}}", color = x)} else {""};
//    let color_post: &str = if let Some(x) = color {"${{color}}"} else {""};
//    println!("{:?}", [goto, color_pre, output, color_post].concat());
//}

fn main() {
    //use clap to get command line args
    let matches = App::new("conky-calendar")
        .version(clap::crate_version!())
        .author("by Michael Noguera")
        .about("Prints a simple calendar with conky format strings embedded.")
        .arg(
            Arg::with_name("label-color")
                .short("l")
                .long("label-color")
                .help("Hex color to use for the month name and weekday labels. Omit the '#' sign.")
                .value_name("COLOR")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("today-color")
                .short("t")
                .long("today-color")
                .help("Hex color to use for the current day. Omit the '#' sign.")
                .value_name("COLOR")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("weekend-color")
                .short("w")
                .long("weekend-color")
                .help("Hex color to use for weekends. Omit the '#' sign.")
                .value_name("COLOR")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("day-color")
                .short("d")
                .long("day-color")
                .help("Hex color to use for otherwise-uncolored days. Omit the '#' sign.")
                .value_name("COLOR")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("monday-as-first")
                .short("m")
                .long("monday-as-first")
                .help("Use Monday as first day of the week"),
        )
        .get_matches();

    let now = Local::now(); // current date & time via chrono

    let labels = match matches.is_present("monday-as-first") {
        true => String::from("Mo Tu We Th Fr Sa Su"),
        false => String::from("Su Mo Tu We Th Fr Sa"),
    };

    // weekday labels
    if matches.is_present("label-color") {
        println!(
            "${{color #{color}}}{:^20}\n{}${{color}}",
            now.format("%B %Y"),
            labels,
            color = matches.value_of("label-color").unwrap()
        );
    } else {
        println!("{:^20}\n{}", now.format("%B %Y"), labels);
    }

    // calculate where the numbers go, add spaces to offset the first day
    let first_day_of_month = NaiveDate::from_ymd(now.year(), now.month(), 1);

    let initial_offset = match matches.is_present("monday-as-first") {
        true => first_day_of_month.weekday().num_days_from_monday(),
        false => first_day_of_month.weekday().num_days_from_sunday(),
    };

    let (saturday_day, sunday_day) = match matches.is_present("monday-as-first") {
        true => (5, 6),
        false => (6, 0),
    };

    let mut col = 0;
    for _ in 0..initial_offset {
        print!("{:^3}", "");
        col = col + 1;
    }

    // output days, colorized if requested
    for day in 1..=days_in_month(now) {
        if col > 6 {
            // col 0 is sunday, 6 is saturday, etc.
            println!();
            col = 0;
        };
        if matches.is_present("today-color") && day == now.day() {
            // color today if needed, overrides later options
            print!(
                "${{color #{color}}}{:^3}${{color}}",
                day,
                color = matches.value_of("today-color").unwrap()
            );
        } else if matches.is_present("weekend-color") && (col == saturday_day || col == sunday_day)
        {
            // color weekends if needed
            print!(
                "${{color #{color}}}{:^3}${{color}}",
                day,
                color = matches.value_of("weekend-color").unwrap()
            );
        } else if matches.is_present("day-color") {
            // generic day color if specified
            print!(
                "${{color #{color}}}{:^3}${{color}}",
                day,
                color = matches.value_of("day-color").unwrap()
            );
        } else {
            // default to no format strings
            print!("{:^3}", day);
        }
        col = col + 1;
    }
    println!();
}
