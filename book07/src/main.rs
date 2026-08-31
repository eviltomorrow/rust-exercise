use std::io::BufRead;
use std::io::{self};

fn main() {
    println!("Hello, world!");

    let r = pirate_share(100, 10);
    println!("{}", r);

    let r = pirate_share(100, 10);
    println!("{}", r);

    match get_weather(LatLng::NEWYORK) {
        Ok(report) => {
            display_weather(LatLng::NEWYORK, &report);
        }
        Err(err) => {
            println!("error querying the weather: {}", err);
        }
    }

    let r = get_weather(LatLng::NEWYORK);
    println!("{}", r.is_ok());
    println!("{}", r.is_err());

    if r.is_ok() {
        println!("{:?}", r.ok());
    } else {
        println!("{:?}", r.err());
    }

    const THE_USUAL: WeatherReport = WeatherReport::Rain;
    let report = get_weather(LatLng::NEWYORK).unwrap_or(THE_USUAL);
    println!("{:?}", report);

    let report = get_weather(LatLng::NEWYORK).unwrap_or_else(|_err| WeatherReport::Rain);
    println!("{:?}", report);

    let report = get_weather(LatLng::NEWYORK).unwrap();
    println!("{:?}", report);

    let report = get_weather(LatLng::NEWYORK).expect("not found");
    println!("{:?}", report);

    let report = get_weather(LatLng::NEWYORK);
    let r = report.as_ref();
    let r = r.unwrap();
    println!("{:?}", r);

    let mut report = get_weather(LatLng::NEWYORK);
    let r = report.as_mut();
    println!("{:?}", r);

    let r = throw_get_weather();
    match r {
        Ok(_) => println!("OK"),
        Err(err) => print_error(&err),
    }

    let c = build_generic_error();
    println!("{:?}", c);
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn build_generic_error() -> GenericResult<()> {
    let io_error = io::Error::new(io::ErrorKind::Other, "timed out");
    return Err(GenericError::from(io_error));
}

#[allow(dead_code)]
fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

fn throw_get_weather() -> Result<(), io::Error> {
    let _ = get_weather(LatLng::NEWYORK)?;
    Ok(())
}

use std::error::Error;
use std::io::Write;
use std::io::stderr;

#[allow(dead_code)]
fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

#[derive(Debug)]
enum LatLng {
    NEWYORK,
}

#[derive(Debug)]
enum WeatherReport {
    Sunny,
    Rain,
}

fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error> {
    match location {
        LatLng::NEWYORK => Ok(WeatherReport::Sunny),
    }
}

fn display_weather(location: LatLng, report: &WeatherReport) {
    println!("{:?}: {:?}", location, report);
}
