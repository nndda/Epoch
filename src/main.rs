use {
    notify_rust::Notification,
    regex::Regex,
    rodio::{source::Source, Decoder, OutputStream, Sink},

    std::{
        io::{self, Cursor, Stdout, Write},
        thread,
        time::{Duration, Instant},
    },
};

fn main() {
    let mut stdout: Stdout = io::stdout();
    let mut inp: String = String::new();

    print!("\rWait for: ");
    stdout.flush().unwrap();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");

    print!("\x1B[1A\x1B[2K");
    stdout.flush().unwrap();

    timer(parse_inp(inp.split_whitespace().collect()));
}

fn parse_inp(inp_arr: Vec<&str>) -> u32 {
    let re: Regex = Regex::new(r"^(\d+)(s|m|h)$").unwrap();
    let mut sec: u32 = 0;

    for e in &inp_arr {
        if re.is_match(e) {
            let caps: regex::Captures<'_> = re.captures(e).unwrap();
            let num: u32 = caps[1].parse().unwrap();

            match &caps[2] {
                "s" => sec += num,
                "m" => sec += num * 60,
                "h" => sec += num * 3600,
                _ => (),
            }
        }
    }

    if sec <= 0 {
        println!("\rInput error: wait time is ≤ 0");
        main();
    }

    return sec;
}

fn format_time(s: u32) -> String {
    if s >= 3600 {
        return format!("{}h {}m {}s", s / 3600, s % 3600 / 60, s % 60);
    } else if s >= 60 {
        return format!("{}m {}s", s / 60, s % 60);
    }
    return format!("{}s", s);
}

fn timer(sec: u32) {
    let mut stdout: Stdout = io::stdout();
    let start: Instant = Instant::now();

    for i in 0..sec {
        let elapsed: u32 = start.elapsed().as_secs() as u32;

        if elapsed >= sec {
            break;
        }

        let time_str: String = format_time(sec - elapsed);

        print!("\rWaiting for {time_str} ");
        stdout.flush().unwrap();

        let next_tick: Duration = Duration::from_secs((i + 1) as u64);
        let now: Duration = start.elapsed();

        if next_tick > now {
            thread::sleep(next_tick - now);
        }
    }

    let end_str: String = format!("Finished waiting for {}s ^^", format_time(sec));

    println!("\r{end_str}\nPress Ctrl+C to quit...");

    Notification::new()
        .summary("Epoch: timeout")
        .body(&end_str)
        .timeout(15)
        .show()
        .unwrap();

    let (_stream, stream_handle): (OutputStream, rodio::OutputStreamHandle) =
        OutputStream::try_default().unwrap();

    let source: rodio::source::Repeat<Decoder<Cursor<&_>>> =
        Decoder::new(Cursor::new(include_bytes!("./beep.ogg").as_ref())).unwrap().repeat_infinite();

    let sink: Sink = rodio::Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}
