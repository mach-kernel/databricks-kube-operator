use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use databricks_kube::util::hash_json_value;

#[test]
fn test_hash() {
    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    let fuzzy_json: Vec<Value> = vec![
        serde_json::from_str(include_str!("fixtures/random-1.json")).unwrap(),
        serde_json::from_str(include_str!("fixtures/random-2.json")).unwrap(),
    ];

    for j in fuzzy_json {
        hash_json_value(&mut hasher_a, &j);
        hash_json_value(&mut hasher_b, &j);

        assert_eq!(hasher_a.finish(), hasher_b.finish());

        hasher_a = DefaultHasher::new();
        hasher_b = DefaultHasher::new();
    }
}
