/* Copyright 2013 10gen Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::hash::Hash;
use collections::hashmap::*;
use std::container;
use std::fmt;
use std::slice;
use std::vec;

///A hashmap which maintains iteration order using a list.
pub struct OrderedHashmap<K,V> {
    map: HashMap<K,V>,
    order: vec::Vec<(K,V)>
}

impl<K:Clone + Hash + TotalEq + Copy, V:Clone + Copy> Clone for OrderedHashmap<K,V> {
    fn clone(&self) -> OrderedHashmap<K,V> {
        let mut m: HashMap<K,V> = HashMap::new();
        for &(k, v) in self.iter() {
            m.insert(k.clone(), v.clone());
        }
        OrderedHashmap {
            map: m,
            order: self.order.clone()
        }
    }
}

impl<K: Hash + TotalEq,V> container::Container for OrderedHashmap<K,V> {
    fn len(&self) -> uint { self.map.len() }
    fn is_empty(&self) -> bool { self.map.is_empty() }
}

impl<K: Hash + TotalEq,V> Mutable for OrderedHashmap<K,V> {
    fn clear(&mut self) {
        self.map = HashMap::new();
        self.order = vec::Vec::new();
    }
}

impl<K:Hash + TotalEq,V: Eq> Eq for OrderedHashmap<K,V> {
    fn eq(&self, other: &OrderedHashmap<K,V>) -> bool {
        self.map == other.map && self.order == other.order
    }
    fn ne(&self, other: &OrderedHashmap<K,V>) -> bool {
        self.map != other.map || self.order != other.order
    }
}

///Expose most of the Hashmap implementation.
impl<K: Hash + TotalEq + Copy,V: Copy> OrderedHashmap<K,V> {
    pub fn len(&self) -> uint { self.map.len() }
    pub fn contains_key(&self, k: &K) -> bool { self.map.contains_key(k) }
    pub fn iter<'a>(&'a self) -> slice::Items<'a, (K, V)> {
        self.order.iter()
    }
    pub fn rev_iter<'a>(&'a self) -> slice::RevItems<'a, (K, V)> {
        self.order.as_slice().rev_iter()
    }
    pub fn find<'a>(&'a self, k: &K) -> Option<&'a V> {
        self.map.find(k)
    }
    pub fn find_mut<'a>(&'a mut self, k: &K) -> Option<&'a mut V> {
        self.map.find_mut(k)
    }
    pub fn insert(&mut self, k: K, v: V) -> bool {
        let success = self.map.insert(k, v);
        if success { self.order.push((k, v)) }
        success
    }

    pub fn new() -> OrderedHashmap<K,V> {
        OrderedHashmap { map: HashMap::new(), order: vec::Vec::new() }
    }
}

impl<K:Hash + TotalEq + ToStr + Copy,V:ToStr + Copy> fmt::Show for OrderedHashmap<K,V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = ~"{";
        for &(k, v) in self.iter() {
            s.push_str(format!(" {}: {}, ", k.to_str(), v.to_str()));
        }
        s = s.slice(0, s.len()-2).to_owned();
        s.push_str("}");
        write!(f.buf, "{}", s)
    }
}
