use Rust::*;

#[test]
fn test_parse_valid_load_operation() {
    let line = " L 04f6b868,8";
    assert_eq!(parse_trace_line(line), Some(('L', 0x04f6b868)));
}

#[test]
fn test_parse_valid_store_operation() {
    let line = " S 7ff0005c8,8";
    assert_eq!(parse_trace_line(line), Some(('S', 0x7ff0005c8)));
}

#[test]
fn test_parse_instruction_load() {
    let invalid_line = "I 12345678,4"; // For lines starting with 'I' None is returned
    assert_eq!(parse_trace_line(invalid_line), None);
}

#[test]
fn test_calculate_index_and_tag_normal() {
    let address = 0x12345678;
    let s = 4;
    let b = 5;
    let expected_set_index = 3;
    let expected_tag = 596523;

    assert_eq!(calculate_index_and_tag(address, s, b), (expected_set_index, expected_tag));
}

#[test]
fn test_calculate_index_and_tag_changed_s_b() {
    let address = 0x12345678;
    let s = 3;
    let b = 4;
    let expected_set_index = 7;
    let expected_tag = 2386092;

    assert_eq!(calculate_index_and_tag(address, s, b), (expected_set_index, expected_tag));
}

#[test]
fn test_calculate_index_and_tag_zero_s_b() {
    let address = 0x12345678;
    let s = 0; // Zero set index bits
    let b = 0; // Zero block offset bits
    // When s and b are zero, expect the set index to be 0 and tag to be the full address
    let expected_set_index = 0; // Expected set index
    let expected_tag = 0x12345678; // Expected tag to be the full address

    assert_eq!(calculate_index_and_tag(address, s, b), (expected_set_index, expected_tag));
}

#[test]
fn cache_line_hit() {
    let line = CacheLine { tag: 0x1234, valid: true, last_used: 0 };
    assert!(line.is_hit(0x1234), "CacheLine should hit when tags match and line is valid.");
}

#[test]
fn cache_line_miss_due_to_tag_mismatch() {
    let line = CacheLine { tag: 0x1234, valid: true, last_used: 0 };
    assert!(!line.is_hit(0x5678), "CacheLine should miss when tags don't match.");
}

#[test]
fn cache_line_miss_due_to_invalid() {
    let line = CacheLine { tag: 0x1234, valid: false, last_used: 0 };
    assert!(!line.is_hit(0x1234), "CacheLine should miss when line is invalid, even if tags match.");
}

#[test]
fn cache_set_hit() {
    let mut set = CacheSet {
        lines: vec![
            CacheLine { tag: 0x1234, valid: true, last_used: 1 },
            CacheLine { tag: 0x0, valid: false, last_used: 0 }, // Empty line available for use
        ],
    };
    let (is_hit, is_eviction) = set.access(0x1234, &mut 2);
    assert!(is_hit, "Access should result in a hit");
    assert!(!is_eviction, "There should be no eviction on a hit");
}

#[test]
fn cache_set_miss_no_eviction() {
    let mut set = CacheSet {
        lines: vec![
            CacheLine { tag: 0x1234, valid: true, last_used: 1 },
            CacheLine { tag: 0x0, valid: false, last_used: 0 }, // Empty line available for use
        ],
    };
    let (is_hit, is_eviction) = set.access(0x5678, &mut 2);
    assert!(!is_hit, "Access should be a miss");
    assert!(!is_eviction, "No eviction should occur when there is available space");
}

#[test]
fn cache_set_miss_with_eviction() {
    let mut set = CacheSet {
        lines: vec![
            CacheLine { tag: 0x1234, valid: true, last_used: 1 },
            CacheLine { tag: 0x5678, valid: true, last_used: 2 }, // No empty lines, set is full
        ],
    };
    let (is_hit, is_eviction) = set.access(0x9abc, &mut 3); // Accessing a new tag, forcing eviction
    assert!(!is_hit, "Access should result in a miss");
    assert!(is_eviction, "Eviction should occur when the set is full");
}

#[test]
fn process_ibm_trace() {

    let s = 4;
    let e = 1;
    let b = 4;

    let mut cache = Cache::new(s, e);

    let lines = vec![
        " L 10,4".to_string(),
        " S 18,4".to_string(),
        " L 20,4".to_string(),
        " S 28,4".to_string(),
        " S 50,4".to_string(),
    ];

    hits_misses_evictions_calc(&mut cache, lines, s, b);

    assert_eq!(cache.hits, 2);
    assert_eq!(cache.misses, 3);
    assert_eq!(cache.evictions, 0);
}

#[test]
fn process_trace_with_various_operations() {

    let s = 4;
    let e = 1;
    let b = 4;

    let mut cache = Cache::new(s, e);

    let lines = vec![
        " S 00600aa0,1".to_string(),
        "I  004005b6,5".to_string(),
        " L 7ff000388,4".to_string(),
        " M 7ff000388,4".to_string(),
    ];

    hits_misses_evictions_calc(&mut cache, lines, s, b);

    assert_eq!(cache.hits, 2);
    assert_eq!(cache.misses, 2);
    assert_eq!(cache.evictions, 0);
}

#[test]
fn process_trace_with_evictions() {

    let mut cache = Cache::new(2, 1);

    let s = 1;
    let b = 5;

    let lines = vec![
        " L 04022d00,8".to_string(),
        " L 04022e00,8".to_string(),
    ];

    hits_misses_evictions_calc(&mut cache, lines, s, b);

    assert_eq!(cache.hits, 0);
    assert_eq!(cache.misses, 2);
    assert_eq!(cache.evictions, 1);
}