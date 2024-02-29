//! # The ArceOS Standard Library
//!
//! The [ArceOS] Standard Library is a mini-std library, with an interface similar
//! to rust [std], but calling the functions directly in ArceOS modules, instead
//! of using libc and system calls.
//!
//! These features are exactly the same as those in [axfeat], they are used to
//! provide users with the selection of features in axfeat, without import
//! [axfeat] additionally:
//!
//! ## Cargo Features
//!
//! - CPU
//!     - `smp`: Enable SMP (symmetric multiprocessing) support.
//!     - `fp_simd`: Enable floating point and SIMD support.
//! - Interrupts:
//!     - `irq`: Enable interrupt handling support.
//! - Memory
//!     - `alloc`: Enable dynamic memory allocation.
//!     - `alloc-tlsf`: Use the TLSF allocator.
//!     - `alloc-slab`: Use the slab allocator.
//!     - `alloc-buddy`: Use the buddy system allocator.
//!     - `paging`: Enable page table manipulation.
//!     - `tls`: Enable thread-local storage.
//! - Task management
//!     - `multitask`: Enable multi-threading support.
//!     - `sched_fifo`: Use the FIFO cooperative scheduler.
//!     - `sched_rr`: Use the Round-robin preemptive scheduler.
//!     - `sched_cfs`: Use the Completely Fair Scheduler (CFS) preemptive scheduler.
//! - Upperlayer stacks
//!     - `fs`: Enable file system support.
//!     - `myfs`: Allow users to define their custom filesystems to override the default.
//!     - `net`: Enable networking support.
//!     - `dns`: Enable DNS lookup support.
//!     - `display`: Enable graphics support.
//! - Device drivers
//!     - `bus-mmio`: Use device tree to probe all MMIO devices.
//!     - `bus-pci`: Use PCI bus to probe all PCI devices.
//!     - `driver-ramdisk`: Use the RAM disk to emulate the block device.
//!     - `driver-ixgbe`: Enable the Intel 82599 10Gbit NIC driver.
//!     - `driver-bcm2835-sdhci`: Enable the BCM2835 SDHCI driver (Raspberry Pi SD card).
//! - Logging
//!     - `log-level-off`: Disable all logging.
//!     - `log-level-error`, `log-level-warn`, `log-level-info`, `log-level-debug`,
//!       `log-level-trace`: Keep logging only at the specified level or higher.
//!
//! [ArceOS]: https://github.com/rcore-os/arceos

#![cfg_attr(all(not(test), not(doc)), no_std)]
#![feature(doc_cfg)]
#![feature(doc_auto_cfg)]
// 作业里提到的宏
// ulib/axstd/src/lib.rs
#![feature(hashmap_internals)]
#![feature(extend_one)]
#![feature(hasher_prefixfree_extras)]
#![feature(error_in_core)]
#![feature(try_reserve_kind)]
#![feature(thread_local)]
#![feature(const_hash)]

// #[cfg(feature = "alloc")]
extern crate alloc;

// #[cfg(feature = "alloc")]
#[doc(no_inline)]
pub use alloc::{boxed, format, string, vec};

#[doc(no_inline)]
pub use core::{arch, cell, cmp, hint, marker, mem, ops, ptr, slice, str};

#[macro_use]
mod macros;

pub mod env;
pub mod io;
pub mod os;
pub mod process;
pub mod sync;
pub mod thread;
pub mod time;

#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "net")]
pub mod net;

pub mod collections {
    use alloc::vec::Vec;
    use core::{
        hash::{self, Hasher},
        ptr::eq,
    };

    struct HashMap<K, V> {
        buckets: Vec<Vec<(K, V)>>,
    }

    impl<K: hash::Hash + Eq, V: Clone> HashMap<K, V> {
        fn new() -> Self {
            Self {
                buckets: Vec::new(),
            }
        }
        fn resize(&mut self) {
            match self.buckets.len() {
                0 => { //// TODO: 初始化
                }
                _ => { //// TODO: 扩容
                }
            }
        }
        fn insert(&mut self, key: K, value: V) -> Option<V> {
            use hash32::Murmur3Hasher;
            let mut fnv = Murmur3Hasher::default();
            key.hash(&mut fnv);
            let index = (fnv.finish() % self.buckets.len() as u64) as usize;
            let bucket = &mut self.buckets[index];
            for (k, v) in bucket.iter_mut() {
                if eq(k, &key) {
                    //假如找到key相同的tuple，就更新，返回原来的
                    let temp = v.clone();
                    *v = value;
                    return Some(temp);
                }
            }
            bucket.push((key, value));
            None
        }
    }
}
