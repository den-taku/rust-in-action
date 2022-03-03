use chrono::{DateTime, Local, TimeZone};
use clap::{Arg, Command};
#[cfg(not(windows))]
use libc;
use std::mem::zeroed;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use libc::{settimeofday, timezone};
        use libc::{suseconds_t, time_t, timeval};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}

fn main() {
    let app = Command::new("clock")
        .version("0.1")
        .about("Gets and (aspirationally) sets the time.")
        .arg(
            Arg::new("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::new("std")
                .short('s')
                .long("standard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(Arg::new("datetime").help(
            "When <action> is 'set', apply <datetime>. \
                Otherwise, ignore.",
        ));

    let args = app.get_matches();
    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        let t_ = args.value_of("datetime").unwrap();

        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };

        let err_msg = format!("Unable to parse {t_} according to {std}");
        let t = parser(t_).expect(&err_msg);

        Clock::set(t)
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
