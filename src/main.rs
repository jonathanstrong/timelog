use std::time::*;
use std::process;
use std::env;

const CURL: &str = "/usr/bin/curl";
const HOST: &str = "coolidge";

fn escape_tag(s: &str) -> String {
    s.replace(" ", r"\ ")
     .replace("'", "")
     .replace("\"", "")
     .replace(",", r"\,")
     .replace("=", r"\=")
}

fn main() {
    let now = SystemTime::now();
    let dur = match now.duration_since(UNIX_EPOCH) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("failed to calculate timestamp: {}", e);
            process::exit(1)
        }
    };
    let pwd = match env::var("PWD") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("failed to retrieve PWD from environment: {}", e);
            process::exit(2)
        }
    };

    let postdata =
        format!("bash,pwd={pwd},host={host} n=1 {millis}",
            pwd = escape_tag(&pwd),
            host = HOST,
            millis = dur.as_millis());

    let localhost = "http://localhost:8086/write?db=timelog&precision=ms";
    let penthu = "http://penthu:8086/write?db=timelog&precision=ms";

    for url in &[localhost, penthu] {
        let verbosity = if cfg!(feature = "debug") { "--verbose" } else { "--silent" };

        let args = &[
            "-XPOST",
            url,
            "--data-binary",
            &postdata,
            verbosity,
        ];

        let mut cmd = process::Command::new(CURL);
        cmd.args(args);

        if cfg!(feature = "debug") {
            println!("{}", args.join(" "));
            let out = cmd.output().expect("cmd.output()");
            println!("{:?}", out.status);
            println!("{}", std::str::from_utf8(&out.stdout[..]).expect("from_utf8"));
            if !out.stderr.is_empty() {
                println!("{}", std::str::from_utf8(&out.stderr[..]).expect("from_utf8"));
            }
        } else {
            let _ = cmd.spawn().ok();
        }
    }
}
