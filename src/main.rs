use Rust::*;
use std::{env};

pub fn main() {

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: ./sim -s <num> -E <num> -b <num> -t <file>

Options:
  -s <num>   Number of set index bits.
  -E <num>   Number of lines per set.
  -b <num>   Number of block offset bits.
  -t <file>  Trace file.");
        std::process::exit(1);
    }

    let s: usize = match args[1].parse() {
        Ok(num) if num > 0 => num,
        _ => {
            eprintln!("Error: 's' must be an integer greater than 0.\n
Usage: ./sim -s <num> -E <num> -b <num> -t <file>

Options:
  -s <num>   Number of set index bits.
  -E <num>   Number of lines per set.
  -b <num>   Number of block offset bits.
  -t <file>  Trace file.");
            std::process::exit(1);
        },
    };

    let e: usize = match args[2].parse() {
        Ok(num) if num > 0 => num,
        _ => {
            eprintln!("Error: 'E' must be an integer greater than 0\n
Usage: ./sim -s <num> -E <num> -b <num> -t <file>

Options:
  -s <num>   Number of set index bits.
  -E <num>   Number of lines per set.
  -b <num>   Number of block offset bits.
  -t <file>  Trace file.");
            std::process::exit(1);
        },
    };

    let b: usize = match args[3].parse() {
        Ok(num) if num > 0 => num,
        _ => {
            eprintln!("Error: 'b' must be an integer greater than 0\n
Usage: ./sim -s <num> -E <num> -b <num> -t <file>

Options:
  -s <num>   Number of set index bits.
  -E <num>   Number of lines per set.
  -b <num>   Number of block offset bits.
  -t <file>  Trace file.");
            std::process::exit(1);
        },
    };

    let tracefile = &args[4];

    // Use the `read_trace_file` function to read the trace file
    let lines = match read_trace_file(tracefile) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Failed to read trace file: {}. \n
Usage: ./sim -s <num> -E <num> -b <num> -t <file>

Options:
  -s <num>   Number of set index bits.
  -E <num>   Number of lines per set.
  -b <num>   Number of block offset bits.
  -t <file>  Trace file.", e);
            return;
        }
    };

    // initialize a new cache to see it's working
    let mut cache = Cache::new(s, e);

    // Process the trace
    hits_misses_evictions_calc(&mut cache, lines, s, b);

    // Print cache statistics for verification
    cache.report_simulation_results();

}