use custom_layer::CustomLayer;
use tracing::{info_span, info, span::Entered, debug, error};
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;

mod custom_layer;

fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(CustomLayer)
        .with(EnvFilter::from_default_env())
        .init();

    let parent_span = info_span!("parent", level = 0);
    let _parent_entered = parent_span.enter();

    // debug!("parent debug event");
    // info!("parent info event");
    // error!("parent error event");

    let child_span = info_span!("child", level = 1);
    let _child_entered = child_span.enter();

    // debug!("child debug event");
    info!("child info event");
    // error!("child error event");
}

fn tracing_across_threads() {
    tracing_subscriber::fmt()
        .json()
        .with_line_number(true)
        .with_file(true)
        // we can set the log level output via RUST_LOG env variable
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::ACTIVE)
        // .json()
        // .flatten_event(true)
        .init();

    // let req_id = Uuid::new_v4();
    // let stream_id = Uuid::new_v4();
    // let replay_type: u8 = 2;

    // let root_span = info_span!(
    //     "root",
    //     req_id = req_id.to_string(),
    //     stream_id = stream_id.to_string(),
    //     replay_type,
    // ).entered();

    // let child_span = Arc::new(info_span!("child_span"));

    // info!("start");

    // let t1_span = child_span.clone();
    // let t1_handle = thread::spawn(move|| {
    //     let _child_span = t1_span.enter();
    //     info!("started thread 1");
    //     thread::sleep(Duration::from_secs(1));
    //     info!("ending thread 1");
    // });

    // let t2_span = child_span.clone();
    // let t2_handle = thread::spawn(move|| {
    //     let _child_span = t2_span.enter();
    //     info!("started thread 2");
    //     thread::sleep(Duration::from_secs(1));
    //     info!("ending thread 2");
    // });

    // t1_handle.join().unwrap();
    // t2_handle.join().unwrap();

    // let t3_span = child_span.clone();
    // let t3_handle = thread::spawn(move|| {
    //     let _child_span = t3_span.enter();
    //     info!("started thread 3");
    //     thread::sleep(Duration::from_secs(1));
        
    //     info_span!("thread_span").in_scope(|| {
    //         info!("started thread span");
    //         thread::sleep(Duration::from_secs(1));
    //         info!("ending thread span");
    //     });

    //     info!("ending thread 3");
    // });

    // t3_handle.join().unwrap();

    let loop_span_1 = info_span!("loop - phase 1");
    let loop_span_2 = info_span!("loop - phase 2");
    let loop_span_3 = info_span!("loop - phase 3");

    // for i in 1..10 {
    //     let _span_guard = match i {
    //         1..=2 => loop_span_1.enter(),
    //         3..=8 => loop_span_2.enter(),
    //         _ => loop_span_3.enter(),
    //     };

    //     info!("loop event: {}", i);
    // }
    let mut span_guard: Option<Entered<'_>>;
    for i in 1..=10 {
        span_guard = match i {
            1 => Some(loop_span_1.enter()),
            3 => Some(loop_span_2.enter()),
            9 => Some(loop_span_3.enter()),
            _ => None,
        };

        info!("loop event: {}", i);

        if i == 2 || i == 8 || i == 10 {
            if let Some(span_guard) = span_guard {
                drop(span_guard);
            }
        }
    }

    info!("end");
}