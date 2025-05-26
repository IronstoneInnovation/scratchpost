use std::env;
use crate::simple_cache;

fn max_items() -> usize {
    match env::var("SCRATCHPOST_MAX_ITEMS") {
        Ok(str_value) => {
            match str_value.parse::<usize>() {
                Ok(value) => value,
                Err(_) => simple_cache::DEFAULT_MAX_ITEMS
            }
        },
        Err(_) => simple_cache::DEFAULT_MAX_ITEMS
    }
}

#[test]
fn test_max_items() {
    let mut cache = simple_cache::new();
    for i in 1..=max_items() {
        cache.push(format!("testkey{}", i), format!("testvalue{}", i));
        let result =  cache.get(format!("testkey{}", i));
        assert_eq!(result, format!("testvalue{}", i));
    }
    assert_eq!(cache.size(), max_items());
    cache.push("anotherkey".to_string(), "anothertestvalue".to_string());
    assert_eq!(cache.size(), max_items());
    assert_eq!(cache.get("testvalue1".to_string()), "");
}