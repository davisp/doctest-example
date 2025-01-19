//! foo for your health
//!
//! ```rust
//! println!("This is a doctest function.");
//! ```
//!
//! ```rust,{source="doctests::public_fun"}
//! ```
//!
//! ```rust,{source="doctests::private_fun"}
//! ```
//!
//! ```rust,{source="doctests::test_fun"}
//! ```

pub fn foo() {
    println!("Hello, world!");
}

#[cfg(any(doc, test))]
mod doctests {
    pub fn public_fun() {
        println!("This is a public function.");
    }

    fn private_fun() {
        println!("This is a private function.");
        let val = 1 + 5;
        assert_eq!(val, 6);
    }

    #[test]
    fn test_fun() {
        println!("This is a test function.");
    }
}
