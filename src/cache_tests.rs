use crate::simple_cache;



#[test]
/// Test that the cache never has more than whatever max_items is set by either
/// the SCRATCHPOST_MAX_ITEMS env var or the default value. 
fn test_max_items() {
    // First, fill the cache to the maximum number of items and check their values
    // are set correctly.
    let mut cache = simple_cache::new();
    for i in 1..=simple_cache::get_max_items() {
        cache.push(format!("testkey{}", i), format!("testvalue{}", i));
        let result =  cache.get(format!("testkey{}", i));
        assert_eq!(result, format!("testvalue{}", i));
    }

    // Ensure the number of items is at the maximum
    assert_eq!(cache.size(), simple_cache::get_max_items());

    // Push one more item and ensure the cache is still at maximum size
    cache.push("anotherkey".to_string(), "anothertestvalue".to_string());
    assert_eq!(cache.size(), simple_cache::get_max_items());

    // Then ensure that the first value to be pushed has expired and no longer exists
    assert_eq!(cache.get("testvalue1".to_string()), "");
}