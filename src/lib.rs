#![cfg_attr(
    test,
    deny(
        missing_docs,
        future_incompatible,
        nonstandard_style,
        rust_2018_idioms,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_qualifications,
    )
)]
#![cfg_attr(test, deny(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::decimal_literal_representation,
    clippy::doc_markdown,
    
    clippy::empty_enum,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::expl_impl_clone_on_copy,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_arithmetic,
    clippy::get_unwrap,
    clippy::if_not_else,
    clippy::indexing_slicing,
    clippy::inline_always,
    clippy::integer_arithmetic,
    clippy::invalid_upcast_comparisons,
    clippy::items_after_statements,
    clippy::manual_find_map,
    clippy::map_entry,
    clippy::map_flatten,
    clippy::match_like_matches_macro,
    clippy::match_same_arms,
    clippy::maybe_infinite_iter,
    clippy::mem_forget,
    
    clippy::module_name_repetitions,
    clippy::multiple_inherent_impl,
    clippy::mut_mut,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::non_ascii_literal,
    clippy::path_buf_push_overwrite,
    
    clippy::redundant_closure_for_method_calls,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::single_match_else,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::type_repetition_in_bounds,
    clippy::unicode_not_nfc,
    clippy::unimplemented,
    clippy::unseparated_literal_suffix,
    clippy::used_underscore_binding,
    clippy::wildcard_dependencies,
))]
#![cfg_attr(
    test,
    warn(
        clippy::missing_const_for_fn,
        clippy::multiple_crate_versions,
        clippy::wildcard_enum_match_arm,
    )
)]

#[cfg(feature = "serde")]
mod serde;

#[cfg(not(feature = "fault_injection"))]
#[inline]
const fn debug_delay() -> bool {
    false
}

#[cfg(feature = "fault_injection")]
fn debug_delay() -> bool { panic!("STUB: not implemented") }

use stack_map::StackMap;

use std::borrow::Borrow;
use std::fmt;
use std::mem::size_of;
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use std::ops::{Bound, Deref};
use std::ptr::NonNull;
use std::sync::{
    atomic::{AtomicPtr, AtomicUsize, Ordering},
    Arc,
};

#[cfg(feature = "timing")]
use std::time::{Duration, Instant};

use ebr::{Ebr, Guard};

const MERGE_SIZE: usize = 1;

#[derive(Debug)]
enum Deferred<
    K: 'static + Clone + Minimum + Send + Sync + Ord,
    V: 'static + Clone + Send + Sync,
    const FANOUT: usize,
> {
    #[allow(unused)]
    Node(Box<Node<K, V, FANOUT>>),
    BoxedAtomicPtr(BoxedAtomicPtr<K, V, FANOUT>),
}

impl<
        K: 'static + Clone + Minimum + Send + Sync + Ord,
        V: 'static + Clone + Send + Sync,
        const FANOUT: usize,
    > Drop for Deferred<K, V, FANOUT>
{
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone, Eq)]
struct BoxedAtomicPtr<
    K: 'static + Clone + Minimum + Send + Sync + Ord,
    V: 'static + Clone + Send + Sync,
    const FANOUT: usize,
>(*const AtomicPtr<Node<K, V, FANOUT>>);

impl<
        K: 'static + Clone + Minimum + Send + Sync + Ord,
        V: 'static + Clone + Send + Sync,
        const FANOUT: usize,
    > Copy for BoxedAtomicPtr<K, V, FANOUT>
{
}

impl<
        K: 'static + Clone + Minimum + Send + Sync + Ord,
        V: 'static + Clone + Send + Sync,
        const FANOUT: usize,
    > PartialEq for BoxedAtomicPtr<K, V, FANOUT>
{
    fn eq(&self, other: &Self) -> bool { panic!("STUB: not implemented") }
}

unsafe impl<
        K: 'static + Clone + Minimum + Send + Sync + Ord,
        V: 'static + Clone + Send + Sync,
        const FANOUT: usize,
    > Send for BoxedAtomicPtr<K, V, FANOUT>
{
}

unsafe impl<
        K: 'static + Clone + Minimum + Send + Sync + Ord,
        V: 'static + Clone + Send + Sync,
        const FANOUT: usize,
    > Sync for BoxedAtomicPtr<K, V, FANOUT>
{
}

impl<
        K: 'static + Clone + Minimum + Send + Sync + Ord,
        V: 'static + Clone + Send + Sync,
        const FANOUT: usize,
    > Deref for BoxedAtomicPtr<K, V, FANOUT>
{
    type Target = AtomicPtr<Node<K, V, FANOUT>>;

    fn deref(&self) -> &AtomicPtr<Node<K, V, FANOUT>> { panic!("STUB: not implemented") }
}

impl<
        K: 'static + Clone + Minimum + Send + Sync + Ord,
        V: 'static + Clone + Send + Sync,
        const FANOUT: usize,
    > BoxedAtomicPtr<K, V, FANOUT>
{
    fn new(node: Box<Node<K, V, FANOUT>>) -> BoxedAtomicPtr<K, V, FANOUT> { panic!("STUB: not implemented") }

    fn node_view<const LOCAL_GC_BUFFER_SIZE: usize>(
        &self,
        _guard: &mut Guard<'_, Deferred<K, V, FANOUT>, LOCAL_GC_BUFFER_SIZE>,
    ) -> Option<NodeView<K, V, FANOUT>> { panic!("STUB: not implemented") }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CasFailure<V> {
    
    pub actual: Option<V>,
    
    pub returned_new_value: Option<V>,
}

#[derive(Debug)]
struct NodeView<K, V, const FANOUT: usize>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    ptr: NonNull<Node<K, V, FANOUT>>,
    id: BoxedAtomicPtr<K, V, FANOUT>,
}

impl<K, V, const FANOUT: usize> NodeView<K, V, FANOUT>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    
    fn cas<const LOCAL_GC_BUFFER_SIZE: usize>(
        &self,
        replacement: Box<Node<K, V, FANOUT>>,
        guard: &mut Guard<'_, Deferred<K, V, FANOUT>, LOCAL_GC_BUFFER_SIZE>,
    ) -> Result<NodeView<K, V, FANOUT>, Option<NodeView<K, V, FANOUT>>> { panic!("STUB: not implemented") }

    unsafe fn get_mut(&mut self) -> &mut Node<K, V, FANOUT> { panic!("STUB: not implemented") }
}

impl<K, V, const FANOUT: usize> Deref for NodeView<K, V, FANOUT>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    type Target = Node<K, V, FANOUT>;

    fn deref(&self) -> &Self::Target { panic!("STUB: not implemented") }
}

pub trait Minimum: Ord {
    
    const MIN: Self;
}

pub trait Maximum: Ord {
    
    const MAX: Self;
}

impl Minimum for () {
    const MIN: Self = ();
}

impl Minimum for bool {
    const MIN: Self = false;
}

impl<T: Maximum> Minimum for std::cmp::Reverse<T> {
    const MIN: Self = std::cmp::Reverse(T::MAX);
}

macro_rules! impl_integer {
    ($($t:ty),+) => {
        $(
            impl Minimum for $t {
                const MIN: Self = <$t>::MIN;
            }

            impl Maximum for $t {
                const MAX: Self = <$t>::MAX;
            }
        )*
    }
}

impl_integer!(
    usize,
    u8,
    u16,
    u32,
    u64,
    u128,
    isize,
    i8,
    i16,
    i32,
    i64,
    i128,
    NonZeroI128,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroI8,
    NonZeroIsize,
    NonZeroU128,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU8,
    NonZeroUsize
);

impl<T: Ord> Minimum for Vec<T> {
    const MIN: Self = Vec::new();
}

impl<T: Ord> Minimum for &[T] {
    const MIN: Self = &[];
}

impl<T: Minimum, const LEN: usize> Minimum for [T; LEN] {
    const MIN: Self = [T::MIN; LEN];
}

impl Minimum for String {
    const MIN: Self = String::new();
}

impl Minimum for &str {
    const MIN: Self = "";
}

impl<A: Minimum, B: Minimum> Minimum for (A, B) {
    const MIN: Self = (A::MIN, B::MIN);
}
impl<A: Minimum, B: Minimum, C: Minimum> Minimum for (A, B, C) {
    const MIN: Self = (A::MIN, B::MIN, C::MIN);
}
impl<A: Minimum, B: Minimum, C: Minimum, D: Minimum> Minimum for (A, B, C, D) {
    const MIN: Self = (A::MIN, B::MIN, C::MIN, D::MIN);
}
impl<A: Minimum, B: Minimum, C: Minimum, D: Minimum, E: Minimum> Minimum for (A, B, C, D, E) {
    const MIN: Self = (A::MIN, B::MIN, C::MIN, D::MIN, E::MIN);
}
impl<A: Minimum, B: Minimum, C: Minimum, D: Minimum, E: Minimum, F: Minimum> Minimum
    for (A, B, C, D, E, F)
{
    const MIN: Self = (A::MIN, B::MIN, C::MIN, D::MIN, E::MIN, F::MIN);
}

#[derive(Clone)]
pub struct ConcurrentMap<K, V, const FANOUT: usize = 64, const LOCAL_GC_BUFFER_SIZE: usize = 128>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    
    ebr: Ebr<Deferred<K, V, FANOUT>, LOCAL_GC_BUFFER_SIZE>,
    
    inner: Arc<Inner<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>>,
    
    len: Arc<AtomicUsize>,
}

impl<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize> PartialEq
    for ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + fmt::Debug + Clone + Minimum + Ord + Send + Sync + PartialEq,
    V: 'static + fmt::Debug + Clone + Send + Sync + PartialEq,
{
    fn eq(&self, other: &Self) -> bool { panic!("STUB: not implemented") }
}

impl<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize> fmt::Debug
    for ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + fmt::Debug + Clone + Minimum + Ord + Send + Sync,
    V: 'static + fmt::Debug + Clone + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize> Default
    for ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn default() -> ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE> { panic!("STUB: not implemented") }
}

struct Inner<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    root: BoxedAtomicPtr<K, V, FANOUT>,
    #[cfg(feature = "timing")]
    slowest_op: AtomicU64,
    #[cfg(feature = "timing")]
    fastest_op: AtomicU64,
}

impl<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize> Drop
    for Inner<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

impl<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize>
    ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    
    pub fn new() -> Self { panic!("STUB: not implemented") }

    pub fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    { panic!("STUB: not implemented") }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    { panic!("STUB: not implemented") }

    pub fn get_lt<Q>(&self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + PartialEq,
    { panic!("STUB: not implemented") }

    pub fn get_lte<Q>(&self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + PartialEq,
    { panic!("STUB: not implemented") }

    pub fn get_gt<Q>(&self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + PartialEq,
    { panic!("STUB: not implemented") }

    pub fn get_gte<Q>(&self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + PartialEq,
    { panic!("STUB: not implemented") }

    pub fn first(&self) -> Option<(K, V)> { panic!("STUB: not implemented") }

    pub fn pop_first(&self) -> Option<(K, V)>
    where
        V: PartialEq,
    { panic!("STUB: not implemented") }

    pub fn pop_first_in_range<Q, R>(&self, range: R) -> Option<(K, V)>
    where
        R: std::ops::RangeBounds<Q> + Clone,
        K: Borrow<Q>,
        V: PartialEq,
        Q: ?Sized + Ord + PartialEq,
    { panic!("STUB: not implemented") }

    pub fn last(&self) -> Option<(K, V)> { panic!("STUB: not implemented") }

    pub fn pop_last(&self) -> Option<(K, V)>
    where
        V: PartialEq,
    { panic!("STUB: not implemented") }

    pub fn pop_last_in_range<Q, R>(&self, range: R) -> Option<(K, V)>
    where
        R: std::ops::RangeBounds<Q> + Clone,
        K: Borrow<Q>,
        V: PartialEq,
        Q: ?Sized + Ord + PartialEq,
    { panic!("STUB: not implemented") }

    pub fn insert(&self, key: K, value: V) -> Option<V> { panic!("STUB: not implemented") }

    pub fn remove<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    { panic!("STUB: not implemented") }

    pub fn cas<VRef>(
        &self,
        key: K,
        old: Option<&VRef>,
        new: Option<V>,
    ) -> Result<Option<V>, CasFailure<V>>
    where
        V: Borrow<VRef>,
        VRef: PartialEq + ?Sized,
    { panic!("STUB: not implemented") }

    pub fn len(&self) -> usize { panic!("STUB: not implemented") }

    pub fn is_empty(&self) -> bool { panic!("STUB: not implemented") }

    pub fn iter(&self) -> Iter<'_, K, V, FANOUT, LOCAL_GC_BUFFER_SIZE> { panic!("STUB: not implemented") }

    pub fn range<Q, R>(&self, range: R) -> Iter<'_, K, V, FANOUT, LOCAL_GC_BUFFER_SIZE, R, Q>
    where
        R: std::ops::RangeBounds<Q>,
        K: Borrow<Q>,
        Q: ?Sized + Ord + PartialEq,
    { panic!("STUB: not implemented") }

    pub fn update_and_fetch<F>(&self, key: K, mut f: F) -> Option<V>
    where
        F: FnMut(Option<&V>) -> Option<V>,
        V: PartialEq,
    { panic!("STUB: not implemented") }

    pub fn fetch_and_update<F>(&self, key: K, mut f: F) -> Option<V>
    where
        F: FnMut(Option<&V>) -> Option<V>,
        V: PartialEq,
    { panic!("STUB: not implemented") }
}

impl<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize>
    ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync + Ord,
{
    
    pub fn fetch_min(&self, key: K, value: V) -> Option<V> { panic!("STUB: not implemented") }

    pub fn fetch_max(&self, key: K, value: V) -> Option<V> { panic!("STUB: not implemented") }
}

pub struct Iter<
    'a,
    K,
    V,
    const FANOUT: usize,
    const LOCAL_GC_BUFFER_SIZE: usize,
    R = std::ops::RangeFull,
    Q = K,
> where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
    R: std::ops::RangeBounds<Q>,
    K: Borrow<Q>,
    Q: ?Sized,
{
    inner: &'a Inner<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>,
    guard: Guard<'a, Deferred<K, V, FANOUT>, LOCAL_GC_BUFFER_SIZE>,
    range: R,
    current: NodeView<K, V, FANOUT>,
    next_index: usize,
    current_back: NodeView<K, V, FANOUT>,
    next_index_from_back: usize,
    q: std::marker::PhantomData<&'a Q>,
}

impl<'a, K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize, R, Q> Iterator
    for Iter<'a, K, V, FANOUT, LOCAL_GC_BUFFER_SIZE, R, Q>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
    R: std::ops::RangeBounds<Q>,
    K: Borrow<Q>,
    Q: ?Sized + PartialEq + Ord,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> { panic!("STUB: not implemented") }
}

impl<'a, K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize, R, Q> DoubleEndedIterator
    for Iter<'a, K, V, FANOUT, LOCAL_GC_BUFFER_SIZE, R, Q>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
    R: std::ops::RangeBounds<Q>,
    K: Borrow<Q>,
    Q: ?Sized + PartialEq + Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> { panic!("STUB: not implemented") }
}

enum LeafSearch<K> {
    
    Eq(K),
    
    Lt(K),
    Max,
}

impl<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize>
    Inner<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn root(
        &self,
        _guard: &mut Guard<'_, Deferred<K, V, FANOUT>, LOCAL_GC_BUFFER_SIZE>,
    ) -> NodeView<K, V, FANOUT> { panic!("STUB: not implemented") }

    fn install_parent_merge<'a>(
        &'a self,
        parent: &NodeView<K, V, FANOUT>,
        child: &NodeView<K, V, FANOUT>,
        guard: &mut Guard<'a, Deferred<K, V, FANOUT>, LOCAL_GC_BUFFER_SIZE>,
    ) -> Result<NodeView<K, V, FANOUT>, ()> { panic!("STUB: not implemented") }

    fn merge_child<'a>(
        &'a self,
        parent: &mut NodeView<K, V, FANOUT>,
        child: &mut NodeView<K, V, FANOUT>,
        guard: &mut Guard<'a, Deferred<K, V, FANOUT>, LOCAL_GC_BUFFER_SIZE>,
    ) { panic!("STUB: not implemented") }

    #[cfg(feature = "timing")]
    fn print_timing(&self) { panic!("STUB: not implemented") }

    #[cfg(feature = "timing")]
    fn record_timing(&self, time: Duration) { panic!("STUB: not implemented") }

    fn leaf_for_key<'a, Q>(
        &'a self,
        search: LeafSearch<&Q>,
        guard: &mut Guard<'a, Deferred<K, V, FANOUT>, LOCAL_GC_BUFFER_SIZE>,
    ) -> NodeView<K, V, FANOUT>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
#[repr(u8)]
enum Data<K, V, const FANOUT: usize>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    Leaf(StackMap<K, V, FANOUT>),
    Index(StackMap<K, BoxedAtomicPtr<K, V, FANOUT>, FANOUT>),
}

impl<K, V, const FANOUT: usize> Data<K, V, FANOUT>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    const fn len(&self) -> usize {
        match self {
            Data::Leaf(ref leaf) => leaf.len(),
            Data::Index(ref index) => index.len(),
        }
    }
}

#[derive(Debug)]
struct Node<K, V, const FANOUT: usize>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    next: Option<BoxedAtomicPtr<K, V, FANOUT>>,
    merging_child: Option<BoxedAtomicPtr<K, V, FANOUT>>,
    data: Data<K, V, FANOUT>,
    lo: K,
    hi: Option<K>,
    is_merging: bool,
}

impl<K, V, const FANOUT: usize> Clone for Node<K, V, FANOUT>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn clone(&self) -> Node<K, V, FANOUT> { panic!("STUB: not implemented") }
}

impl<K, V, const FANOUT: usize> Node<K, V, FANOUT>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    const fn index(&self) -> &StackMap<K, BoxedAtomicPtr<K, V, FANOUT>, FANOUT> {
        if let Data::Index(ref index) = self.data {
            index
        } else {
            unreachable!()
        }
    }

    fn index_mut(&mut self) -> &mut StackMap<K, BoxedAtomicPtr<K, V, FANOUT>, FANOUT> { panic!("STUB: not implemented") }

    const fn leaf(&self) -> &StackMap<K, V, FANOUT> {
        if let Data::Leaf(ref leaf) = self.data {
            leaf
        } else {
            unreachable!()
        }
    }

    fn leaf_mut(&mut self) -> &mut StackMap<K, V, FANOUT> { panic!("STUB: not implemented") }

    const fn is_leaf(&self) -> bool {
        matches!(self.data, Data::Leaf(..))
    }

    fn new_root() -> Box<Node<K, V, FANOUT>> { panic!("STUB: not implemented") }

    fn new_leaf(lo: K) -> Box<Node<K, V, FANOUT>> { panic!("STUB: not implemented") }

    fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    { panic!("STUB: not implemented") }

    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    { panic!("STUB: not implemented") }

    fn insert(&mut self, key: K, value: V) -> Option<V> { panic!("STUB: not implemented") }

    fn cas<V2>(
        &mut self,
        key: K,
        old: Option<&V2>,
        new: Option<V>,
    ) -> Result<Option<V>, CasFailure<V>>
    where
        V: Borrow<V2>,
        V2: ?Sized + PartialEq,
    { panic!("STUB: not implemented") }

    const fn should_merge(&self) -> bool {
        if self.merging_child.is_some() || self.is_merging {
            return false;
        }
        self.len() <= MERGE_SIZE
    }

    const fn should_split(&self) -> bool {
        if self.merging_child.is_some() || self.is_merging {
            return false;
        }
        self.len() > FANOUT - MERGE_SIZE
    }

    const fn len(&self) -> usize {
        self.data.len()
    }

    fn split(&mut self) -> BoxedAtomicPtr<K, V, FANOUT> { panic!("STUB: not implemented") }

    fn merge(&mut self, rhs: &NodeView<K, V, FANOUT>) { panic!("STUB: not implemented") }

    fn is_viable_parent_for(&self, possible_child: &NodeView<K, V, FANOUT>) -> bool { panic!("STUB: not implemented") }
}

impl<K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize> FromIterator<(K, V)>
    for ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self { panic!("STUB: not implemented") }
}

impl<'a, K, V, const FANOUT: usize, const LOCAL_GC_BUFFER_SIZE: usize> IntoIterator
    for &'a ConcurrentMap<K, V, FANOUT, LOCAL_GC_BUFFER_SIZE>
where
    K: 'static + Clone + Minimum + Ord + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    type Item = (K, V);
    type IntoIter = Iter<'a, K, V, FANOUT, LOCAL_GC_BUFFER_SIZE, std::ops::RangeFull>;

    fn into_iter(self) -> Self::IntoIter { panic!("STUB: not implemented") }
}

const fn _test_impls() {
    const fn send<T: Send>() {}
    const fn clone<T: Clone>() {}
    send::<ConcurrentMap<usize, usize>>();
    clone::<ConcurrentMap<usize, usize>>();
}

#[test]
fn basic_map() {
    let map = ConcurrentMap::<usize, usize>::default();

    let n = 64; 
    for i in 0..=n {
        assert_eq!(map.get(&i), None);
        map.insert(i, i);
        assert_eq!(map.get(&i), Some(i), "failed to get key {i}");
    }

    for (i, (k, _v)) in map.range(..).enumerate() {
        assert_eq!(i, k);
    }

    for (i, (k, _v)) in map.range(..).rev().enumerate() {
        assert_eq!(n - i, k);
    }

    for (i, (k, _v)) in map.iter().enumerate() {
        assert_eq!(i, k);
    }

    for (i, (k, _v)) in map.iter().rev().enumerate() {
        assert_eq!(n - i, k);
    }

    for (i, (k, _v)) in map.range(0..).enumerate() {
        assert_eq!(i, k);
    }

    for (i, (k, _v)) in map.range(0..).rev().enumerate() {
        assert_eq!(n - i, k);
    }

    for (i, (k, _v)) in map.range(0..n).enumerate() {
        assert_eq!(i, k);
    }

    for (i, (k, _v)) in map.range(0..n).rev().enumerate() {
        assert_eq!((n - 1) - i, k);
    }

    for (i, (k, _v)) in map.range(0..=n).enumerate() {
        assert_eq!(i, k);
    }

    for (i, (k, _v)) in map.range(0..=n).rev().enumerate() {
        assert_eq!(n - i, k);
    }

    for i in 0..=n {
        assert_eq!(map.get(&i), Some(i), "failed to get key {i}");
    }
}

#[test]
fn timing_map() {
    use std::time::Instant;

    let map = ConcurrentMap::<u64, u64>::default();

    let n = 1024 * 1024;

    let insert = Instant::now();
    for i in 0..n {
        map.insert(i, i);
    }
    let insert_elapsed = insert.elapsed();
    println!(
        "{} inserts/s, total {:?}",
        (n * 1_000_000) / u64::try_from(insert_elapsed.as_micros().max(1)).unwrap_or(u64::MAX),
        insert_elapsed
    );

    let scan = Instant::now();
    let count = map.range(..).count();
    assert_eq!(count as u64, n);
    let scan_elapsed = scan.elapsed();
    println!(
        "{} scanned items/s, total {:?}",
        (n * 1_000_000) / u64::try_from(scan_elapsed.as_micros().max(1)).unwrap_or(u64::MAX),
        scan_elapsed
    );

    let scan_rev = Instant::now();
    let count = map.range(..).rev().count();
    assert_eq!(count as u64, n);
    let scan_rev_elapsed = scan_rev.elapsed();
    println!(
        "{} reverse-scanned items/s, total {:?}",
        (n * 1_000_000) / u64::try_from(scan_rev_elapsed.as_micros().max(1)).unwrap_or(u64::MAX),
        scan_rev_elapsed
    );

    let gets = Instant::now();
    for i in 0..n {
        map.get(&i);
    }
    let gets_elapsed = gets.elapsed();
    println!(
        "{} gets/s, total {:?}",
        (n * 1_000_000) / u64::try_from(gets_elapsed.as_micros().max(1)).unwrap_or(u64::MAX),
        gets_elapsed
    );
}
