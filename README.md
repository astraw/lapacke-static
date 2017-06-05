# lapacke-static - Statically compiled LAPACKE library

## What is this?

This is a package of LAPACKE into a statically compiled rust crate. LAPACKE
is a library that allows calling LAPACK implementations with row-major matrix
memory layouts, as is conventional in C.

## Why?

Some LAPACK implementations (e.g. Apple's Accelerate.framework) do not provide
the LAPACKE interface, instead supporting only column-major matrix memory layouts.

With this crate, rust code can use either column-major layouts (using directly
the the lapack-sys implementation) or row-major layouts (via this crate).

## Where is this code from?

`LAPACKE/` directory extracted from lapack-3.7.0.tgz downloaded from netlib.org
(sha256sum ef6ce65c1339dd680699d365cc325f8c2310bc10b8b1573a79c5ca5c9bfe1945 )

## License

BSD-3-Clause
