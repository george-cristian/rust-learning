pub mod greetings {
    pub mod english;
    pub mod french;
}

#[cfg(test)]
mod tests {
    use super::greetings; // --> Note that I use super here to move up one module in order to access greetings

    #[test] // --> This marks that the following function is actually a unit test, and the Rust tester will interpret it as such
    fn test_english_greeting_correct() {
        assert_eq!("hello", greetings::english::hello());
    }

    #[test]
    #[should_panic] // --> This indicates that the following test is supposed to fail
    #[ignore] // --> This will make the test runner ignore this test
    fn test_english_greeeting_incorrect() {
        assert_eq!("salut", greetings::english::hello());
    }
}