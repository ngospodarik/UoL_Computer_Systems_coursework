use std::{env, io};
use std::io::{BufRead};
use std::fs::File;
use std::path::Path;

// Define the CacheLine struct. Each CacheLine represents a single cache line.
struct CacheLine {
    valid: bool, // Indicates if the line is valid (contains data).
    tag: u64,    // The tag part of the address stored in this cache line.
    last_used: u64, // Used for implementing LRU; higher number means more recently used.
}

// Define the CacheSet struct. Each CacheSet contains multiple CacheLines.
struct CacheSet {
    lines: Vec<CacheLine>, // A vector of CacheLine, represents all lines within a set.
}

// Define the Cache struct with its properties.
struct Cache {
    sets: Vec<CacheSet>, // A vector of CacheSet, represents all the sets in the cache.
    hits: u64,           // A counter for the number of cache hits.
    misses: u64,         // A counter for the number of cache misses.
    evictions: u64,      // A counter for the number of cache evictions.
}

impl Cache {
    // Define a method to create a new Cache instance.
    fn new(s: usize, e: usize, b: usize) -> Cache {
        let num_sets = 2_usize.pow(s as u32); // Calculate the number of sets (S=2^s).
        // Create all the sets for the cache.
        let sets = (0..num_sets).map(|_| {
            // For each set, create the specified number of lines (E).
            let lines = (0..e).map(|_| CacheLine {
                valid: false, // Initially, no line has data, so valid is false.
                tag: 0,       // Initial tag is set to 0, indicating no data is stored.
                last_used: 0, // Initialize last_used to 0 (will be updated on access).
            }).collect();
            CacheSet { lines } // Create a CacheSet with the generated lines.
        }).collect();

        // Return a new Cache instance with the generated sets and reset counters.
        Cache {
            sets,
            hits: 0,
            misses: 0,
            evictions: 0,
        }
    }
}

// Function to read the trace file and return lines as a vector of strings.
fn read_trace_file(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file at the given path.
    let file = File::open(Path::new(file_path))?;

    // Create a buffered reader for efficiently reading lines.
    let buf = io::BufReader::new(file);

    // Collect lines into a vector
    buf.lines().collect()
}

pub fn main() {

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    let s = &args[1].parse().unwrap();
    let e = &args[2].parse().unwrap();
    let b = &args[2].parse().unwrap();
    let tracefile = &args[4];

    println!("s: {}, E: {}, b: {}, tracefile: {}", s, e, b, tracefile);

    // Call `read_trace_file` with the path to our sample file.
    match read_trace_file(tracefile) {
        Ok(lines) => {
            // If successful, iterate over the lines and print each one.
            for line in lines {
                println!("{}", line);
            }
        },
        Err(e) => {
            // If there's an error, print it out.
            println!("Error reading file: {}", e);
        }
    }

    // initialize a new cache to see it's working
    let cache = Cache::new(*s, *e, *b);

    // For testing, let's just print the number of sets to verify our cache is initialized correctly.
    println!("Initialized cache with {} sets.", cache.sets.len());

}