use crate::simple_cache;



#[test]
fn test_max_items() {

    let mut cache = simple_cache::new();
    for i in 1..=simple_cache::get_max_items() {
        cache.push(format!("testkey{}", i), format!("testvalue{}", i));
        let result =  cache.get(format!("testkey{}", i));
        assert_eq!(result, format!("testvalue{}", i));
    }
    assert_eq!(cache.size(), simple_cache::get_max_items());
    cache.push("anotherkey".to_string(), "anothertestvalue".to_string());
    assert_eq!(cache.size(), simple_cache::get_max_items());
    assert_eq!(cache.get("testvalue1".to_string()), "");
}