//! Debugging macros. These macros allow adding suppressable debug
//! prints to code.
//!
//! To enable debugging prints, compile this crate with the `debug_emit` feature enabled.  See
//! [set_debug] for details.

#[doc(hidden)]
pub use std::io::stderr;
#[doc(hidden)]
pub use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
#[doc(hidden)]
pub use std::{write, writeln};

/// Rename of [std::io::Write] as a convenience.
pub use std::io::Write as WriteIO;

static DEBUG: AtomicBool =
    AtomicBool::new(cfg!(feature="debug_emit") && (cfg!(debug_assertions) || cfg!(test)));

/// Force debugging on or off.
///
/// Debugging will be on by default when this crate is compiled with its `debug_emit` feature
/// enabled, *and also* `debug_assertions` or `test` configured. Otherwise/// debugging will be
/// off by default.
pub fn set_debug(debug: bool) {
    DEBUG.store(debug, SeqCst);
}

/// Report current debugging status.
pub fn is_debug() -> bool {
    DEBUG.load(SeqCst)
}

/// Write a message to a formatter ala [std::writeln]. The formatter
/// must have a `write_fmt` method: generally this is either [std::fmt::Write] or
/// [std::io::Write].
///
/// # Examples
///
/// ```
/// use debug_macros::debug_writeln;
/// use std::fmt::Write; // For writing a String.
/// let mut logger = String::new();
/// debug_macros::set_debug(true); // Overrides default.
/// debug_writeln!(&mut logger, "oopsie", Some(0));
/// assert_eq!(logger, "debug: oopsie: Some(0)\n");
/// ```
///
/// # Panics
///
/// Panics if a write fails.
#[macro_export]
macro_rules! debug_writeln {
    ($f:expr, $msg:literal, $x0:expr $(, $xs:expr)* $(,)?) => {
        if $crate::is_debug() {
            $crate::write!($f, "debug: {}: ", $msg).unwrap();
            $crate::write!($f, "{:?}", $x0).unwrap();
            $($crate::write!($f, ", {:?}", $xs).unwrap();)*
            $crate::writeln!($f).unwrap();
        }
    };
    ($f:expr, $msg:literal) => {
        if $crate::is_debug() {
            $crate::writeln!($f, "debug: {}", $msg).unwrap();
        }
    };
    ($f:expr) => {
        if $crate::is_debug() {
            $crate::writeln!($f, "debug").unwrap();
        }
    };
}

/// Calls [debug_writeln] to write to [std::io::stderr] (locked, so that
/// debug output occurs consecutively).
#[macro_export]
macro_rules! debug {
    ($msg:literal, $($e:expr),+ $(,)?) => {{
        use $crate::WriteIO;
        let stderr = $crate::stderr();
        $crate::debug_writeln!(&mut stderr.lock(), $msg, $($e),*);
    }};
    ($msg:literal) => {{
        use $crate::WriteIO;
        let stderr = $crate::stderr();
        $crate::debug_writeln!(&mut stderr.lock(), $msg);
    }};
    () => {{
        use $crate::WriteIO;
        let stderr = $crate::stderr();
        $crate::debug_writeln!(&mut stderr.lock());
    }};
}

#[test]
pub fn test_debug_writeln() {
    use std::fmt::Write;
    set_debug(true);
    macro_rules! test_msg {
        ($r:literal, $m:literal $(, $e:expr)* ; $($comma:tt)?) => {{
            assert!($crate::is_debug());
            let mut msg = String::new();
            debug_writeln!(&mut msg, $m $(, $e)* $($comma)?);
            assert_eq!($r, msg);
        }};
        () => {{
            let mut msg = String::new();
            debug_writeln!(&mut msg);
            assert_eq!("debug\n", msg);
        }};
    }

    test_msg!("debug: running: Some(5), \"x\"\n", "running", Some(5), "x";);
    test_msg!("debug: running: Some(5), \"x\"\n", "running", Some(5), "x";,);
    test_msg!("debug: still running\n", "still running";);
    test_msg!();
}

// XXX This test is currently disabled since it writes on stderr.  There is no good way to capture
// this output, and it blorts into the `cargo test` output where it is not wanted.
#[cfg(any())]
#[test]
fn test_debug() {
    debug!("debugging", "example value");
}
