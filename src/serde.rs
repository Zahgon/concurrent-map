use std::marker::PhantomData;

use serde::de::{Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize, Serializer};

use crate::{ConcurrentMap, Minimum};

impl<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize> Serialize
    for ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + Serialize + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Serialize + Clone + Send + Sync,
{
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    { panic!("STUB: not implemented") }
}

struct ConcurrentMapVisitor<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize> {
    pd: PhantomData<(K, V)>,
}

impl<'de, K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize> Visitor<'de>
    for ConcurrentMapVisitor<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + Deserialize<'de> + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Deserialize<'de> + Clone + Send + Sync,
{
    type Value = ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result { panic!("STUB: not implemented") }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    { panic!("STUB: not implemented") }
}

impl<'de, K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize> Deserialize<'de>
    for ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + Deserialize<'de> + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Deserialize<'de> + Clone + Send + Sync,
{
    fn deserialize<D>(d: D) -> Result<ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>, D::Error>
    where
        D: Deserializer<'de>,
    { panic!("STUB: not implemented") }
}
