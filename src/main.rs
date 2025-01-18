use {
    notify_rust::Notification,
    once_cell::sync::Lazy,
    regex::Regex,
    // crossterm::terminal,
    rodio::{source::Source, Decoder, OutputStream, Sink},

    std::{
        io::{self, /*BufReader,*/ Cursor, Write},
        // fs::File,
        thread,
        time::{Duration, Instant},
    },
};

fn main() {
    let mut inp: String = String::new();

    print!("\rWait for: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");

    print!("\x1B[1A\x1B[2K");
    io::stdout().flush().unwrap();

    timer(parse_inp(inp.split_whitespace().collect()));
}

const INP_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)(s|m|h)$").unwrap());

fn parse_inp(inp_arr: Vec<&str>) -> u32 {
    let mut sec: u32 = 0;

    for e in &inp_arr {
        if INP_RE.is_match(e) {
            let caps: regex::Captures<'_> = INP_RE.captures(e).unwrap();
            let num: u32 = caps[1].parse().unwrap();

            match &caps[2] {
                // "s" => sec += num,
                "m" => sec += num * 60,
                "h" => sec += num * 3600,
                _ => sec += num,
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
    let start: Instant = Instant::now();

    for i in 0..sec {
        let elapsed: u32 = start.elapsed().as_secs() as u32;

        if elapsed >= sec {
            break;
        }

        let time_str: String = format_time(sec - elapsed);
        // TODO
        // let term_w = terminal::size().unwrap().1 as usize;

        // print!(
        //     "\rWaiting for {time_str} {}",
        //     "=".repeat(
        //         (term_w - 15 - time_str.len()) as usize
        //     )
        // );
        print!("\rWaiting for {time_str}");
        io::stdout().flush().unwrap();

        let next_tick: Duration = Duration::from_secs((i + 1) as u64);
        let now: Duration = start.elapsed();

        if next_tick > now {
            thread::sleep(next_tick - now);
        }
    }

    let end_str: String = format!("Finished waiting for {sec}s ^^");

    println!("\r{end_str}\nPress Ctrl+C to quit...");

    Notification::new()
        .summary("Epoch: timeout")
        .body(&end_str)
        .timeout(15)
        .show()
        .unwrap();

    let (_stream, stream_handle): (OutputStream, rodio::OutputStreamHandle) =
        OutputStream::try_default().unwrap();

    let source: Decoder<Cursor<&[u8]>> =
        Decoder::new(Cursor::new(include_bytes!("./beep.ogg").as_ref())).unwrap();

    let sink: Sink = rodio::Sink::try_new(&stream_handle).unwrap();

    sink.append(source.repeat_infinite());
    sink.sleep_until_end();
}
