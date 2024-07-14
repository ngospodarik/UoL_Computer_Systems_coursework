use std::{io};
use std::io::{BufRead};
use std::fs::File;
use std::path::Path;


// Define the CacheLine struct. Each CacheLine represents a single cache line.
pub struct CacheLine {
    pub valid: bool, // Indicates if the line is valid (contains data).
    pub tag: u64,    // The tag part of the address stored in this cache line.
    pub last_used: u64, // Used for implementing LRU; higher number means more recently used.
}

impl CacheLine {
    // Method to check if a given tag matches the tag of this cache line
    // and if this cache line is valid (in use).
    pub fn is_hit(&self, tag: u64) -> bool {
        self.valid && self.tag == tag
    }
}

// Define the CacheSet struct. Each CacheSet contains multiple CacheLines.
pub struct CacheSet {
    pub lines: Vec<CacheLine>, // A vector of CacheLine, represents all lines within a set.
}

impl CacheSet {
    // Method to access a cache set with a given tag.
    // It returns a tuple indicating whether the access was a hit and if an eviction occurred.
    pub fn access(&mut self, tag: u64, current_time: &mut u64) -> (bool, bool) {
        // First, try to find a line that results in a hit.
        if let Some(line) = self.lines.iter_mut().find(|line| line.is_hit(tag)) {
            // If a hit is found, update the last_used to the current time and return (hit, no eviction).
            line.last_used = *current_time;
            return (true, false);
        }

        // No hit found; this is a miss. Increment current time for LRU logic.
        *current_time += 1;

        // Find the least recently used line for potential eviction.
        if let Some(least_used_line) = self.lines.iter_mut().min_by_key(|line| line.last_used) {
            // Check if we need to evict a line (if all lines are valid).
            let eviction = least_used_line.valid;
            // Update the least recently used line with the new tag and current time.
            least_used_line.valid = true;
            least_used_line.tag = tag;
            least_used_line.last_used = *current_time;

            // Return (miss, eviction status).
            return (false, eviction);
        }

        // Default return should never be reached if cache set is initialized correctly.
        unreachable!("Cache set must have at least one line");
    }
}

// Define the Cache struct with its properties.
pub struct Cache {
    pub sets: Vec<CacheSet>, // A vector of CacheSet, represents all the sets in the cache.
    pub hits: u64,           // A counter for the number of cache hits.
    pub misses: u64,         // A counter for the number of cache misses.
    pub evictions: u64,      // A counter for the number of cache evictions.
}

impl Cache {
    // Define a method to create a new Cache instance.
    pub fn new(s: usize, e: usize) -> Cache {
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
    // Method to report the simulation results
    pub fn report_simulation_results(&self) {
        println!("hits:{} misses:{} evictions:{}", self.hits, self.misses, self.evictions);
    }
}

// Function to read the trace file and return lines as a vector of strings.
pub fn read_trace_file(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file at the given path.
    let file = File::open(Path::new(file_path))?;

    // Create a buffered reader for efficiently reading lines.
    let buf = io::BufReader::new(file);

    // Collect lines into a vector
    buf.lines().collect()
}

// Function to get operation type and address from trace line
pub fn parse_trace_line(line: &str) -> Option<(char, u64)> {
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
pub fn calculate_index_and_tag(address: u64, s: usize, b: usize) -> (usize, u64) {
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

pub fn hits_misses_evictions_calc(cache: &mut Cache, lines: Vec<String>, s: usize, b: usize) {
    let mut current_time = 0_u64;

    for line in lines {
        if let Some((operation, address)) = parse_trace_line(&line) {
            let (set_index, tag) = calculate_index_and_tag(address, s, b);
            let set = &mut cache.sets[set_index];

            // Perform the access and update cache metrics based on the result.
            let (hit, eviction) = set.access(tag, &mut current_time);

            // Update cache statistics based on access result.
            if hit {
                cache.hits += 1;
            } else {
                cache.misses += 1;
                if eviction {
                    cache.evictions += 1;
                }
            }

            // For 'M' operations, which are a load followed by a store, simulate a hit for the store operation.
            if operation == 'M' {
                cache.hits += 1; // Store hit
            }
        }

    }
}