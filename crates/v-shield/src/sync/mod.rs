//! 🛡️ V-Shield Sync Primitives
//!
//! Abstracción multiplataforma para primitivas de sincronización.
//!
//! # Features
//!
//! - `native` (default): Usa `std::sync` para Linux/Windows/macOS
//! - `wasm`: Usa `wasm-bindgen` + `js-sys` para WASM
//! - `async-tokio`: Wrappers async con `tokio::sync`
//!
//! # Ejemplo
//!
//! ```rust
//! use v_shield::sync::{Mutex, RwLock};
//!
//! let lock = Mutex::new(42);
//! let guard = lock.lock().unwrap();
//! assert_eq!(*guard, 42);
//!
//! let rw = RwLock::new(vec![1, 2, 3]);
//! let read = rw.read().unwrap();
//! assert_eq!(read.len(), 3);
//! ```

pub mod mutex;
pub mod rwlock;

pub use mutex::Mutex;
pub use rwlock::RwLock;

// Re-export de condvar y barrier desde std (siempre disponible en native)
#[cfg(all(feature = "native", not(feature = "async-tokio")))]
pub use std::sync::{Barrier, Condvar};

// En WASM, Barrier y Condvar no están en std, usamos fallback
#[cfg(feature = "wasm")]
pub use wasm_fallback::{Barrier, Condvar};

#[cfg(feature = "wasm")]
mod wasm_fallback {
    use std::sync::Mutex as StdMutex;
    use std::sync::Condvar as StdCondvar;

    /// Fallback de Barrier para WASM (usa Condvar + contador)
    pub struct Barrier {
        count: StdMutex<usize>,
        condvar: StdCondvar,
        expected: usize,
    }

    impl Barrier {
        pub fn new(n: usize) -> Self {
            Self {
                count: StdMutex::new(0),
                condvar: StdCondvar::new(),
                expected: n,
            }
        }

        pub fn wait(&self) {
            let mut count = self.count.lock().unwrap();
            *count += 1;
            if *count >= self.expected {
                *count = 0;
                self.condvar.notify_all();
            } else {
                drop(count);
                let _ = self.condvar.wait_while(
                    self.count.lock().unwrap(),
                    |c| *c < self.expected,
                );
            }
        }
    }

    /// Fallback de Condvar para WASM (re-export std)
    pub type Condvar = StdCondvar;
}

// Async wrappers (solo con feature async-tokio)
#[cfg(feature = "async-tokio")]
pub mod async_wrappers {
    pub use tokio::sync::{Mutex as AsyncMutex, RwLock as AsyncRwLock};
}
