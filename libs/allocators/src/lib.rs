
#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
pub type Allocator = tikv_jemallocator::Jemalloc;

#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
pub static GLOBAL: Allocator = tikv_jemallocator::Jemalloc;


#[cfg(feature = "mimalloc")]
pub type Allocator = mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
pub static GLOBAL: Allocator = mimalloc::MiMalloc;

#[cfg(feature = "snmalloc")]
pub type Allocator = snmalloc_rs::SnMalloc;

#[cfg(feature = "snmalloc")]
pub static GLOBAL: Allocator = snmalloc_rs::SnMalloc;
