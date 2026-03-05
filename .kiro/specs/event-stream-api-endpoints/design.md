# Design Document: Event Stream API Endpoints

## Overview

This design extends the existing Rust Lambda API to support day-based indexing and alternate stream URLs for FRC event streams. The API currently provides basic year/event lookups that redirect to YouTube URLs. This enhancement adds eight new route handlers to support:

- Current day stream access (with and without year specification)
- Specific day stream access via 1-indexed day numbers
- Alternate stream URLs via `/alt` suffix
- Case-insensitive event codes and stream selectors

The design leverages existing data helper functions (`get_current_stream`, `get_current_stream_for_year`, `get_day_stream`, `get_day_stream_for_year`) and maintains the lightweight, redirect-focused architecture of the current API.

## Architecture

### Route Handler Structure

The API will implement eight new route handlers following Rocket's attribute-based routing:

```
/:event                          -> current day, primary stream, current year
/:event/alt                      -> current day, alternate stream, current year
/:event/:day                     -> specific day, primary stream, current year
/:event/:day/alt                 -> specific day, alternate stream, current year
/:year/:event                    -> current day, primary stream, specific year
/:year/:event/alt                -> current day, alternate stream, specific year
/:year/:event/:day               -> specific day, primary stream, specific year
/:year/:event/:day/alt           -> specific day, alternate stream, specific year
```

### Route Priority and Disambiguation

Rocket processes routes in order of specificity. The challenge is distinguishing between:
- `/:event/:day` (where day is numeric)
- `/:event/alt` (where "alt" is a literal string)
- `/:year/:event` (where year is numeric)

**Solution**: Use Rocket's route guards and ordering:
1. Literal routes (`/:event/alt`) are registered before parameterized routes
2. Numeric validation happens in route guards or handler logic
3. Year routes are distinguished by checking if the first segment is a 4-digit number

### Data Flow

```
HTTP Request
    ↓
Route Handler (normalize case, parse parameters)
    ↓
Data Helper Function (get_current_stream* or get_day_stream*)
    ↓
Option<&'static str> (stream URL or None)
    ↓
HTTP Response (Redirect 302 or 404)
```

## Components and Interfaces

### Route Handlers

All route handlers follow this pattern:
- Accept path parameters as `&str`
- Normalize event codes to lowercase
- Normalize "alt" suffix to lowercase (case-insensitive)
- Convert 1-indexed day to 0-indexed for data layer
- Call appropriate data helper function
- Return `Result<Redirect, Status>`

#### Handler: `current_stream`
```rust
#[get("/<event>")]
fn current_stream(event: &str) -> Result<Redirect, Status>
```
- Normalizes event code to lowercase
- Calls `get_current_stream(event, false)`
- Returns 302 redirect or 404

#### Handler: `current_stream_alt`
```rust
#[get("/<event>/alt", rank = 1)]
fn current_stream_alt(event: &str) -> Result<Redirect, Status>
```
- Normalizes event code to lowercase
- Calls `get_current_stream(event, true)`
- Returns 302 redirect or 404
- Rank 1 ensures it matches before `/:event/:day`

#### Handler: `day_stream`
```rust
#[get("/<event>/<day>")]
fn day_stream(event: &str, day: &str) -> Result<Redirect, Status>
```
- Normalizes event code to lowercase
- Parses day as usize, returns 404 if invalid
- Converts 1-indexed to 0-indexed (day - 1)
- Returns 404 if day < 1
- Calls `get_day_stream(event, day_index, false)`
- Returns 302 redirect or 404

#### Handler: `day_stream_alt`
```rust
#[get("/<event>/<day>/alt", rank = 1)]
fn day_stream_alt(event: &str, day: &str) -> Result<Redirect, Status>
```
- Same as `day_stream` but calls with `alt = true`
- Rank 1 ensures proper matching priority

#### Handler: `year_current_stream`
```rust
#[get("/<year>/<event>")]
fn year_current_stream(year: &str, event: &str) -> Result<Redirect, Status>
```
- Normalizes event code to lowercase
- Calls `get_current_stream_for_year(year, event, false)`
- Returns 302 redirect or 404

#### Handler: `year_current_stream_alt`
```rust
#[get("/<year>/<event>/alt", rank = 1)]
fn year_current_stream_alt(year: &str, event: &str) -> Result<Redirect, Status>
```
- Same as `year_current_stream` but calls with `alt = true`

#### Handler: `year_day_stream`
```rust
#[get("/<year>/<event>/<day>")]
fn year_day_stream(year: &str, event: &str, day: &str) -> Result<Redirect, Status>
```
- Normalizes event code to lowercase
- Parses day as usize, returns 404 if invalid
- Converts 1-indexed to 0-indexed (day - 1)
- Returns 404 if day < 1
- Calls `get_day_stream_for_year(year, event, day_index, false)`
- Returns 302 redirect or 404

#### Handler: `year_day_stream_alt`
```rust
#[get("/<year>/<event>/<day>/alt", rank = 1)]
fn year_day_stream_alt(year: &str, event: &str, day: &str) -> Result<Redirect, Status>
```
- Same as `year_day_stream` but calls with `alt = true`

### Integration with Existing Code

The design maintains compatibility with existing code:
- Existing routes (`/`, `/<year>/<path>`, `/<path>`) remain unchanged
- Data helper functions are used as-is (no modifications needed)
- The `EventStream` structure with `alt_stream: Option<&'static str>` is unchanged
- Fallback logic (alternate to primary) is handled by data layer

### Removed Components

The existing routes `events` and `events_specific_year` in main.rs will be replaced by the new route handlers, as they appear to be incomplete implementations that don't match the requirements.

## Data Models

### Existing Data Structures (No Changes)

```rust
struct EventStream {
    date: &'static str,
    stream: &'static str,
    alt_stream: Option<&'static str>,
}

type Event = &'static [EventStream];
type EventYear = phf::Map<&'static str, Event>;
```

### Day Index Conversion

The API exposes 1-indexed days (user-friendly), but the data layer uses 0-indexed arrays (Rust convention):

- User requests `/blk/1` → handler converts to index 0
- User requests `/blk/2` → handler converts to index 1
- User requests `/blk/0` → handler returns 404 (invalid)

This conversion happens in route handlers before calling data functions.

### Current Day Logic

The data layer determines "current day" as:
- The most recent EventStream where `date <= today` (Pacific timezone)
- Implemented by `get_current_stream_for_year` via reverse iteration

Route handlers don't need to implement this logic; they simply call the appropriate data function.


## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Invalid Event Codes Return 404

*For any* request path containing an event code that does not exist in the data, the API should return HTTP 404.

**Validates: Requirements 1.2, 2.5**

### Property 2: Current Day Stream Redirect

*For any* valid event code and year (current or specified), when requesting the current day stream, the API should redirect to the primary stream URL for the most recent non-future date in Pacific timezone.

**Validates: Requirements 1.1, 4.1**

### Property 3: Specific Day Stream Redirect

*For any* valid event code, year (current or specified), and valid day index, when requesting a specific day stream, the API should redirect to the primary stream URL for that day.

**Validates: Requirements 2.1, 4.2**

### Property 4: Day Index 1-Based Mapping

*For any* event with N streams, requesting day 1 should return the first stream (index 0), day 2 should return the second stream (index 1), and day N should return the Nth stream (index N-1).

**Validates: Requirements 2.2**

### Property 5: Invalid Day Indices Return 404

*For any* event with N streams, requesting a day index less than 1 or greater than N should return HTTP 404.

**Validates: Requirements 2.3, 2.4**

### Property 6: Current Day Alternate Stream Redirect

*For any* valid event code and year (current or specified), when requesting the current day alternate stream via `/alt` suffix, the API should redirect to the alternate stream URL if available, or fall back to the primary stream URL.

**Validates: Requirements 3.1, 4.3**

### Property 7: Specific Day Alternate Stream Redirect

*For any* valid event code, year (current or specified), and valid day index, when requesting a specific day alternate stream via `/:day/alt` suffix, the API should redirect to the alternate stream URL if available, or fall back to the primary stream URL.

**Validates: Requirements 3.2, 4.4**

### Property 8: Alternate Stream Fallback

*For any* request with `/alt` suffix where the alternate stream is not available, the API should redirect to the primary stream URL instead of returning an error.

**Validates: Requirements 3.3**

### Property 9: Case-Insensitive Alt Suffix

*For any* valid request path, the stream selector "alt" should be case-insensitive, such that "/alt", "/ALT", and "/Alt" all produce the same redirect.

**Validates: Requirements 3.4**

### Property 10: Case-Insensitive Event Codes

*For any* valid event code, requests with different capitalizations (e.g., "blk", "BLK", "Blk") should all resolve to the same event and produce the same redirect.

**Validates: Requirements 5.1**

### Property 11: Invalid Year Returns 404

*For any* year that does not exist in the data, requests to year-specific endpoints should return HTTP 404.

**Validates: Requirements 4.5**

### Property 12: Current Day Calculation

*For any* event with multiple streams, the current day should be determined as the most recent stream date that is not in the future, based on Pacific timezone.

**Validates: Requirements 1.4**

## Error Handling

### HTTP Status Codes

The API uses two status codes:
- **302 Found**: Successful redirect to stream URL
- **404 Not Found**: Resource not found (invalid event, year, or day)

### Error Conditions

1. **Invalid Event Code**: Return 404
   - Event code not found in data for the specified year
   - Applies to all endpoints

2. **Invalid Year**: Return 404
   - Year not found in ALL_STREAMS map
   - Applies to year-specific endpoints only

3. **Invalid Day Index**: Return 404
   - Day < 1 (user provided 0 or negative)
   - Day > number of streams for event
   - Day is not a valid integer

4. **No Current Stream**: Return 404
   - All stream dates are in the future
   - Applies to current day endpoints only

5. **Malformed Request**: Handled by Rocket framework
   - Returns 404 for unmatched routes
   - No custom handling needed

### Error Handling Strategy

- All errors result in 404 (no error messages or JSON responses)
- This maintains the lightweight redirect-only API design
- Errors are logged by Rocket framework for debugging
- No custom error types or error handling middleware needed

### Alternate Stream Fallback

When an alternate stream is requested but not available:
- **Not an error condition**
- API falls back to primary stream URL
- Returns 302 redirect (not 404)
- Fallback logic is handled by data layer helper functions

## Testing Strategy

### Dual Testing Approach

This feature requires both unit tests and property-based tests for comprehensive coverage:

- **Unit tests**: Verify specific examples, edge cases, and error conditions
- **Property tests**: Verify universal properties across all inputs

Both testing approaches are complementary and necessary. Unit tests catch concrete bugs and validate specific scenarios, while property tests verify general correctness across a wide range of inputs.

### Unit Testing

Unit tests should focus on:

1. **Specific Examples**
   - Request `/blk` returns expected stream for current day
   - Request `/2026/osf/1` returns first stream for OSF 2026
   - Request `/blk/alt` with no alt stream falls back to primary

2. **Edge Cases**
   - Event with only future dates returns 404
   - Day index 0 returns 404
   - Day index exceeding stream count returns 404
   - Empty event code handling

3. **Integration Points**
   - Route handlers correctly call data helper functions
   - Case normalization happens before data lookup
   - Day index conversion (1-indexed to 0-indexed) is correct

4. **Error Conditions**
   - Invalid event codes return 404
   - Invalid years return 404
   - Non-numeric day values return 404

### Property-Based Testing

Property-based testing will use the **proptest** crate for Rust. Each property test should:
- Run a minimum of 100 iterations
- Reference the corresponding design document property
- Use the tag format: **Feature: event-stream-api-endpoints, Property N: [property text]**

Property tests should focus on:

1. **Property 1: Invalid Event Codes Return 404**
   - Generate random invalid event codes
   - Verify all return 404

2. **Property 2: Current Day Stream Redirect**
   - Generate events with various date configurations
   - Verify correct current day stream is selected

3. **Property 3: Specific Day Stream Redirect**
   - Generate events with N streams
   - Generate valid day indices (1 to N)
   - Verify correct stream is returned

4. **Property 4: Day Index 1-Based Mapping**
   - Generate events with N streams
   - Verify day 1 maps to index 0, day 2 to index 1, etc.

5. **Property 5: Invalid Day Indices Return 404**
   - Generate events with N streams
   - Test with day < 1 and day > N
   - Verify all return 404

6. **Property 6-7: Alternate Stream Redirects**
   - Generate events with and without alt streams
   - Verify correct URL is returned

7. **Property 8: Alternate Stream Fallback**
   - Generate events without alt streams
   - Verify fallback to primary stream

8. **Property 9: Case-Insensitive Alt Suffix**
   - Generate various casings of "alt"
   - Verify all produce same result

9. **Property 10: Case-Insensitive Event Codes**
   - Generate various casings of event codes
   - Verify all resolve to same event

10. **Property 11: Invalid Year Returns 404**
    - Generate random invalid years
    - Verify all return 404

11. **Property 12: Current Day Calculation**
    - Generate events with various date configurations
    - Verify most recent non-future date is selected

### Test Configuration

```rust
// Example property test configuration
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    // Feature: event-stream-api-endpoints, Property 1: Invalid Event Codes Return 404
    #[test]
    fn invalid_event_codes_return_404(event_code in "[a-z]{3,10}") {
        // Test implementation
    }
}
```

### Testing Challenges

1. **Static Data**: The data layer uses static data structures, making it difficult to inject test data
   - Solution: Use existing test data in ALL_STREAMS for property tests
   - Solution: Add test-specific events to the data structure for comprehensive testing

2. **Time-Dependent Logic**: Current day calculation depends on Pacific timezone
   - Solution: Property tests should use relative dates (past, present, future)
   - Solution: Unit tests can use specific dates from test data

3. **HTTP Testing**: Testing Rocket routes requires test client setup
   - Solution: Use Rocket's `local::blocking::Client` for synchronous tests
   - Solution: Test both route handlers and data functions independently

### Test Organization

```
lambda/
  src/
    main.rs          (route handlers)
    data.rs          (data helper functions)
    tests/
      mod.rs         (test module setup)
      unit_tests.rs  (unit tests for specific examples)
      property_tests.rs (property-based tests)
```
