use std::thread;
use std::time::*;

fn main() {
    //println!("{:?}", std::env::args().collect::<Vec<String>>());
    //eprintln!("launching a thread");
    //thread::spawn(move || {
    //    eprintln!("sleeping in thread");
    //    thread::sleep(Duration::from_secs(5));
    //    eprintln!("done sleeping in thread");
    //});
    //eprintln!("exiting");
    //std::process::exit(0);
    std::process::Command::new("/Users/jstrong/src/timelog/measure.sh")
        .spawn()
        .ok();
    std::process::exit(0);
}
