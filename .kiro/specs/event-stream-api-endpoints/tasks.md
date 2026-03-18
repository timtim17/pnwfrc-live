# Implementation Plan: Event Stream API Endpoints

## Overview

This implementation adds 8 new route handlers to the Rust Lambda API for day-based indexing and alternate stream URLs. The implementation leverages existing data helper functions and maintains the lightweight redirect-focused architecture. Tasks are organized to build incrementally, with early validation through tests.

## Tasks

- [x] 1. Prepare data module for public access
  - Update `data.rs` to make helper functions public (`pub fn`)
  - Make `EventStream` struct and its fields public if needed by tests
  - Verify existing helper functions work as expected
  - _Requirements: 1.1, 2.1, 3.1, 4.1_

- [x] 2. Add proptest dependency
  - Add `proptest = "1.0"` to `[dev-dependencies]` in `Cargo.toml`
  - _Requirements: All (testing infrastructure)_

- [x] 3. Implement current year route handlers
  - [x] 3.1 Implement `current_stream` handler for `/:event`
    - Accept event parameter as `&str`
    - Normalize event code to lowercase
    - Call `get_current_stream(event, false)`
    - Return `Redirect` on success or `Status::NotFound` on None
    - _Requirements: 1.1, 1.2, 1.3, 5.1_
  
  - [x] 3.2 Implement `current_stream_alt` handler for `/:event/alt`
    - Set rank = 1 to match before `/:event/:day`
    - Normalize event code to lowercase
    - Call `get_current_stream(event, true)`
    - Return `Redirect` or `Status::NotFound`
    - _Requirements: 3.1, 3.3, 3.4, 5.1_
  
  - [x] 3.3 Implement `day_stream` handler for `/:event/:day`
    - Parse day as usize, return 404 if invalid or < 1
    - Convert 1-indexed to 0-indexed (day - 1)
    - Normalize event code to lowercase
    - Call `get_day_stream(event, day_index, false)`
    - Return `Redirect` or `Status::NotFound`
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 5.1_
  
  - [x] 3.4 Implement `day_stream_alt` handler for `/:event/:day/alt`
    - Set rank = 1 for proper matching priority
    - Parse and validate day parameter (same as 3.3)
    - Convert 1-indexed to 0-indexed
    - Normalize event code to lowercase
    - Call `get_day_stream(event, day_index, true)`
    - Return `Redirect` or `Status::NotFound`
    - _Requirements: 3.2, 3.3, 3.4, 5.1_

- [x] 4. Write unit tests for current year handlers
  - Test `current_stream` with valid event returns redirect
  - Test `current_stream` with invalid event returns 404
  - Test `current_stream_alt` with alt stream available
  - Test `current_stream_alt` fallback to primary when alt unavailable
  - Test `day_stream` with valid day index
  - Test `day_stream` with day < 1 returns 404
  - Test `day_stream` with day > stream count returns 404
  - Test `day_stream` with non-numeric day returns 404
  - Test `day_stream_alt` with alt stream available
  - Test case-insensitive event codes (BLK, blk, Blk)
  - _Requirements: 1.1, 1.2, 2.1, 2.3, 2.4, 3.1, 3.2, 3.3, 5.1_

- [x] 5. Checkpoint - Verify current year handlers
  - Ensure all tests pass, ask the user if questions arise.

- [x] 6. Implement year-specific route handlers
  - [x] 6.1 Implement `year_current_stream` handler for `/:year/:event`
    - Accept year and event parameters as `&str`
    - Normalize event code to lowercase
    - Call `get_current_stream_for_year(year, event, false)`
    - Return `Redirect` or `Status::NotFound`
    - _Requirements: 4.1, 4.5, 5.1_
  
  - [x] 6.2 Implement `year_current_stream_alt` handler for `/:year/:event/alt`
    - Set rank = 1 for proper matching
    - Normalize event code to lowercase
    - Call `get_current_stream_for_year(year, event, true)`
    - Return `Redirect` or `Status::NotFound`
    - _Requirements: 4.3, 4.5, 5.1_
  
  - [x] 6.3 Implement `year_day_stream` handler for `/:year/:event/:day`
    - Parse day as usize, return 404 if invalid or < 1
    - Convert 1-indexed to 0-indexed (day - 1)
    - Normalize event code to lowercase
    - Call `get_day_stream_for_year(year, event, day_index, false)`
    - Return `Redirect` or `Status::NotFound`
    - _Requirements: 4.2, 4.5, 4.6, 5.1_
  
  - [x] 6.4 Implement `year_day_stream_alt` handler for `/:year/:event/:day/alt`
    - Set rank = 1 for proper matching
    - Parse and validate day parameter (same as 6.3)
    - Convert 1-indexed to 0-indexed
    - Normalize event code to lowercase
    - Call `get_day_stream_for_year(year, event, day_index, true)`
    - Return `Redirect` or `Status::NotFound`
    - _Requirements: 4.4, 4.5, 4.6, 5.1_

- [x] 7. Write unit tests for year-specific handlers
  - Test `year_current_stream` with valid year and event
  - Test `year_current_stream` with invalid year returns 404
  - Test `year_current_stream_alt` with alt stream available
  - Test `year_day_stream` with valid parameters
  - Test `year_day_stream` with invalid year returns 404
  - Test `year_day_stream` with invalid day returns 404
  - Test `year_day_stream_alt` with valid parameters
  - Test case-insensitive event codes for year-specific routes
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 5.1_

- [x] 8. Update route registration in build_rocket()
  - Remove existing `events` and `events_specific_year` routes
  - Register all 8 new route handlers in correct order
  - Ensure routes with rank = 1 are registered appropriately
  - _Requirements: All_

- [x] 9. Checkpoint - Verify all handlers integrated
  - Ensure all tests pass, ask the user if questions arise.

- [x] 10. Create property-based test infrastructure
  - Create `tests/mod.rs` for test module setup
  - Create `tests/property_tests.rs` for property-based tests
  - Set up proptest configuration with 100 cases minimum
  - Create helper functions for generating test data
  - _Requirements: All (testing infrastructure)_

- [x] 11. Implement property tests for invalid inputs
  - [x] 11.1 Write property test for invalid event codes
    - **Property 1: Invalid Event Codes Return 404**
    - **Validates: Requirements 1.2, 2.5**
    - Generate random invalid event codes
    - Verify all endpoints return 404
  
  - [x] 11.2 Write property test for invalid day indices
    - **Property 5: Invalid Day Indices Return 404**
    - **Validates: Requirements 2.3, 2.4**
    - Generate events with N streams
    - Test day < 1 and day > N return 404
  
  - [x] 11.3 Write property test for invalid years
    - **Property 11: Invalid Year Returns 404**
    - **Validates: Requirements 4.5**
    - Generate random invalid years
    - Verify year-specific endpoints return 404

- [x] 12. Implement property tests for valid redirects
  - [x] 12.1 Write property test for current day stream redirect
    - **Property 2: Current Day Stream Redirect**
    - **Validates: Requirements 1.1, 4.1**
    - Generate events with various date configurations
    - Verify correct current day stream is selected
  
  - [x] 12.2 Write property test for specific day stream redirect
    - **Property 3: Specific Day Stream Redirect**
    - **Validates: Requirements 2.1, 4.2**
    - Generate events with N streams and valid day indices
    - Verify correct stream is returned
  
  - [x] 12.3 Write property test for day index mapping
    - **Property 4: Day Index 1-Based Mapping**
    - **Validates: Requirements 2.2**
    - Generate events with N streams
    - Verify day 1 → index 0, day 2 → index 1, etc.

- [ ] 13. Implement property tests for alternate streams
  - [x] 13.1 Write property test for current day alternate redirect
    - **Property 6: Current Day Alternate Stream Redirect**
    - **Validates: Requirements 3.1, 4.3**
    - Generate events with and without alt streams
    - Verify correct URL is returned for current day
  
  - [x] 13.2 Write property test for specific day alternate redirect
    - **Property 7: Specific Day Alternate Stream Redirect**
    - **Validates: Requirements 3.2, 4.4**
    - Generate events with and without alt streams
    - Verify correct URL is returned for specific day
  
  - [ ] 13.3 Write property test for alternate stream fallback
    - **Property 8: Alternate Stream Fallback**
    - **Validates: Requirements 3.3**
    - Generate events without alt streams
    - Verify fallback to primary stream (not 404)

- [ ] 14. Implement property tests for case insensitivity
  - [ ] 14.1 Write property test for case-insensitive alt suffix
    - **Property 9: Case-Insensitive Alt Suffix**
    - **Validates: Requirements 3.4**
    - Generate various casings of "alt" (ALT, Alt, aLt, etc.)
    - Verify all produce same redirect
  
  - [ ] 14.2 Write property test for case-insensitive event codes
    - **Property 10: Case-Insensitive Event Codes**
    - **Validates: Requirements 5.1**
    - Generate various casings of valid event codes
    - Verify all resolve to same event and redirect

- [ ] 15. Implement property test for current day calculation
  - **Property 12: Current Day Calculation**
  - **Validates: Requirements 1.4**
  - Generate events with multiple streams at various dates
  - Verify most recent non-future date is selected based on Pacific timezone
  - _Requirements: 1.4_

- [ ] 16. Final checkpoint - Verify complete implementation
  - Run all unit tests and property tests
  - Verify all 8 route handlers work correctly
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Property tests validate universal correctness across all inputs
- Unit tests validate specific examples and edge cases
- Route handlers build incrementally: current year first, then year-specific
- Checkpoints ensure validation at key milestones
