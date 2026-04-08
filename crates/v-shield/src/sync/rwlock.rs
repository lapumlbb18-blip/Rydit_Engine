//! RwLock wrapper multiplataforma
//!
//! - `native`: `std::sync::RwLock`
//! - `wasm`: `std::sync::RwLock` (compatibilidad)
//! - `async-tokio`: `tokio::sync::RwLock`

#[cfg(all(feature = "native", not(feature = "async-tokio")))]
pub use std::sync::RwLock;

#[cfg(all(feature = "wasm", not(feature = "async-tokio")))]
pub use std::sync::RwLock;

#[cfg(feature = "async-tokio")]
pub use tokio::sync::RwLock;

#[cfg(test)]
mod tests {
    use super::RwLock;

    #[test]
    fn test_rwlock_read() {
        let rw = RwLock::new(42);
        let read = rw.read().unwrap();
        assert_eq!(*read, 42);
    }

    #[test]
    fn test_rwlock_write() {
        let rw = RwLock::new(vec![1, 2]);
        {
            let mut write = rw.write().unwrap();
            write.push(3);
        }
        let read = rw.read().unwrap();
        assert_eq!(*read, vec![1, 2, 3]);
    }

    #[test]
    fn test_rwlock_multiple_readers() {
        let rw = std::sync::Arc::new(RwLock::new("hello"));
        let r1 = rw.read().unwrap();
        let r2 = rw.read().unwrap();
        assert_eq!(*r1, "hello");
        assert_eq!(*r2, "hello");
        drop(r1);
        drop(r2);
    }
}
