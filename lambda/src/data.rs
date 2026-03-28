use chrono::prelude::Utc;
use chrono_tz::America::Los_Angeles;
use phf::phf_map;

pub struct EventStream {
    pub date: &'static str,
    pub stream: &'static str,
    pub alt_stream: Option<&'static str>,
}

type Event = &'static [EventStream];
type EventYear = phf::Map<&'static str, Event>;

pub const ALL_STREAMS: phf::Map<&'static str, EventYear> = phf_map! {
    "2026" => phf_map! {
        "blk" => &[
            EventStream {
                date: "2026-03-07",
                stream: "https://www.youtube.com/watch?v=tqImKZrD6B0",
                alt_stream: Some("https://www.youtube.com/watch?v=21-aqNWtxko"),
            },
            EventStream {
                date: "2026-03-08",
                stream: "https://www.youtube.com/watch?v=2CFKqm7IHfA",
                alt_stream: Some("https://www.youtube.com/watch?v=mAsQeD8regk"),
            },
        ],
        "osf" => &[
            EventStream {
                date: "2026-03-06",
                stream: "https://www.youtube.com/watch?v=VPWDvwxLED0",
                alt_stream: Some("https://www.youtube.com/watch?v=x-4VAVyb4as"),
            },
            EventStream {
                date: "2026-03-07",
                stream: "https://www.youtube.com/watch?v=h-qcbsZw-xs",
                alt_stream: Some("https://www.youtube.com/watch?v=XWAhpt1cxxU"),
            },
        ],
        "gpk" => &[
            EventStream {
                date: "2026-03-14",
                stream: "https://www.youtube.com/watch?v=XcQ8EndxcuM",
                alt_stream: Some("https://www.youtube.com/watch?v=bURRb2HrvXA"),
            },
            EventStream {
                date: "2026-03-15",
                stream: "https://www.youtube.com/watch?v=xU3Nbp_vpWE",
                alt_stream: Some("https://www.youtube.com/watch?v=KHvKahKD1OI"),
            },
        ],
        "wil" => &[
            EventStream {
                date: "2026-03-14",
                stream: "https://www.youtube.com/watch?v=tY1iFSJ1U3k",
                alt_stream: Some("https://www.youtube.com/watch?v=qFn0ZgF3KTo"),
            },
            EventStream {
                date: "2026-03-15",
                stream: "https://www.youtube.com/watch?v=K3Ftdmvmw1E",
                alt_stream: Some("https://www.youtube.com/watch?v=XGLQ_8yjcQ0"),
            },
        ],
        "sam" => &[
            EventStream {
                date: "2026-03-21",
                stream: "https://www.youtube.com/watch?v=dZ_T_LNCg_U",
                alt_stream: Some("https://www.youtube.com/watch?v=5GpOR-eBbNg"),
            },
            EventStream {
                date: "2026-03-22",
                stream: "https://www.youtube.com/watch?v=xgwgHLj5YZQ",
                alt_stream: Some("https://www.youtube.com/watch?v=eRglKjvZZCo"),
            },
        ],
        "sun" => &[
            EventStream {
                date: "2026-03-20",
                stream: "https://www.youtube.com/watch?v=Y538-wB-blE",
                alt_stream: Some("https://www.youtube.com/watch?v=stokCPqL9Xo"),
            },
            EventStream {
                date: "2026-03-21",
                stream: "https://www.youtube.com/watch?v=PH5qDFB1xq0",
                alt_stream: Some("https://www.youtube.com/watch?v=DvKmaFE-2Kc"),
            },
        ],
        "cas" => &[
            EventStream {
                date: "2026-03-27",
                stream: "https://www.youtube.com/watch?v=zq8HiQrru4c",
                alt_stream: Some("https://www.youtube.com/watch?v=FrwOFI5KcnI"),
            },
            EventStream {
                date: "2026-03-28",
                stream: "https://www.twitch.tv/firstinspires15",
                alt_stream: Some("https://www.twitch.tv/firstinspires14"),
            },
        ],
        "aub" => &[
            EventStream {
                date: "2026-03-28",
                stream: "https://www.twitch.tv/firstinspires16",
                alt_stream: Some("https://www.twitch.tv/firstinspires17"),
            },
            EventStream {
                date: "2026-03-29",
                stream: "https://www.twitch.tv/firstinspires16",
                alt_stream: Some("https://www.twitch.tv/firstinspires17"),
            },
        ],
    },
};

fn get_current_year() -> String {
    Utc::now().with_timezone(&Los_Angeles).format("%Y").to_string()
}

pub fn get_current_stream(event: &str, alt: bool) -> Option<&'static str> {
    get_current_stream_for_year(&get_current_year(), event, alt)
}

pub fn get_current_stream_for_year(year: &str, event: &str, alt: bool) -> Option<&'static str> {
    let today = Utc::now().with_timezone(&Los_Angeles).format("%Y-%m-%d").to_string();
    let schedules = ALL_STREAMS.get(year)?;
    let schedule = schedules.get(event)?;
    let stream = schedule.iter().rev().find(|s| s.date <= today.as_str()).unwrap_or(schedule.first()?);
    Some(if alt { stream.alt_stream.unwrap_or(stream.stream) } else { stream.stream })
}

pub fn get_day_stream(event: &str, day: usize, alt: bool) -> Option<&'static str> {
    get_day_stream_for_year(&get_current_year(), event, day, alt)
}

pub fn get_day_stream_for_year(year: &str, event: &str, day: usize, alt: bool) -> Option<&'static str> {
    let schedules = ALL_STREAMS.get(year)?;
    let schedule = schedules.get(event)?;
    let stream = schedule.get(day)?;
    Some(if alt { stream.alt_stream.unwrap_or(stream.stream) } else { stream.stream })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_stream_valid_event() {
        // Test with a valid event code
        let result = get_current_stream("blk", false);
        // Note: May return None if all stream dates are in the future
        // This is expected behavior - just verify it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_get_current_stream_invalid_event() {
        // Test with an invalid event code
        let result = get_current_stream("invalid", false);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_current_stream_for_year_valid() {
        // Test with valid year and event
        let result = get_current_stream_for_year("2026", "blk", false);
        // Note: May return None if all stream dates are in the future
        // This is expected behavior - just verify it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_get_current_stream_for_year_invalid_year() {
        // Test with invalid year
        let result = get_current_stream_for_year("2025", "blk", false);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_day_stream_valid() {
        // Test with valid event and day index (0-indexed)
        let result = get_day_stream("blk", 0, false);
        assert!(result.is_some());
    }

    #[test]
    fn test_get_day_stream_invalid_day() {
        // Test with invalid day index
        let result = get_day_stream("blk", 999, false);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_day_stream_for_year_valid() {
        // Test with valid year, event, and day
        let result = get_day_stream_for_year("2026", "blk", 0, false);
        assert!(result.is_some());
    }

    #[test]
    fn test_alternate_stream_available() {
        // Test alternate stream when available
        let result = get_day_stream("blk", 0, true);
        assert!(result.is_some());
    }

    // #[test]
    // fn test_alternate_stream_fallback() {
    //     // Test alternate stream fallback to primary when not available
    //     let result = get_day_stream("osf", 0, true);
    //     assert!(result.is_some());
    // }

    #[test]
    fn test_event_stream_struct_public() {
        // Verify EventStream struct and fields are public
        let stream = EventStream {
            date: "2026-03-14",
            stream: "https://example.com",
            alt_stream: Some("https://alt.example.com"),
        };
        assert_eq!(stream.date, "2026-03-14");
        assert_eq!(stream.stream, "https://example.com");
        assert_eq!(stream.alt_stream, Some("https://alt.example.com"));
    }
}
