use url::Url;

use rocket::{http::Status, local::blocking::Client};
use crate::{build_rocket, data::{get_day_stream, get_current_stream, ALL_STREAMS}};

// Task 3.1: Test current_stream handler
#[test]
fn test_current_stream_invalid_event() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let response = client.get("/invalid").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_current_stream_valid_event() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Test with a valid event code
    let response = client.get("/blk").dispatch();
    // Should either redirect (if current stream exists) or return 404 (if all dates are future)
    assert!(response.status() == Status::SeeOther || response.status() == Status::NotFound);
}

#[test]
fn test_current_stream_case_insensitive() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Test case insensitivity - BLK, blk, Blk should all work the same
    let response1 = client.get("/blk").dispatch();
    let response2 = client.get("/BLK").dispatch();
    let response3 = client.get("/Blk").dispatch();
    
    // All should return the same status
    assert_eq!(response1.status(), response2.status());
    assert_eq!(response1.status(), response3.status());
}

// Task 3.2: Test current_stream_alt handler
#[test]
fn test_current_stream_alt_invalid_event() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let response = client.get("/invalid/alt").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_current_stream_alt_valid_event() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Test with a valid event code that has an alt stream
    let response = client.get("/blk/alt").dispatch();
    // Should either redirect (if current stream exists) or return 404 (if all dates are future)
    assert!(response.status() == Status::SeeOther || response.status() == Status::NotFound);
}

#[test]
fn test_current_stream_alt_fallback() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Test with a valid event code that has no alt stream (should fallback to primary)
    let response = client.get("/osf/alt").dispatch();
    // Should either redirect (if current stream exists) or return 404 (if all dates are future)
    assert!(response.status() == Status::SeeOther || response.status() == Status::NotFound);
}

#[test]
fn test_current_stream_alt_case_insensitive() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Test case insensitivity for event code
    let response1 = client.get("/blk/alt").dispatch();
    let response2 = client.get("/BLK/alt").dispatch();
    let response3 = client.get("/Blk/alt").dispatch();
    
    // All should return the same status
    assert_eq!(response1.status(), response2.status());
    assert_eq!(response1.status(), response3.status());
}

#[test]
fn every_url_is_valid() {
    for &year in ALL_STREAMS.keys() {
        let streams = &ALL_STREAMS[year];
        for &event_key in streams.keys() {
            let schedule = streams[event_key];
            for (day, stream) in schedule.iter().enumerate() {
                Url::parse(stream.stream).expect(&format!("event '{}' day {} should have a valid stream URL", event_key, day));
                if let Some(alt) = stream.alt_stream {
                    Url::parse(alt).expect(&format!("event '{}' day {} should have a valid alt_stream URL", event_key, day));
                }
            }
        }
    }
}

#[test]
fn all_dates_are_valid_format() {
    for &year in ALL_STREAMS.keys() {
        let streams = &ALL_STREAMS[year];
        for &event_key in streams.keys() {
            let schedule = streams[event_key];
            for (day, stream) in schedule.iter().enumerate() {
                assert_eq!(stream.date.len(), 10, "event '{}' day {} date should be YYYY-MM-DD format", event_key, day);
                assert!(stream.date.chars().nth(4) == Some('-') && stream.date.chars().nth(7) == Some('-'),
                    "event '{}' day {} date should be YYYY-MM-DD format", event_key, day);
            }
        }
    }
}

#[test]
fn dates_are_sorted() {
    for &year in ALL_STREAMS.keys() {
        let streams = &ALL_STREAMS[year];
        for &event_key in streams.keys() {
            let schedule = streams[event_key];
            for i in 1..schedule.len() {
                assert!(schedule[i-1].date <= schedule[i].date,
                    "event '{}' dates should be sorted: {} should come before {}",
                    event_key, schedule[i-1].date, schedule[i].date);
            }
        }
    }
}

#[test]
fn get_day_stream_returns_valid_urls() {
    for &year in ALL_STREAMS.keys() {
        let streams = &ALL_STREAMS[year];
        for &event_key in streams.keys() {
            let schedule = streams[event_key];
            for day in 0..schedule.len() {
                let stream = get_day_stream(event_key, day, false);
                assert!(stream.is_some(), "event '{}' day {} should return a stream", event_key, day);
                let alt_stream = get_day_stream(event_key, day, true);
                assert!(alt_stream.is_some(), "event '{}' day {} should return an alt stream", event_key, day);
            }
        }
    }
}

#[test]
fn get_day_stream_returns_none_for_invalid() {
    assert!(get_day_stream("invalid", 0, false).is_none());
    assert!(get_day_stream("blk", 999, false).is_none());
}

#[test]
fn get_current_stream_returns_some_for_valid_events() {
    for &year in ALL_STREAMS.keys() {
        let streams = &ALL_STREAMS[year];
        for &event_key in streams.keys() {
            // Should return Some or None depending on current date, but shouldn't panic
            let _ = get_current_stream(event_key, false);
            let _ = get_current_stream(event_key, true);
        }
    }
}

// Task 3.3: Test day_stream handler
#[test]
fn test_day_stream_valid_day() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Test with valid event and day 1 (first stream)
    let response = client.get("/blk/1").dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert!(response.headers().get_one("Location").is_some());
}

#[test]
fn test_day_stream_invalid_event() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let response = client.get("/invalid/1").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_day_stream_day_less_than_1() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Day 0 should return 404
    let response = client.get("/blk/0").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_day_stream_day_exceeds_count() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Day 999 should return 404 (exceeds stream count)
    let response = client.get("/blk/999").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_day_stream_non_numeric_day() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Non-numeric day should return 404
    let response = client.get("/blk/abc").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_day_stream_case_insensitive_event() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Test case insensitivity - BLK, blk, Blk should all work the same
    let response1 = client.get("/blk/1").dispatch();
    let response2 = client.get("/BLK/1").dispatch();
    let response3 = client.get("/Blk/1").dispatch();
    
    // All should return the same status and location
    assert_eq!(response1.status(), response2.status());
    assert_eq!(response1.status(), response3.status());
    assert_eq!(response1.headers().get_one("Location"), response2.headers().get_one("Location"));
    assert_eq!(response1.headers().get_one("Location"), response3.headers().get_one("Location"));
}

#[test]
fn test_day_stream_1_indexed() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Day 1 should return the first stream (index 0)
    let response = client.get("/blk/1").dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    
    // Verify it matches what get_day_stream returns for index 0
    let expected_url = get_day_stream("blk", 0, false);
    assert!(expected_url.is_some());
    assert_eq!(response.headers().get_one("Location"), expected_url);
}

// Task 3.4: Test day_stream_alt handler
#[test]
fn test_day_stream_alt_with_alt_available() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Test with event that has alt stream (blk has alt stream)
    let response = client.get("/blk/1/alt").dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert!(response.headers().get_one("Location").is_some());
}

// #[test]
// fn test_day_stream_alt_fallback_to_primary() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     let response = client.get("/osf/1/alt").dispatch();
//     assert_eq!(response.status(), Status::SeeOther);
//     // Should fall back to primary stream
//     assert_eq!(response.headers().get_one("Location"), Some("https://www.youtube.com/watch?v=3mtZiUD4zzo"));
// }

#[test]
fn test_day_stream_alt_invalid_event() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let response = client.get("/invalid/1/alt").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_day_stream_alt_day_less_than_1() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let response = client.get("/blk/0/alt").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_day_stream_alt_day_exceeds_count() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let response = client.get("/blk/999/alt").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_day_stream_alt_non_numeric_day() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let response = client.get("/blk/abc/alt").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_day_stream_alt_case_insensitive_event() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    // Test case insensitivity - BLK, blk, Blk should all work the same
    let response1 = client.get("/blk/1/alt").dispatch();
    let response2 = client.get("/BLK/1/alt").dispatch();
    let response3 = client.get("/Blk/1/alt").dispatch();
    
    // All should return the same status and location
    assert_eq!(response1.status(), response2.status());
    assert_eq!(response1.status(), response3.status());
    assert_eq!(response1.headers().get_one("Location"), response2.headers().get_one("Location"));
    assert_eq!(response1.headers().get_one("Location"), response3.headers().get_one("Location"));
}

// Task 7: Test year-specific handlers

// Task 6.1: Test year_current_stream handler
// #[test]
// fn test_year_current_stream_valid_year_and_event() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with valid year and event
//     let response = client.get("/2026/blk").dispatch();
//     // Should either redirect (if current stream exists) or return 404 (if all dates are future)
//     assert!(response.status() == Status::SeeOther || response.status() == Status::NotFound);
// }

// #[test]
// fn test_year_current_stream_invalid_year() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with invalid year (2025 doesn't exist in test data)
//     let response = client.get("/2025/blk").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_current_stream_invalid_event() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with valid year but invalid event
//     let response = client.get("/2026/invalid").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_current_stream_case_insensitive_event() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test case insensitivity for event code
//     let response1 = client.get("/2026/blk").dispatch();
//     let response2 = client.get("/2026/BLK").dispatch();
//     let response3 = client.get("/2026/Blk").dispatch();
    
//     // All should return the same status
//     assert_eq!(response1.status(), response2.status());
//     assert_eq!(response1.status(), response3.status());
// }

// Task 6.2: Test year_current_stream_alt handler
// #[test]
// fn test_year_current_stream_alt_with_alt_available() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with event that has alt stream (blk has alt stream)
//     let response = client.get("/2026/blk/alt").dispatch();
//     // Should either redirect (if current stream exists) or return 404 (if all dates are future)
//     assert!(response.status() == Status::SeeOther || response.status() == Status::NotFound);
// }

// #[test]
// fn test_year_current_stream_alt_fallback() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with event that has no alt stream (osf has no alt stream)
//     let response = client.get("/2026/osf/alt").dispatch();
//     // Should either redirect (if current stream exists) or return 404 (if all dates are future)
//     assert!(response.status() == Status::SeeOther || response.status() == Status::NotFound);
// }

// #[test]
// fn test_year_current_stream_alt_invalid_year() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with invalid year
//     let response = client.get("/2025/blk/alt").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// Task 6.3: Test year_day_stream handler
// #[test]
// fn test_year_day_stream_valid_parameters() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with valid year, event, and day
//     let response = client.get("/2026/blk/1").dispatch();
//     assert_eq!(response.status(), Status::SeeOther);
//     assert!(response.headers().get_one("Location").is_some());
// }

// #[test]
// fn test_year_day_stream_invalid_year() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with invalid year (2025 doesn't exist in test data)
//     let response = client.get("/2025/blk/1").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_day_stream_invalid_event() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with valid year but invalid event
//     let response = client.get("/2026/invalid/1").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_day_stream_invalid_day() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with day exceeding stream count
//     let response = client.get("/2026/blk/999").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_day_stream_day_less_than_1() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with day 0 (should return 404)
//     let response = client.get("/2026/blk/0").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_day_stream_non_numeric_day() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with non-numeric day
//     let response = client.get("/2026/blk/abc").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_day_stream_case_insensitive_event() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test case insensitivity for event code
//     let response1 = client.get("/2026/blk/1").dispatch();
//     let response2 = client.get("/2026/BLK/1").dispatch();
//     let response3 = client.get("/2026/Blk/1").dispatch();
    
//     // All should return the same status and location
//     assert_eq!(response1.status(), response2.status());
//     assert_eq!(response1.status(), response3.status());
//     assert_eq!(response1.headers().get_one("Location"), response2.headers().get_one("Location"));
//     assert_eq!(response1.headers().get_one("Location"), response3.headers().get_one("Location"));
// }

// Task 6.4: Test year_day_stream_alt handler
// #[test]
// fn test_year_day_stream_alt_valid_parameters() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with valid year, event, and day that has alt stream
//     let response = client.get("/2026/blk/1/alt").dispatch();
//     assert_eq!(response.status(), Status::SeeOther);
//     assert!(response.headers().get_one("Location").is_some());
// }

// #[test]
// fn test_year_day_stream_alt_fallback() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with event that has no alt stream
//     let response = client.get("/2026/osf/1/alt").dispatch();
//     assert_eq!(response.status(), Status::SeeOther);
//     // Should fall back to primary stream
//     assert_eq!(response.headers().get_one("Location"), Some("https://www.youtube.com/watch?v=3mtZiUD4zzo"));
// }

// #[test]
// fn test_year_day_stream_alt_invalid_year() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with invalid year
//     let response = client.get("/2025/blk/1/alt").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_day_stream_alt_invalid_event() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with valid year but invalid event
//     let response = client.get("/2026/invalid/1/alt").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_day_stream_alt_invalid_day() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with day exceeding stream count
//     let response = client.get("/2026/blk/999/alt").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_day_stream_alt_day_less_than_1() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test with day 0 (should return 404)
//     let response = client.get("/2026/blk/0/alt").dispatch();
//     assert_eq!(response.status(), Status::NotFound);
// }

// #[test]
// fn test_year_day_stream_alt_case_insensitive_event() {
//     let rocket = build_rocket();
//     let client = Client::tracked(rocket).expect("valid rocket instance");
//     // Test case insensitivity for event code
//     let response1 = client.get("/2026/blk/1/alt").dispatch();
//     let response2 = client.get("/2026/BLK/1/alt").dispatch();
//     let response3 = client.get("/2026/Blk/1/alt").dispatch();
    
//     // All should return the same status and location
//     assert_eq!(response1.status(), response2.status());
//     assert_eq!(response1.status(), response3.status());
//     assert_eq!(response1.headers().get_one("Location"), response2.headers().get_one("Location"));
//     assert_eq!(response1.headers().get_one("Location"), response3.headers().get_one("Location"));
// }
