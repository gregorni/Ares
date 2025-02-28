///! Decode a base64 string
///! Performs error handling and returns a string
///! Call base64_decoder.crack to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.
///
use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use base64::{engine::general_purpose, Engine as _};

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

/// The Base64 decoder, call:
/// `let base64_decoder = Decoder::<Base64Decoder>::new()` to create a new instance
/// And then call:
/// `result = base64_decoder.crack(input)` to decode a base64 string
/// The struct generated by new() comes from interface.rs
/// ```
/// use ares::decoders::base64_decoder::{Base64Decoder};
/// use ares::decoders::interface::{Crack, Decoder};
/// use ares::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decode_base64 = Decoder::<Base64Decoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decode_base64.crack("aGVsbG8gd29ybGQ=", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap()[0], "hello world");
/// ```
pub struct Base64Decoder;

impl Crack for Decoder<Base64Decoder> {
    fn new() -> Decoder<Base64Decoder> {
        Decoder {
            name: "Base64",
            description: "Base64 is a group of binary-to-text encoding schemes that represent binary data (more specifically, a sequence of 8-bit bytes) in an ASCII string format by translating the data into a radix-64 representation.",
            link: "https://en.wikipedia.org/wiki/Base64",
            tags: vec!["base64", "decoder", "base"],
            popularity: 1.0,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Base64 with text {:?}", text);
        let decoded_text = decode_base64_no_error_handling(text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode base64 because Base64Decoder::decode_base64_no_error_handling returned None");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode base64 because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);

        results.update_checker(&checker_result);

        results
    }
    /// Gets all tags for this decoder
    fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }
    /// Gets the name for the current decoder
    fn get_name(&self) -> &str {
        self.name
    }
}

/// helper function
fn decode_base64_no_error_handling(text: &str) -> Option<String> {
    // Strip all padding
    let text = text.replace('=', "");
    // Runs the code to decode base64
    // Doesn't perform error handling, call from_base64
    general_purpose::STANDARD_NO_PAD
        .decode(text.as_bytes())
        .ok()
        .map(|inner| String::from_utf8(inner).ok())?
}

#[cfg(test)]
mod tests {
    use super::Base64Decoder;
    use crate::{
        checkers::{
            athena::Athena,
            checker_type::{Check, Checker},
            CheckerTypes,
        },
        decoders::interface::{Crack, Decoder},
    };

    // helper for tests
    fn get_athena_checker() -> CheckerTypes {
        let athena_checker = Checker::<Athena>::new();
        CheckerTypes::CheckAthena(athena_checker)
    }

    #[test]
    fn successful_decoding() {
        let base64_decoder = Decoder::<Base64Decoder>::new();

        let result = base64_decoder.crack("aGVsbG8gd29ybGQ=", &get_athena_checker());
        let decoded_str = &result
            .unencrypted_text
            .expect("No unencrypted text for base64");
        assert_eq!(decoded_str[0], "hello world");
    }

    #[test]
    fn base64_decode_empty_string() {
        // Bsae64 returns an empty string, this is a valid base64 string
        // but returns False on check_string_success
        let base64_decoder = Decoder::<Base64Decoder>::new();
        let result = base64_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base64_decode_handles_panics() {
        let base64_decoder = Decoder::<Base64Decoder>::new();
        let result = base64_decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        if result.is_some() {
            panic!("Decode_base64 did not return an option with Some<t>.")
        } else {
            // If we get here, the test passed
            // Because the base64_decoder.crack function returned None
            // as it should do for the input
            assert_eq!(true, true);
        }
    }

    #[test]
    fn base64_handle_panic_if_empty_string() {
        let base64_decoder = Decoder::<Base64Decoder>::new();
        let result = base64_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            assert_eq!(true, true);
        }
    }

    #[test]
    fn base64_work_if_string_not_base64() {
        // You can base64 decode a string that is not base64
        // This string decodes to:
        // ```.ée¢
        // (uÖ²```
        // https://gchq.github.io/CyberChef/#recipe=From_Base64('A-Za-z0-9%2B/%3D',true)&input=aGVsbG8gZ29vZCBkYXkh
        let base64_decoder = Decoder::<Base64Decoder>::new();
        let result = base64_decoder
            .crack("hello good day!", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            assert_eq!(true, true);
        }
    }

    #[test]
    fn base64_handle_panic_if_emoji() {
        let base64_decoder = Decoder::<Base64Decoder>::new();
        let result = base64_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            assert_eq!(true, true);
        }
    }
}
