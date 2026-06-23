#![allow(dead_code)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::panic)]
#![allow(clippy::missing_panics_doc)]

// #[test]
// fn caller() {
//     // Doing this because of the 100k open bracket test
//     let thread = std::thread::Builder::new()
//         .stack_size(mirl_core::constants::bytes::GB as usize);
//     let output = thread.spawn(main).unwrap();

//     let _result = output.join();
//     // match result {
//     //     Ok(()) => {}
//     //     Err(_) => {
//     //         panic!("Not gud")
//     //     }
//     // }
// }

use crate::{prelude::*, values::PositionedValue};
#[test]
/// Test the codecs
pub fn main() {
    println!("Testing {}", DefaultJson::NAME);
    // simple_test_json();
    full_test_json();
}
/// Make a full json test
#[allow(clippy::too_many_lines)]
pub fn full_test_json() {
    use rayon::{ThreadPoolBuilder, prelude::*};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Crashability {
        ShouldSucceed,
        ShouldFail,
        EitherOkay,
    }

    fn get_crashability(name: &str) -> Crashability {
        let id: Vec<&str> = name.split('_').collect();
        match id[0] {
            "y" => Crashability::ShouldSucceed,
            "n" => Crashability::ShouldFail,
            "i" => Crashability::EitherOkay,
            _ => panic!("Invalid name: {name}"),
        }
    }

    let fol = "../.././.tests/mirl_codec_info/json/full_scale";

    let items: Vec<_> = std::fs::read_dir(fol)
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
    let crashability: Vec<Crashability> = items
        .iter()
        .map(|x| get_crashability(&x.file_name().to_string_lossy()))
        .collect();

    let final_list: Vec<(&std::fs::DirEntry, Crashability)> = items
        .iter()
        .zip(crashability)
        .collect::<Vec<(&std::fs::DirEntry, Crashability)>>()
        .clone();

    let name_list: Vec<String> = final_list
        .iter()
        .map(|x| x.0.file_name().to_string_lossy().to_string())
        .collect();

    assert!(!items.is_empty(), "No items found for full test at {fol}");

    // Create a thread pool with larger stack size and custom thread names
    let thread_pool = ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        // .stack_size(mirl_core::constants::bytes::MB as usize * 32) // 8 MB stack instead of default 2 MB
        .thread_name(move |idx| format!("json-parser-{}", name_list[idx]))
        .build()
        .unwrap();

    // Process files in parallel with the custom thread pool
    let test_results: Vec<TestResult> = thread_pool.install(|| {
        final_list
            .par_iter()
            .map(|(name, crash)| {
                let path = name.path();
                let path_str = path.to_string_lossy().to_string();
                // if path_str != r"../.././.tests/mirl_codec_info/json/full_scale\i_number_neg_int_huge_exp.json"{
                //     return TestResult { path: path_str, is_error: false, message: String::new() ,value:PositionedValue::default()}
                // }
                println!("#>{path_str}");

                let value: Result<
                    (),
                    Result<Option<PositionedValue>, crate::error::CodecError>,
                > = {
                    match std::fs::read(&path) {
                        Ok(bytes) => {
                            let file = decode_file_bytes(&bytes);

                            if file.is_empty() {
                                return TestResult {
                                    path: path_str,
                                    is_error: false,
                                    message: "File is empty, skipped".to_string(),value:PositionedValue::default()
                                };
                            }

                            Err(crate::from_str::<DefaultJson>(&file))
                        }
                        Err(_) => Err(Err(crate::error::CodecError::Unknown)),
                    }
                };

                match value.unwrap_err() {
                    Ok(val) => {
                        // Successfully parsed
                        if *crash == Crashability::ShouldFail {
                            TestResult {
                                message: format!("Parsed {path_str} when it should have failed"),
                                path: path_str,
                                is_error: true,
                                value: val.unwrap_or_default()
                            }
                        } else {
                            TestResult {
                                message: format!("Successfully parsed {path_str}: {val:?}"),
                                path: path_str,
                                is_error: false,
                                value: val.unwrap_or_default()
                            }
                        }
                    }
                    Err(val) => {
                        // Failed to parse
                        if *crash == Crashability::ShouldFail {
                            TestResult {
                                message: format!("Successfully failed parsing {path_str}"),
                                path: path_str,
                                is_error: false,
                                value: PositionedValue::default()
                            }
                        } else {
                            TestResult {
                                message: format!(
                                    "Failed Parsing when it should have succeeded:\n\"{path_str}\": {val}"
                                ),
                                path: path_str,
                                is_error: true,
                                value: PositionedValue::default()
                            }
                        }
                    }
                }
            })
            .collect()
    });

    // Collect errors and print results sequentially
    let errors: Vec<String> = test_results
        .iter()
        .filter_map(|result| {
            if result.is_error {
                println!("> {}", result.message);
                Some(result.message.clone())
            } else {
                println!("{}", result.message);
                None
            }
        })
        .collect();

    assert!(
        errors.is_empty(),
        "{:#?}",
        errors.iter().map(|x| if x.chars().count() < 500 {
            x
        } else {
            &x[0..500]
        })
    );
}
/// The result after a test
#[derive(Debug, Clone)]
pub struct TestResult {
    /// Path of the test
    pub path: String,
    /// If an error occurred
    pub is_error: bool,
    /// Info
    pub message: String,
    /// The resulting value
    pub value: PositionedValue,
}
/// Test simple json cases
#[allow(clippy::panic)]
#[allow(clippy::missing_panics_doc)]
pub fn simple_test_json() {
    fn get_id(name: &str) -> usize {
        let id: Vec<&str> = name.split('_').collect();
        id.first()
            .map_or_else(|| 0, |x| x.parse().unwrap_or_default())
    }
    let fol = "../.././.tests/mirl_codec_info/json/simple";

    let mut items: Vec<std::fs::DirEntry> = std::fs::read_dir(fol)
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    assert!(!items.is_empty(), "No items found for simple test at {fol}");
    items.sort_by_key(|x| get_id(&x.file_name().to_string_lossy()));

    for i in items {
        let path = format!("{fol}/{}", i.file_name().to_string_lossy());
        let file = std::fs::read_to_string(&path).unwrap();
        let Some(original_value) = test_parse_json(&file, Some(&i.file_name().to_string_lossy()))
        else {
            println!("{path} is empty");
            continue;
        };
        let string = test_marshal::<DefaultJson>(&original_value, &i.file_name().to_string_lossy());
        let value = test_parse_json(&string, None).unwrap();
        let another_string = test_marshal::<DefaultJson>(&value, &i.file_name().to_string_lossy());
        let another_value = test_parse_json(&another_string, None).unwrap();
        assert!(
            string == another_string,
            "Strings don't match: '{string}' and '{another_string}'"
        );
        assert!(
            value == another_value,
            "Values don't match: \n'{value:?}'\nand\n'{another_value:?}'"
        );
    }
}
#[must_use]
/// Test the json parser on a single test
pub fn test_parse_json(file: &str, name: Option<&str>) -> Option<PositionedValue> {
    let value = crate::from_str::<DefaultJson>(file);

    match value {
        Ok(val) => {
            // if file.chars().count() < 1000 {
            //     println!("{val:#?}");
            // }
            val
        }
        Err(val) => name.map_or_else(
            || panic!("Error at {val} while testing: {file}"),
            |name| panic!("Error while testing:\n{name}:{val}"),
        ),
    }
}
#[must_use]
/// Marshal the value back into text
pub fn test_marshal<T: StaticCompactMarshal>(file: &PositionedValue, _name: &str) -> String {
    T::to_compact_string(file, 0).unwrap()
}
#[must_use]
/// Decode the file bytes into valid utf-8 if it isn't
#[allow(clippy::similar_names)]
pub fn decode_file_bytes(bytes: &[u8]) -> String {
    // Check for UTF-8 BOM first
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF])
        && let Ok(s) = std::str::from_utf8(&bytes[3..])
    {
        return s.to_string();
    }

    // Check for UTF-16 LE BOM
    if bytes.len() >= 2 && bytes.starts_with(&[0xFF, 0xFE]) {
        return decode_utf16_lossy(&bytes[2..], false); // LE
    }

    // Check for UTF-16 BE BOM
    if bytes.len() >= 2 && bytes.starts_with(&[0xFE, 0xFF]) {
        return decode_utf16_lossy(&bytes[2..], true); // BE
    }

    // Try strict UTF-8
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }

    // Heuristic: try UTF-16 only if length is even
    if bytes.len() >= 2 && bytes.len().is_multiple_of(2) {
        let (decoded_le, count_le) = decode_utf16_with_count(bytes, false);
        let (decoded_be, count_be) = decode_utf16_with_count(bytes, true);

        // Accept only if replacement rate is reasonable (< 10%, for example)
        let threshold = bytes.len() / 20; // 5% threshold
        if count_le < threshold && count_le <= count_be {
            return decoded_le;
        }
        if count_be < threshold {
            return decoded_be;
        }
    }

    // Last resort: lossy UTF-8
    String::from_utf8_lossy(bytes).to_string()
}
#[must_use]
/// Decode utf-16
pub fn decode_utf16_lossy(bytes: &[u8], big_endian: bool) -> String {
    String::from_utf16_lossy(
        &bytes
            .chunks_exact(2)
            .map(|c| {
                if big_endian {
                    u16::from_be_bytes([c[0], c[1]])
                } else {
                    u16::from_le_bytes([c[0], c[1]])
                }
            })
            .collect::<Vec<_>>(),
    )
}
#[must_use]
/// Decode utf-16 and count the characters
pub fn decode_utf16_with_count(bytes: &[u8], big_endian: bool) -> (String, usize) {
    let utf16_units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| {
            if big_endian {
                u16::from_be_bytes([c[0], c[1]])
            } else {
                u16::from_le_bytes([c[0], c[1]])
            }
        })
        .collect();

    let decoded = String::from_utf16_lossy(&utf16_units);
    let count = decoded.chars().filter(|c| *c == '\u{FFFD}').count();
    (decoded, count)
}
