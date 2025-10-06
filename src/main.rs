use reqwest::blocking::get;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;
use std::fs::OpenOptions;

fn getinp(var: &mut String, txt: &str) {
    print!("{}",txt);
    io::stdout().flush().expect("An error occured ...");
    let mut output: String = String::new();
    io::stdin().read_line(&mut output).expect("An error occured ...");
    output = output.trim().to_string();
    *var = output;
}

fn main() {
    println!("Welcome to sitewatcheR - Remake of old project made with go ...");
    let mut url: String = String::new();
    let mut durationtxt: String = String::new();
    let mut shownftxt: String = String::new();
    let mut addtologtxt: String = String::new();

    getinp(&mut url, "-Enter URL to watch : ");
    getinp(&mut durationtxt, "-Enter time to wait between requests (seconds) : ");
    getinp(&mut shownftxt, "-Do you want to get notified when cannot find? (y for yes, anything for no) : ");
    getinp(&mut addtologtxt, "-Do you want log? (y for yes, anything for no) : ");

    let duration_secs: u64 = durationtxt.parse().unwrap();
    let duration: Duration = Duration::from_secs(duration_secs);

    let mut showft: bool = false;
    if shownftxt == "y".to_string() {
        showft = true;
    }
    let mut addtolog: bool = false;
    let mut logfile = OpenOptions::new().append(true).create(true).open("log.txt").expect("Error: Cannot open or create log file ...");
    if addtologtxt == "y".to_string() {
        addtolog = true;
    }

    let mut oldsite:String = get(&url).unwrap().text().unwrap();

    loop {
        sleep(duration);

        let site: String = get(&url).unwrap().text().unwrap();

        if site != oldsite {
            println!("----- A change detected! -> time : {} -----",Local::now());
            if addtolog {
                writeln!(logfile,"----- A change detected! -> time : {} -----",Local::now()).expect("Error: Cannot write to log ...");
            }
            oldsite = site;
        } else if showft {
            println!("No change found ...")
        }
    }
}