pub mod dlist;
pub mod arr;

#[test]
fn t1() {
    use dlist::LRUCache;

    fn assert_cache(cache: &LRUCache, expected: &[(i32, i32)]) {
        assert_eq!(&cache.iter().collect::<Vec<_>>(), expected);
    }

    let mut cache = LRUCache::new(2);
    assert_eq!(cache.len, 0);

    cache.put(1, 10);
    assert_cache(&cache, &[(1, 10)]);
    assert_eq!(cache.len, 1);

    cache.put(2, 20);
    assert_cache(&cache, &[(2, 20), (1, 10)]);
    assert_eq!(cache.len, 2);

    assert_eq!(cache.get(1), 10);
    assert_cache(&cache, &[(1, 10), (2, 20)]);
    assert_eq!(cache.len, 2);

    cache.put(3, 30);
    assert_cache(&cache, &[(3, 30), (1, 10)]);
    assert_eq!(cache.len, 2);

    assert_eq!(cache.get(2), -1);
    assert_cache(&cache, &[(3, 30), (1, 10)]);
    assert_eq!(cache.len, 2);

    cache.put(4, 40);
    assert_cache(&cache, &[(4, 40), (3, 30)]);
    assert_eq!(cache.len, 2);

    assert_eq!(cache.get(1), -1);
    assert_cache(&cache, &[(4, 40), (3, 30)]);
    assert_eq!(cache.len, 2);

    assert_eq!(cache.get(3), 30);
    assert_cache(&cache, &[(3, 30), (4, 40)]);
    assert_eq!(cache.len, 2);

    assert_eq!(cache.get(4), 40);
    assert_cache(&cache, &[(4, 40), (3, 30)]);
    assert_eq!(cache.len, 2);
}

#[test]
fn t2() {
    use dlist::LRUCache;

    fn assert_cache(cache: &LRUCache, expected: &[(i32, i32)]) {
        assert_eq!(&cache.iter().collect::<Vec<_>>(), expected);
    }

    let mut cache = LRUCache::new(3);
    assert_eq!(cache.len, 0);

    cache.put(1, 10);
    assert_cache(&cache, &[(1, 10)]);
    assert_eq!(cache.len, 1);

    cache.put(2, 20);
    assert_cache(&cache, &[(2, 20), (1, 10)]);
    assert_eq!(cache.len, 2);

    cache.put(3, 30);
    assert_cache(&cache, &[(3, 30), (2, 20), (1, 10)]);
    assert_eq!(cache.len, 3);

    cache.put(4, 40);
    assert_cache(&cache, &[(4, 40), (3, 30), (2, 20)]);
    assert_eq!(cache.len, 3);

    assert_eq!(cache.get(4), 40);
    assert_cache(&cache, &[(4, 40), (3, 30), (2, 20)]);

    assert_eq!(cache.get(3), 30);
    assert_cache(&cache, &[(3, 30), (4, 40), (2, 20)]);

    assert_eq!(cache.get(2), 20);
    assert_cache(&cache, &[(2, 20), (3, 30), (4, 40)]);

    assert_eq!(cache.get(1), -1);
    assert_cache(&cache, &[(2, 20), (3, 30), (4, 40)]);

    cache.put(5, 50);
    assert_cache(&cache, &[(5, 50), (2, 20), (3, 30)]);
}

