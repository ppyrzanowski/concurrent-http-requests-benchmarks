use chrono::Local;
use std::time::Instant;
use tabled::builder::Builder;
use tabled::object::Segment;
use tabled::{Alignment, Modify, Style};

pub fn now() -> String {
    //! returns time string with 6 milliseconds digits precision
    Local::now().format("%H:%M:%S%.6f").to_string()
}

pub struct Timeline {
    measurements: Vec<(String, Instant)>,
}

impl Timeline {
    pub fn new() -> Self {
        let mut log = Timeline {
            measurements: Vec::new(),
        };
        log.add("init");
        log
    }

    pub fn add(&mut self, log_name: &str) {
        //! Add new event
        self.measurements
            .push((log_name.to_string(), Instant::now()))
    }

    pub fn get_cols(&self) -> Vec<String> {
        self.measurements
            .iter()
            .map(|item| item.0.to_string())
            .collect()
    }

    pub fn get_vals(&self, start: Instant) -> Vec<u128> {
        self.measurements
            .iter()
            .map(|item| item.1.duration_since(start).as_millis())
            .collect()
    }
}

pub fn print_logs_table(logs_vec: Vec<Timeline>, start_time: Instant) {
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
