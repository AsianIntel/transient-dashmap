use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use std::{
    hash::Hash,
    ops::Deref,
    time::{Duration, Instant},
};

pub use dashmap::mapref::entry::*;

pub struct Data<T> {
    object: T,
    expiration: Instant,
}

pub struct TransientDashMap<K, V> {
    map: DashMap<K, Data<V>>,
    duration: Duration,
}

impl<T> Data<T> {
    pub fn new(object: T, expiration: Instant) -> Self {
        Data { object, expiration }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expiration
    }
}

impl<K: Hash + Eq + Clone, V> TransientDashMap<K, V> {
    pub fn new(duration: Duration) -> Self {
        TransientDashMap {
            map: DashMap::new(),
            duration,
        }
    }

    pub fn insert(&self, key: K, value: V) -> Option<Data<V>> {
        self.map
            .insert(key, Data::new(value, Instant::now() + self.duration))
    }

    pub fn insert_with_expiration(&self, key: K, value: V, expiration: Instant) -> Option<Data<V>> {
        self.map.insert(key, Data::new(value, expiration))
    }

    pub fn get(&self, key: &K) -> Option<Ref<'_, K, Data<V>>> {
        match self.map.get(key) {
            Some(k) => {
                if !k.is_expired() {
                    return Some(k);
                }
            }
            None => return None,
        }
        self.map.remove(key);
        None
    }

    pub fn get_mut(&self, key: &K) -> Option<RefMut<'_, K, Data<V>>> {
        match self.map.get_mut(key) {
            Some(k) => {
                if !k.is_expired() {
                    return Some(k);
                }
            }
            None => return None,
        }
        self.map.remove(key);
        None
    }

    pub fn remove(&self, key: &K) -> Option<(K, V)> {
        self.map.remove(key).map(|(k, v)| (k, v.object))
    }

    pub fn entry(&self, key: K) -> Entry<'_, K, Data<V>> {
        self.map.entry(key)
    }

    pub fn purge(&self) {
        let items = self.map.iter().map(|r| r.key().clone());
        for item in items {
            let _ = self.map.get(&item);
        }
    }
}

impl<T> Deref for Data<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
