//! External names.
//!
//! These are identifiers for declaring entities defined outside the current
//! function. The name of an external declaration doesn't have any meaning to
//! Cretonne, which compiles functions independently.

use std::fmt::{self, Write};

const TESTCASE_NAME_LENGTH: usize = 16;

/// The name of an external is either a reference to a user-defined symbol
/// table, or a short sequence of ascii bytes so that test cases do not have
/// to keep track of a sy mbol table.
///
/// External names are primarily used as keys by code using Cretonne to map
/// from a cretonne::ir::FuncRef or similar to additional associated data.
///
/// External names can also serve as a primitive testing and debugging tool.
/// In particular, many `.cton` test files use function names to identify
/// functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalName {
    /// A name in a user-defined symbol table. Cretonne does not interpret
    /// these numbers in any way.
    User {
        /// Arbitrary
        namespace: u32,
        /// Arbitrary
        index: u32,
    },
    /// A test case function name of up to 10 ascii characters. This is
    /// not intended to be used outside test cases.
    TestCase {
        /// How many of the bytes in `ascii` are valid
        length: u8,
        /// Ascii bytes of the name
        ascii: [u8; TESTCASE_NAME_LENGTH],
    },
}

impl ExternalName {
    /// Creates a new external name from a sequence of bytes. Caller is expected
    /// to guarantee bytes are only ascii alphanumeric or `_`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cretonne::ir::ExternalName;
    /// // Create `ExternalName` from a string.
    /// let name = ExternalName::testcase("hello");
    /// assert_eq!(name.to_string(), "%hello");
    /// ```
    pub fn testcase<T>(v: T) -> ExternalName
    where
        T: Into<Vec<u8>>,
    {
        let vec = v.into();
        let len = if vec.len() > TESTCASE_NAME_LENGTH {
            TESTCASE_NAME_LENGTH
        } else {
            vec.len()
        };
        let mut bytes = [0u8; TESTCASE_NAME_LENGTH];
        for (i, &byte) in vec.iter().take(len).enumerate() {
            bytes[i] = byte;
        }
        ExternalName::TestCase {
            length: len as u8,
            ascii: bytes,
        }
    }

    /// Create a new external name from user-provided integer indicies.
    ///
    /// # Examples
    /// ```rust
    /// # use cretonne::ir::ExternalName;
    /// // Create `ExternalName` from integer indicies
    /// let name = ExternalName::user(123, 456);
    /// assert_eq!(name.to_string(), "u123:456");
    /// ```
    pub fn user(namespace: u32, index: u32) -> ExternalName {
        ExternalName::User {
            namespace: namespace,
            index: index,
        }
    }
}

impl Default for ExternalName {
    fn default() -> ExternalName {
        ExternalName::user(0, 0)
    }
}

impl fmt::Display for ExternalName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExternalName::User { namespace, index } => write!(f, "u{}:{}", namespace, index),
            ExternalName::TestCase { length, ascii } => {
                f.write_char('%')?;
                for byte in ascii.iter().take(length as usize) {
                    f.write_char(*byte as char)?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExternalName;

    #[test]
    fn display_testcase() {
        assert_eq!(ExternalName::testcase("").to_string(), "%");
        assert_eq!(ExternalName::testcase("x").to_string(), "%x");
        assert_eq!(ExternalName::testcase("x_1").to_string(), "%x_1");
        assert_eq!(
            ExternalName::testcase("longname12345678").to_string(),
            "%longname12345678"
        );
        // Constructor will silently drop bytes beyond the 16th
        assert_eq!(
            ExternalName::testcase("longname123456789").to_string(),
            "%longname12345678"
        );
    }

    #[test]
    fn display_user() {
        assert_eq!(ExternalName::user(0, 0).to_string(), "u0:0");
        assert_eq!(ExternalName::user(1, 1).to_string(), "u1:1");
        assert_eq!(
            ExternalName::user(::std::u32::MAX, ::std::u32::MAX).to_string(),
            "u4294967295:4294967295"
        );
    }
}
