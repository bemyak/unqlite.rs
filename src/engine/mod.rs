use std::mem;
use std::ffi::CString;
use ffi::{unqlite_close, unqlite_open};

/// UnQlite database entity.
pub struct UnQlite {
    db: *mut ::ffi::unqlite,
}

unsafe impl Send for UnQlite {}
unsafe impl Sync for UnQlite {}

impl Default for UnQlite {
    fn default() -> UnQlite {
        UnQlite { db: unsafe { mem::uninitialized() } }
    }
}

impl<'open> UnQlite {
    /// Create UnQlite database at specific path.
    ///
    /// ```ignore
    /// let _ = UnQlite::open("str");
    /// let _ = UnQlite::open(String::new());
    /// ```
    #[inline]
    fn open<P: AsRef<str>>(filename: P, mode: OpenMode) -> ::Result<UnQlite> {
        unsafe {
            let mut unqlite = UnQlite::default();
            let filename = filename.as_ref();
            let filename = try!(CString::new(filename));
            error_or!(unqlite_open(&mut unqlite.db, filename.as_ptr(), mode.into()),
                      unqlite)
        }
    }

    /// Create UnQlite database as `filename`.
    ///
    /// By default, the database is created in read-write mode.
    ///
    /// ## Panics
    ///
    /// Will panic if failed in creating.
    ///
    /// ## Example
    ///
    /// ```ignore
    /// let _ = UnQlite::create("test.db");
    /// ```
    ///
    /// ## C
    ///
    /// ```c
    /// unqlite *pDb;
    ///
    /// // in-disk database
    /// rc = unqlite_open(&pDb,"test.db",UNQLITE_OPEN_CREATE);
    ///
    /// // in-memory database
    /// rc = unqlite_open(&pDb, ":mem:", UNQLITE_OPEN_MEM);
    /// ```
    #[inline]
    pub fn create<P: AsRef<str>>(filename: P) -> UnQlite {
        Self::open(filename, OpenMode::Create).unwrap()
    }

    /// Create database in memory.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// let _ = UnQlite::create(":mem:");
    /// ```
    #[inline]
    pub fn create_in_memory() -> UnQlite {
        Self::create(":mem:")
    }

    /// A private, temporary on-disk database will be created.
    ///
    /// This private database will be automatically deleted as soon as
    /// the database connection is closed.
    ///
    /// ## C
    ///
    /// ```c
    /// int rc = unqlite_open("test.db", UNQLITE_OPEN_TEMP_DB);
    /// ```
    #[inline]
    pub fn create_temp() -> UnQlite {
        Self::open("", OpenMode::TempDB).unwrap()
    }

    /// Obtain a read-only memory view of the whole database.
    ///
    /// You will get significant performance improvements with this combination but your database
    /// is still read-only.
    ///
    /// ## Panics
    ///
    /// Panic if open failed.
    ///
    /// ## C
    ///
    /// ```c
    /// unqlite_open(&pDb, "test.db", UNQLITE_OPEN_MMAP | UNQLITE_OPEN_READONLY);
    /// ```
    #[inline]
    pub fn open_mmap<P: AsRef<str>>(filename: P) -> UnQlite {
        Self::open(filename, OpenMode::MMap).unwrap()
    }

    /// Open the database in a read-only mode.
    ///
    /// That is, you cannot perform a store, append, commit or rollback operations with this
    /// control flag.
    ///
    /// Always prefer to use `open_mmap` for readonly in disk database.
    ///
    /// ## Panics
    ///
    /// Panic too.
    ///
    /// ## C
    /// ```c
    /// unqlite_open(&pDb, "test.db", UNQLITE_OPEN_READONLY);
    /// ```
    #[inline]
    pub fn open_readonly<P: AsRef<str>>(filename: P) -> UnQlite {
        Self::open(filename, OpenMode::ReadOnly).unwrap()
    }

    fn close(&mut self) -> ::Result<()> {
        unsafe { error_or!(unqlite_close(self.db)) }
    }
}

impl Drop for UnQlite {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

macro_rules! _components {
    ($($i: ident),*) => {
        $(
            pub use self::$i::*;
            mod $i;
         )*
    }
}

_components!(openmode, config, util, transaction);

#[cfg(test)]
#[cfg(feature = "enable-threads")]
mod tests_threadsafe {
    use super::UnQlite;

    #[test]
    fn create_temp() {
        let _ = UnQlite::create_temp();
    }

    #[test]
    fn create_in_memory() {
        let _ = UnQlite::create_in_memory();
    }

    #[test]
    fn from_readonly_memory() {
        let _ = UnQlite::open_readonly(":mem:");
    }
}

#[cfg(test)]
mod tests {
    use super::UnQlite;

    #[test]
    fn open() {
        let _ = UnQlite::create_temp();
        let _ = UnQlite::create_in_memory();
        let _ = UnQlite::open_readonly(":mem:");
    }
}
