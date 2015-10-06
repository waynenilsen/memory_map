//! A cross-platform Rust API for memory-mapped file IO.

#![doc(html_logo_url =
           "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
       html_root_url = "http://maidsafe.github.io/memory_map")]

// For explanation of lint checks, run `rustc -W help` or see
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md
#![forbid(bad_style, exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(deprecated, drop_with_repr_extern, improper_ctypes, missing_docs,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, raw_pointer_derive, stable_features,
        unconditional_recursion, unknown_lints, unsafe_code, unused, unused_allocation,
        unused_attributes, unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, variant_size_differences)]
#![allow(box_pointers, fat_ptr_transmutes, missing_copy_implementations,
         missing_debug_implementations)]

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::MmapInner;

#[cfg(not(target_os = "windows"))]
mod posix;
#[cfg(not(target_os = "windows"))]
use posix::MmapInner;

use std::{fs, io};
use std::borrow::{Borrow, BorrowMut};
use std::ops::{
    Deref, DerefMut,
    Index, IndexMut,
    Range, RangeFrom, RangeTo, RangeFull,
};
use std::path::Path;

/// Memory map protection.
///
/// Determines how a memory map may be used. If the memory map is backed by a file, then the file
/// must have permissions corresponding to the operations the protection level allows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protection {

    /// A read-only memory map. Writes to the memory map will result in a panic.
    Read,

    /// A read-write memory map. Writes to the memory map will be reflected in the file after a
    /// call to `Mmap::flush` or after the `Mmap` is dropped.
    ReadWrite,

    /// A read, copy-on-write memory map. Writes to the memory map will not be carried through to
    /// the underlying file. It is unspecified whether changes made to the file after the memory map
    /// is created will be visible.
    ReadCopy,
}

impl Protection {

    fn as_open_options(self) -> fs::OpenOptions {
        let mut options = fs::OpenOptions::new();
        let _ = options.read(true)
                       .write(self.write());

        options
    }

    /// Returns `true` if the `Protection` is writable.
    pub fn write(self) -> bool {
        use Protection::*;
        match self {
            ReadWrite | ReadCopy => true,
            _ => false,
        }
    }
}

/// A memory-mapped buffer.
///
/// A file-backed `Mmap` buffer may be used to read or write data to a file. Use `Mmap::open(..)` to
/// create a file-backed memory map. An anonymous `Mmap` buffer may be used any place that an
/// in-memory byte buffer is needed, and gives the added features of a memory map. Use
/// `Mmap::anonymous(..)` to create an anonymous memory map.
///
/// Changes written to a memory-mapped file are not guaranteed to be durable until the memory map is
/// flushed, or it is dropped.
///
/// ```
/// #[allow(dead_code)]
/// use std::io::Write;
/// use memory_map::{Mmap, Protection};
///
/// let file_mmap = Mmap::open("README.md", Protection::Read).unwrap();
/// assert_eq!(b"# Memory Map", &file_mmap[0..12]);
///
/// let mut anon_mmap = Mmap::anonymous(4096, Protection::ReadWrite).unwrap();
/// (&mut *anon_mmap).write(b"foo").unwrap();
/// assert_eq!(b"foo\0\0", &anon_mmap[0..5]);
/// ```

pub struct Mmap {
    inner: MmapInner
}

impl Mmap {

    /// Opens a file-backed memory map.
    pub fn open<P>(path: P, prot: Protection) -> io::Result<Mmap> where P: AsRef<Path> {
        MmapInner::open(path, prot).map(|inner| Mmap { inner: inner })
    }

    /// Opens an anonymous memory map.
    pub fn anonymous(len: usize, prot: Protection) -> io::Result<Mmap> {
        MmapInner::anonymous(len, prot).map(|inner| Mmap { inner: inner })
    }

    /// Flushes outstanding memory map modifications to disk.
    ///
    /// When this returns with a non-error result, all outstanding changes to a file-backed memory
    /// map are guaranteed to be durably stored. The file's metadata (including last modification
    /// timestamp) may not be updated.
    pub fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    /// Asynchronously flushes outstanding memory map modifications to disk.
    ///
    /// This method initiates flushing modified pages to durable storage, but it will not wait
    /// for the operation to complete before returning. The file's metadata (including last
    /// modification timestamp) may not be updated.
    pub fn flush_async(&mut self) -> io::Result<()> {
        self.inner.flush_async()
    }

    /// Returns the length of the memory map.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Deref for Mmap {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &*self.inner
    }
}

impl DerefMut for Mmap {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut *self.inner
    }
}

impl AsRef<[u8]> for Mmap {
    fn as_ref(&self) -> &[u8] {
        &*self
    }
}

impl AsMut<[u8]> for Mmap {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut *self
    }
}

impl Borrow<[u8]> for Mmap {
    fn borrow(&self) -> &[u8] {
        &*self
    }
}

impl BorrowMut<[u8]> for Mmap {
    fn borrow_mut(&mut self) -> &mut [u8] {
        &mut *self
    }
}

impl Index<usize> for Mmap {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &(*self.inner)[index]
    }
}

impl IndexMut<usize> for Mmap {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut (*self.inner)[index]
    }
}

impl Index<Range<usize>> for Mmap {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &[u8] {
        Index::index(&**self, index)
    }
}

impl Index<RangeTo<usize>> for Mmap {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &[u8] {
        Index::index(&**self, index)
    }
}

impl Index<RangeFrom<usize>> for Mmap {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &[u8] {
        Index::index(&**self, index)
    }
}

impl Index<RangeFull> for Mmap {
    type Output = [u8];

    fn index(&self, _index: RangeFull) -> &[u8] {
        self
    }
}

impl IndexMut<Range<usize>> for Mmap {
    fn index_mut(&mut self, index: Range<usize>) -> &mut [u8] {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl IndexMut<RangeTo<usize>> for Mmap {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut [u8] {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl IndexMut<RangeFrom<usize>> for Mmap {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [u8] {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl IndexMut<RangeFull> for Mmap {
    fn index_mut(&mut self, _index: RangeFull) -> &mut [u8] {
        self
    }
}

#[cfg(test)]
mod test {
    extern crate tempdir;

    use std::{fs, iter};
    use std::io::{Read, Write};
    use std::error::Error;
    use std::thread;

    use super::*;

    #[test]
    fn map_file() {
        let expected_len = 128;
        let tempdir = tempdir::TempDir::new("mmap").unwrap();
        let path = tempdir.path().join("mmap");

        fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(&path).unwrap()
                        .set_len(expected_len as u64).unwrap();

        let mut mmap = Mmap::open(path, Protection::ReadWrite).unwrap();
        let len = mmap.len();
        assert_eq!(expected_len, len);

        let zeros = iter::repeat(0).take(len).collect::<Vec<_>>();
        let incr = (0..len).map(|n| n as u8).collect::<Vec<_>>();

        // check that the mmap is empty
        assert_eq!(&zeros[..], &*mmap);

        // write values into the mmap
        mmap.as_mut().write_all(&incr[..]).unwrap();

        // read values back
        assert_eq!(&incr[..], &*mmap);
    }

    // Check that a 0-length file will not be mapped
    #[test]
    fn map_empty_file() {
        let tempdir = tempdir::TempDir::new("mmap").unwrap();
        let path = tempdir.path().join("mmap");

        let _ = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&path).unwrap();

        assert!(Mmap::open(path, Protection::ReadWrite).is_err());
    }


    #[test]
    fn map_anon() {
        let expected_len = 128;
        let mut mmap = Mmap::anonymous(expected_len, Protection::ReadWrite).unwrap();
        let len = mmap.len();
        assert_eq!(expected_len, len);

        let zeros = iter::repeat(0).take(len).collect::<Vec<_>>();
        let incr = (0..len).map(|n| n as u8).collect::<Vec<_>>();

        // check that the mmap is empty
        assert_eq!(&zeros[..], &*mmap);

        // write values into the mmap
        mmap.as_mut().write_all(&incr[..]).unwrap();

        // read values back
        assert_eq!(&incr[..], &*mmap);
    }

    #[test]
    fn anonymous_overflow() {
        let expected_len = 128;
        let mut mmap = Mmap::anonymous(expected_len, Protection::ReadWrite).unwrap();
        let len = mmap.len();
        assert_eq!(expected_len, len);

        let zeros = iter::repeat(0).take(len).collect::<Vec<_>>();
        // add more values than the mapping can handle
        let incr = (0..len + 1).map(|n| n as u8).collect::<Vec<_>>();
        // expected values written
        let expected = (0..len).map(|n| n as u8).collect::<Vec<_>>();

        // check that the mmap is empty
        assert_eq!(&zeros[..], &*mmap);

        // try to write values into the mmap
        match mmap.as_mut().write_all(&incr[..]) {
            Ok(()) => panic!("write to mapping succeeded."),
            Err(error) => assert_eq!(error.description(), "failed to write whole buffer"),
        }

        // read values back
        assert_eq!(&expected[..], &*mmap);
    }

    #[test]
    fn file_write() {
        let tempdir = tempdir::TempDir::new("mmap").unwrap();
        let path = tempdir.path().join("mmap");

        let mut file = fs::OpenOptions::new()
                                       .read(true)
                                       .write(true)
                                       .create(true)
                                       .open(&path).unwrap();
        file.set_len(128).unwrap();

        let write = b"abc123";
        let mut read = [0u8; 6];

        let mut mmap = Mmap::open(&path, Protection::ReadWrite).unwrap();
        let _ = (&mut mmap[..]).write(write).unwrap();
        mmap.flush().unwrap();

        let _ = file.read(&mut read).unwrap();
        assert_eq!(write, &read);
    }

    #[test]
    fn file_overflow() {
        const EXPECTED_LENGTH: usize = 128;
        let tempdir = tempdir::TempDir::new("mmap").unwrap();
        let path = tempdir.path().join("mmap");

        let mut file = fs::OpenOptions::new()
                                       .read(true)
                                       .write(true)
                                       .create(true)
                                       .open(&path).unwrap();
        file.set_len(EXPECTED_LENGTH as u64).unwrap();

        let incr = (0..EXPECTED_LENGTH + 1).map(|n| n as u8).collect::<Vec<_>>();
        let expected = (0..EXPECTED_LENGTH).map(|n| n as u8).collect::<Vec<_>>();

        let mut mmap = Mmap::open(&path, Protection::ReadWrite).unwrap();

        match (&mut mmap[..]).write(&incr[..]) {
            Ok(size) => assert_eq!(EXPECTED_LENGTH, size),
            Err(_) => panic!("write to mapping failed."),
        }

        mmap.flush().unwrap();

        let mut read = [0u8; EXPECTED_LENGTH];
        let _ = file.read(&mut read).unwrap();
        assert_eq!(expected, read.to_vec());
    }


    #[test]
    fn map_copy() {
        let tempdir = tempdir::TempDir::new("mmap").unwrap();
        let path = tempdir.path().join("mmap");

        let mut file = fs::OpenOptions::new()
                                       .read(true)
                                       .write(true)
                                       .create(true)
                                       .open(&path).unwrap();
        file.set_len(128).unwrap();

        let nulls = b"\0\0\0\0\0\0";
        let write = b"abc123";
        let mut read = [0u8; 6];

        let mut mmap = Mmap::open(&path, Protection::ReadCopy).unwrap();
        let _ = (&mut mmap[..]).write(write).unwrap();
        mmap.flush().unwrap();

        // The mmap contains the write
        let _ = (&*mmap).read(&mut read).unwrap();
        assert_eq!(write, &read);

        // The file does not contain the write
        let _ = file.read(&mut read).unwrap();
        assert_eq!(nulls, &read);

        // another mmap does not contain the write
        let mmap2 = Mmap::open(&path, Protection::Read).unwrap();
        let _ = (&*mmap2).read(&mut read).unwrap();
        assert_eq!(nulls, &read);
    }

    #[test]
    fn index() {
        let mut mmap = Mmap::anonymous(128, Protection::ReadWrite).unwrap();
        mmap[0] = 42;
        assert_eq!(42, mmap[0]);
    }

    #[test]
    fn send() {
        let mut mmap = Mmap::anonymous(128, Protection::ReadWrite).unwrap();
        let _ = (&mut mmap[..]).write(b"foobar").unwrap();
        let _ = thread::spawn(move || {
            mmap.flush().unwrap();
        });
    }
}
