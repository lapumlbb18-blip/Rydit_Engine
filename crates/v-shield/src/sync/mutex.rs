//! Mutex wrapper multiplataforma
//!
//! - `native`: `std::sync::Mutex`
//! - `wasm`: `std::sync::Mutex` (WASM single-threaded, pero thread-safe para compatibilidad)
//! - `async-tokio`: `tokio::sync::Mutex`

#[cfg(all(feature = "native", not(feature = "async-tokio")))]
pub use std::sync::Mutex;

#[cfg(all(feature = "wasm", not(feature = "async-tokio")))]
pub use std::sync::Mutex;

#[cfg(feature = "async-tokio")]
pub use tokio::sync::Mutex;

#[cfg(test)]
mod tests {
    use super::Mutex;

    #[test]
    fn test_mutex_basic() {
        let m = Mutex::new(42);
        let guard = m.lock().unwrap();
        assert_eq!(*guard, 42);
        drop(guard);

        let m2 = Mutex::new(vec![1, 2, 3]);
        {
            let mut g = m2.lock().unwrap();
            g.push(4);
        }
        assert_eq!(*m2.lock().unwrap(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_mutex_arc() {
        use std::sync::Arc;
        let m = Arc::new(Mutex::new(0));
        let m_clone = Arc::clone(&m);
        let handle = std::thread::spawn(move || {
            let mut g = m_clone.lock().unwrap();
            *g += 1;
        });
        {
            let mut g = m.lock().unwrap();
            *g += 1;
        }
        handle.join().unwrap();
        assert_eq!(*m.lock().unwrap(), 2);
    }
}
