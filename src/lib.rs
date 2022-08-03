pub(crate) use egg::*;

mod sketch;
mod analysis;
mod extract;

pub use {
    sketch::Sketch
};

pub(crate) type BuildHasher = fxhash::FxBuildHasher;

pub(crate) type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasher>;
pub(crate) type HashSet<K> = hashbrown::HashSet<K, BuildHasher>;

pub(crate) type IndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasher>;
pub(crate) type IndexSet<K> = indexmap::IndexSet<K, BuildHasher>;