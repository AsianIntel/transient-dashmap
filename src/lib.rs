use dashmap::{DashMap, mapref::one::{Ref, RefMut}};
use std::hash::Hash;
use std::time::{Instant, Duration};
use std::sync::Arc;

pub struct Item<T> {
    pub object: T,
    pub expiration: Instant
}

#[derive(Clone)]
pub struct TransientDashMap<K, V> {
    map: Arc<DashMap<K, Item<V>>>,
    duration: Duration
}

impl<T> Item<T> {
    pub fn new(object: T, expiration: Instant) -> Self {
        Item {
            object,
            expiration
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expiration
    }
}

impl<K: Hash + Eq, V> TransientDashMap<K, V> {
    pub fn new(duration: Duration) -> Self {
        TransientDashMap {
            map: Arc::new(DashMap::new()),
            duration
        }
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.map.insert(key, Item::new(value, Instant::now() + self.duration)).map(|v| v.object)
    }

    pub fn insert_with_expiration(&self, key: K, value: V, expiration: Instant) -> Option<V> {
        self.map.insert(key, Item::new(value, expiration)).map(|v| v.object)
    }

    pub fn get(&self, key: &K) -> Option<Ref<'_, K, Item<V>>> {
        match self.map.get(key) {
            Some(k) => {
                if k.is_expired() {
                    self.map.remove(key);
                    return None
                }
                Some(k)
            },
            None => None
        }
    }

    pub fn get_mut(&self, key: &K) -> Option<RefMut<'_, K, Item<V>>> {
        match self.map.get_mut(key) {
            Some(k) => {
                if k.is_expired() {
                    self.map.remove(key);
                    return None
                }
                Some(k)
            },
            None => None
        }
    }

    pub fn remove(&self, key: &K) -> Option<(K, V)> {
        self.map.remove(key).map(|(k, v)| (k, v.object))
    }
}