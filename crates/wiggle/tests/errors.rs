/// Execute the wiggle guest conversion code to exercise it
mod convert_just_errno {
    use anyhow::Result;
    use wiggle_test::{impl_errno, HostMemory, WasiCtx};

    // Define an errno with variants corresponding to RichError. Use it in a
    // trivial function.
    wiggle::from_witx!({
        witx_literal: "
(typename $errno (enum (@witx tag u8) $ok $invalid_arg $picket_line))
(module $one_error_conversion
  (@interface func (export \"foo\")
     (param $strike u32)
     (result $err (expected (error $errno)))))
    ",
        errors: { errno },
    });

    impl_errno!(types::Errno);

    impl one_error_conversion::OneErrorConversion for WasiCtx {
        fn foo(&mut self, strike: u32) -> Result<(), wiggle::Error<types::Errno>> {
            // We use the argument to this function to exercise all of the
            // possible error cases we could hit here
            match strike {
                0 => Ok(()),
                1 => Err(types::Errno::PicketLine.into()),
                _ => Err(types::Errno::InvalidArg.into()),
            }
        }
    }

    #[test]
    fn one_error_conversion_test() {
        let mut ctx = WasiCtx::new();
        let host_memory = HostMemory::new();

        // Exercise each of the branches in `foo`.
        // Start with the success case:
        let r0 = one_error_conversion::foo(&mut ctx, &host_memory, 0).unwrap();
        assert_eq!(
            r0,
            types::Errno::Ok as i32,
            "Expected return value for strike=0"
        );

        // First error case:
        let r1 = one_error_conversion::foo(&mut ctx, &host_memory, 1).unwrap();
        assert_eq!(
            r1,
            types::Errno::PicketLine as i32,
            "Expected return value for strike=1"
        );

        // Second error case:
        let r2 = one_error_conversion::foo(&mut ctx, &host_memory, 2).unwrap();
        assert_eq!(
            r2,
            types::Errno::InvalidArg as i32,
            "Expected return value for strike=2"
        );
    }
}

/// Type-check the wiggle guest conversion code against a more complex case where
/// we use two distinct error types.
mod convert_multiple_error_types {
    use anyhow::Result;
    use wiggle_test::{impl_errno, WasiCtx};

    // Just like the prior test, except that we have a second errno type. This should mean there
    // are two functions in UserErrorConversion.
    // Additionally, test that the function "baz" marked noreturn always returns a wasmtime::Trap.
    wiggle::from_witx!({
        witx_literal: "
(typename $errno (enum (@witx tag u8) $ok $invalid_arg $picket_line))
(typename $errno2 (enum (@witx tag u8) $ok $too_much_coffee))
(module $two_error_conversions
  (@interface func (export \"foo\")
     (param $strike u32)
     (result $err (expected (error $errno))))
  (@interface func (export \"bar\")
     (param $drink u32)
     (result $err (expected (error $errno2))))
  (@interface func (export \"baz\")
     (param $drink u32)
     (@witx noreturn)))
    ",
        errors: { errno, errno2 },
    });

    impl_errno!(types::Errno);
    impl_errno!(types::Errno2);

    // And here's the witx module trait impl, bodies elided
    impl two_error_conversions::TwoErrorConversions for WasiCtx {
        fn foo(&mut self, _: u32) -> Result<(), wiggle::Error<types::Errno>> {
            unimplemented!()
        }
        fn bar(&mut self, _: u32) -> Result<(), wiggle::Error<types::Errno2>> {
            unimplemented!()
        }
        fn baz(&mut self, _: u32) -> anyhow::Error {
            unimplemented!()
        }
    }
}
