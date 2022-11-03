use anyhow::Result;
use wiggle_test::{impl_errno, HostMemory, WasiCtx};

// Define an errno with variants corresponding to RichError. Use it in a
// trivial function.
wiggle::from_witx!({
witx_literal: "
(typename $errno (enum (@witx tag u8) $ok $invalid_arg $picket_line))
(typename $s (record (field $f1 (@witx usize)) (field $f2 (@witx pointer u8))))
(typename $t (record (field $f1 u32) (field $f2 f32)))
(module $one_error_conversion
  (@interface func (export \"foo\")
     (param $strike u32)
     (param $s $s)
     (result $err (expected $t (error $errno)))))
    ",
    errors: { errno },
});

impl_errno!(types::Errno);
impl<'a> one_error_conversion::OneErrorConversion for WasiCtx {
    fn foo(&mut self, strike: u32, _s: &types::S) -> Result<types::T, wiggle::Error<types::Errno>> {
        // We use the argument to this function to exercise all of the
        // possible error cases we could hit here
        match strike {
            0 => Ok(types::T {
                f1: 123,
                f2: 456.78,
            }),
            1 => Err(wiggle::Error::new(types::Errno::PicketLine)
                .context("you can throw a string in here")),
            _ => Err(wiggle::Error::new(types::Errno::InvalidArg)
                .context("and it will show up as the error in the tracing logs")),
        }
    }
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        // with no RUST_LOG env variable: use the tracing subscriber.
        let subscriber = tracing_subscriber::fmt()
            // all spans/events with a level equal to or higher than TRACE (e.g, trace, debug, info, warn, etc.)
            // will be written to stdout.
            .with_max_level(tracing::Level::TRACE)
            // builds the subscriber.
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("set global tracing subscriber");
    } else {
        // with RUST_LOG set: use the env_logger backend to tracing.
        env_logger::init();
    }

    let mut ctx = WasiCtx::new();
    let host_memory = HostMemory::new();

    // Exercise each of the branches in `foo`.
    // Start with the success case:
    let r0 = one_error_conversion::foo(&mut ctx, &host_memory, 0, 0, 8).unwrap();
    assert_eq!(
        r0,
        types::Errno::Ok as i32,
        "Expected return value for strike=0"
    );
    assert!(ctx.log.borrow().is_empty(), "No error log for strike=0");

    // First error case:
    let r1 = one_error_conversion::foo(&mut ctx, &host_memory, 1, 0, 8).unwrap();
    assert_eq!(
        r1,
        types::Errno::PicketLine as i32,
        "Expected return value for strike=1"
    );
    // Second error case:
    let r2 = one_error_conversion::foo(&mut ctx, &host_memory, 2, 0, 8).unwrap();
    assert_eq!(
        r2,
        types::Errno::InvalidArg as i32,
        "Expected return value for strike=2"
    );
}
