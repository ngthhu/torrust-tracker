use std::sync::Arc;

use repository::rw_lock_std::RwLockStd;
use repository::rw_lock_tokio::RwLockTokio;
use repository::skip_map_mutex_std::CrossbeamSkipList;
use torrust_tracker_clock::clock;

pub mod entry;
pub mod repository;

pub type EntrySingle = entry::Torrent;
pub type EntryMutexStd = Arc<std::sync::Mutex<entry::Torrent>>;
pub type EntryMutexTokio = Arc<tokio::sync::Mutex<entry::Torrent>>;

pub type TorrentsRwLockStd = RwLockStd<EntrySingle>;
pub type TorrentsRwLockStdMutexStd = RwLockStd<EntryMutexStd>;
pub type TorrentsRwLockStdMutexTokio = RwLockStd<EntryMutexTokio>;
pub type TorrentsRwLockTokio = RwLockTokio<EntrySingle>;
pub type TorrentsRwLockTokioMutexStd = RwLockTokio<EntryMutexStd>;
pub type TorrentsRwLockTokioMutexTokio = RwLockTokio<EntryMutexTokio>;

pub type TorrentsSkipMapMutexStd = CrossbeamSkipList<EntryMutexStd>;

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;