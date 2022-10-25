use chrono::Local;
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeTupleStruct};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tabled::builder::Builder;
use tabled::object::Segment;
use tabled::{Alignment, Modify, Style};

pub fn now() -> String {
    //! returns time string with 6 milliseconds digits precision
    Local::now().format("%H:%M:%S%.6f").to_string()
}

/// Internal representaion of an event, an `Event` gains its  meaning in a relation/collection like in `Timeline`
#[derive(Debug)]
pub struct Event(String, Instant);

#[derive(Serialize)]
pub struct EventSinceStart(String, u128);

impl Event {
    fn new(name: &str) -> Self {
        Event(name.to_string(), Instant::now())
    }
}

/// Every point represents the amount of time from creaation
/// at which the event was captured
struct ProgressiveTimeline {}

/// Timeline/sequence of events that were recorded over a timespan by the `event()` method
#[derive(Debug)]
pub struct Timeline {
    created_at: Instant,
    events: Vec<Event>,
}

impl Timeline {
    pub fn new() -> Self {
        let mut timeline = Timeline {
            created_at: Instant::now(),
            events: Vec::new(),
        };
        timeline.event("init");
        timeline
    }

    pub fn event(&mut self, name: &str) {
        //! Add new event
        self.events.push(Event::new(name))
    }

    pub fn from_start(&self, start: Instant) -> Vec<EventSinceStart> {
        self.events
            .iter()
            .map(|event| {
                EventSinceStart(
                    event.0.to_string(),
                    event.1.duration_since(start).as_micros(),
                )
            })
            .collect()
    }

    pub fn get_cols(&self) -> Vec<String> {
        self.events.iter().map(|item| item.0.to_string()).collect()
    }

    pub fn get_vals(&self, start: Instant) -> Vec<u128> {
        self.events
            .iter()
            .map(|item| item.1.duration_since(start).as_millis())
            .collect()
    }

    pub fn fill_map(&self, mut map: &mut HashMap<String, Vec<u128>>) {
        for EventSinceStart(header, timepoint) in self.from_start(self.created_at) {
            let vec = map.entry(header).or_insert(Vec::new());
            vec.push(timepoint);
        }
    }
}

impl Serialize for Timeline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.events.len()))?;
        for e in self.from_start(self.created_at.clone()) {
            seq.serialize_element(&e)?
        }
        seq.end()
    }
}

impl Display for Timeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Timeline>")
    }
}

/// Collection of multiple `Timeline`'s which share the same event markers and a shared start point
pub struct TimelineCollection {
    created_at: Instant,
    timelines: Vec<Timeline>,
}

impl TimelineCollection {
    pub fn new() -> Self {
        TimelineCollection {
            created_at: Instant::now(),
            timelines: Vec::new(),
        }
    }

    pub fn add(&mut self, timeline: Timeline) {
        self.timelines.push(timeline);
    }

    pub fn get_columns(&self) -> Vec<String> {
        self.timelines.get(0).unwrap().get_cols()
    }

    pub fn export_as_csv(&self, path: impl AsRef<Path>) -> Result<(), csv::Error> {
        let file = File::create(path).unwrap();
        let mut writer = csv::Writer::from_writer(file);

        writer.serialize(&self.timelines)?;
        writer.flush()?;

        Ok(())
    }

    pub fn print(&self) {}
}

impl Serialize for TimelineCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(&self.timelines)
        // let mut seq = serializer.serialize_seq(Some(self.timelines.len()))?;
        // for timeline in &self.timelines {
        //     let time_synced_timeline = timeline.from_start(self.created_at.clone());
        //     seq.serialize_element(&time_synced_timeline)?;
        // }
    }
}

pub fn print_timeline_table(logs_vec: Vec<Timeline>, start_time: Instant) {
    unimplemented!("Fix wtih csv format for Timeline addoption");
    let mut builder = Builder::default();
    let mut columns: Option<Vec<String>> = None;
    for logs in logs_vec {
        if columns.is_none() {
            columns = Some(logs.get_cols());
        }
        let values = logs.get_vals(start_time);
        builder.add_record(values);
    }

    match columns {
        Some(columns) => {
            builder.set_columns(columns);

            let table = builder
                .build()
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::right()));
            println!("{}", table);
        }
        None => {
            println!("At least one Logs struct is required to print a table.");
        }
    }
}
