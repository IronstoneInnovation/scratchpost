


fn main() {

    let mut s = scratchpost::new_simple_cache();

    s.push("hello".to_string(), "world!".to_string());
    s.get("hello".to_string());


}