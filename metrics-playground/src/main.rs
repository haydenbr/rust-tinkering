use metrics::{counter, increment_counter};
use metrics_runtime::{
    Receiver, observers::YamlBuilder, exporters::LogExporter,
};
use log::Level;
use std::{thread, time::Duration};

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let receiver = Receiver::builder().build().expect("failed to create receiver");

    // Now create our exporter/observer configuration, and wire it up.
    let mut exporter = LogExporter::new(
        receiver.controller(),
        YamlBuilder::new(),
        Level::Info,
        Duration::from_secs(1),
    );

    thread::spawn(move || {
        exporter.run();
    });

    let metric_thread = thread::spawn(move || {
        let mut sink = receiver.sink();
    
        // We can update a counter.  Counters are monotonic, unsigned integers that start at 0 and
        // increase over time.
        // Take some measurements, similar to what we had in other examples:
        sink.counter(name)
        sink.increment_counter("widgets", 5);
        sink.update_gauge("red_balloons", 99);
        
        let start = sink.now();
        thread::sleep(Duration::from_millis(10));
        let end = sink.now();
        sink.record_timing("select_products_ns", start, end);
        sink.record_timing("gizmo_query", start, end);
        
        let num_rows = 46;
        sink.record_value("select_products_num_rows", num_rows);

        thread::sleep(Duration::from_secs(5));

        increment_counter!("bob");
    });

    metric_thread.join().unwrap();
}
