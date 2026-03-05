# Requirements Document

## Introduction

This feature extends the FRC event stream API to support day-based indexing and alternate stream URLs. The API redirects users to YouTube stream URLs for FIRST Robotics Competition events. Currently, the API supports basic year/event lookups. This enhancement adds support for accessing specific days of multi-day events, retrieving current streams, and accessing alternate stream URLs when available.

## Glossary

- **API**: The Rust Lambda API using Rocket framework that handles HTTP requests and redirects
- **Event**: A FIRST Robotics Competition event identified by a short code (e.g., "blk", "osf")
- **Event_Stream**: A data structure containing a date, primary stream URL, and optional alternate stream URL
- **Day_Index**: A 1-indexed integer representing the day of a multi-day event (day 1, 2, 3, etc.)
- **Current_Day**: The most recent stream day that is not in the future, based on Pacific timezone
- **Current_Year**: The current year in Pacific timezone
- **Alternate_Stream**: An optional secondary YouTube URL for the same event day
- **Stream_Selector**: A path component that determines whether to return the primary or alternate stream

## Requirements

### Requirement 1: Current Day Stream Access

**User Story:** As an FRC viewer, I want to access the current day's stream for an event in the current year, so that I can quickly watch today's competition.

#### Acceptance Criteria

1. WHEN a request is made to `/:event`, THE API SHALL redirect to the primary stream URL for the Current_Day of the Event in the Current_Year
2. IF no Event matches the event code, THEN THE API SHALL return HTTP 404
3. IF no stream exists for the Current_Day, THEN THE API SHALL return HTTP 404
4. THE API SHALL determine Current_Day as the most recent Event_Stream date that is not in the future based on Pacific timezone

### Requirement 2: Specific Day Stream Access

**User Story:** As an FRC viewer, I want to access a specific day's stream for an event, so that I can watch recordings from earlier days of the competition.

#### Acceptance Criteria

1. WHEN a request is made to `/:event/:day` where day is a Day_Index, THE API SHALL redirect to the primary stream URL for that day of the Event in the Current_Year
2. THE API SHALL treat Day_Index as 1-indexed (day 1 is the first stream, day 2 is the second stream, etc.)
3. IF the Day_Index is less than 1, THEN THE API SHALL return HTTP 404
4. IF the Day_Index exceeds the number of streams for the Event, THEN THE API SHALL return HTTP 404
5. IF no Event matches the event code, THEN THE API SHALL return HTTP 404

### Requirement 3: Alternate Stream Access

**User Story:** As an FRC viewer, I want to access alternate stream URLs when available, so that I can watch different camera angles or backup streams.

#### Acceptance Criteria

1. WHEN a request is made to `/:event/alt`, THE API SHALL redirect to the alternate stream URL for the Current_Day of the Event in the Current_Year
2. WHEN a request is made to `/:event/:day/alt`, THE API SHALL redirect to the alternate stream URL for that Day_Index of the Event in the Current_Year
3. IF an Alternate_Stream is not available for the requested day, THEN THE API SHALL redirect to the primary stream URL instead
4. THE Stream_Selector "alt" SHALL be case-insensitive

### Requirement 4: Year-Specific Stream Access

**User Story:** As an FRC viewer, I want to access streams from previous years, so that I can watch historical competition footage.

#### Acceptance Criteria

1. WHEN a request is made to `/:year/:event`, THE API SHALL redirect to the primary stream URL for the Current_Day of the Event in the specified year
2. WHEN a request is made to `/:year/:event/:day`, THE API SHALL redirect to the primary stream URL for that Day_Index of the Event in the specified year
3. WHEN a request is made to `/:year/:event/alt`, THE API SHALL redirect to the alternate stream URL for the Current_Day of the Event in the specified year
4. WHEN a request is made to `/:year/:event/:day/alt`, THE API SHALL redirect to the alternate stream URL for that Day_Index of the Event in the specified year
5. IF the year does not exist in the data, THEN THE API SHALL return HTTP 404
6. THE API SHALL apply the same Current_Day and Day_Index logic as year-agnostic endpoints

### Requirement 5: Event Code Case Insensitivity

**User Story:** As an FRC viewer, I want to use any capitalization for event codes, so that I don't have to remember the exact casing.

#### Acceptance Criteria

1. WHEN a request contains an event code, THE API SHALL convert it to lowercase before lookup
2. THE API SHALL treat "BLK", "blk", and "Blk" as equivalent event codes

## Design Considerations

### Alternate Stream Handling Options

The current implementation stores alternate streams as optional fields on each Event_Stream. Three potential approaches for improvement:

1. **Status Quo (Optional Field)**: Keep `alt_stream: Option<&'static str>` on Event_Stream
   - Pros: Simple, minimal changes, clear data model
   - Cons: Feels "shoehorned in", requires fallback logic in multiple places

2. **Stream Type Enum**: Replace single stream with `streams: Vec<(StreamType, &'static str)>` where StreamType is an enum
   - Pros: Extensible to more stream types, explicit about available streams
   - Cons: More complex data structure, harder to maintain static data

3. **Separate Collections**: Split into primary and alternate event maps
   - Pros: Clean separation, no optional handling
   - Cons: Duplicate event structure, harder to keep in sync

**Recommendation**: Stick with Option 1 (status quo) for this lightweight spec. The optional field approach is idiomatic Rust, keeps the data model simple, and the fallback logic is isolated to a few helper functions. The "shoehorned" feeling is actually just Rust's explicit handling of optional values, which is a feature, not a bug.

### Day Index Conversion

The data structure uses 0-indexed arrays, but the API should expose 1-indexed days (more intuitive for users). The conversion should happen at the routing layer, with helper functions receiving 0-indexed values internally.
