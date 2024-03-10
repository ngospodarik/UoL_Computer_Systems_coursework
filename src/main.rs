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

// Function to get operation type and address from trace line
fn parse_trace_line(line: &str) -> Option<(char, u64)> {
    // Check if the first character of the line indicates an instruction load ('I').
    // If so, we ignore these lines by returning None.
    if line.starts_with('I') {
        return None;
    }

    // Split the line into parts using whitespace as the delimiter.
    // This separates the operation type ('L', 'S', 'M') from the memory address.
    let parts: Vec<&str> = line.split_whitespace().collect();

    // Extract the operation type, which is the first character of the first part.
    let operation = parts[0].chars().next().unwrap();

    // Separate address from size and other data which may appear
    let addr_size: Vec<&str> = parts[1].split(',').collect();

    // Extract the memory address from the second part.
    let address = u64::from_str_radix(addr_size[0], 16).unwrap();

    // Return the operation and address
    Some((operation, address))
}

// Function to calculate the set index and tag from an address
fn calculate_index_and_tag(address: u64, s: usize, b: usize) -> (usize, u64) {
    // Shift the address right by 'b' bits to discard the block offset bits,
    // then mask it with (2^s - 1) to keep only the 's' bits used for the set index.
    let set_index = (address >> b) & ((1 << s) - 1);

    // Shift the address right by 's+b' bits to discard both the set index and block offset bits,
    // leaving only the tag bits.
    let tag = address >> (s + b);

    // Return the set index and tag as a tuple. The set index is cast to usize for indexing purposes,
    // and the tag is kept as u64, its original type.
    (set_index as usize, tag)
}

pub fn main() {

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    let s = &args[1].parse().unwrap();
    let e = &args[2].parse().unwrap();
    let b = &args[2].parse().unwrap();
    let tracefile = &args[4];

    println!("s: {}, E: {}, b: {}, tracefile: {}", s, e, b, tracefile);

    // Use the `read_trace_file` function to read the trace file
    let lines = match read_trace_file(tracefile) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Failed to read trace file: {}", e);
            return;
        }
    };

    // Process and parse each line from the trace file
    for (index, line) in lines.iter().enumerate() {
        let parse_result = parse_trace_line(line);

        // Print the line and its parse result for verification
        println!("Line {}: {}", index + 1, line);
        match parse_result {
            Some((operation, address)) => {
                println!("  Parsed: Operation '{}', Address '{:X}'", operation, address);

                // Once we have the address, calculate the set index and tag
                let (set_index, tag) = calculate_index_and_tag(address, *s, *b);

                // Print the calculated set index and tag for further verification
                println!("    Calculated Set Index: {}, Tag: '{:X}'", set_index, tag);
            },
            None => println!("  Parsed: Ignored or Invalid Format"),
        }
    }

    // initialize a new cache to see it's working
    let cache = Cache::new(*s, *e, *b);

    // For testing, let's just print the number of sets to verify our cache is initialized correctly.
    println!("Initialized cache with {} sets.", cache.sets.len());

}