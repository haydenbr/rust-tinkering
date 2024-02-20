use std::time::Duration;

struct Never;

pub enum TsReport {
    Onvif,
    SenderReport,
    Inferred,
}

pub enum PauseResumeMethod {
    Rtsp,
    Reconnect,
}

pub enum UrlType {
    Standard,
    Timestamped,
}

pub trait StreamUrl {

}

pub struct TimestampedUrl {

}

impl StreamUrl for TimestampedUrl {

}

pub struct StandardUrl {

}

impl StreamUrl for StandardUrl {

}

pub struct PlaybackBuilder<U = Never> {
    url: U
}

// pub fn fmt() -> SubscriberBuilder {
//     SubscriberBuilder::default()
// }

// impl Default for SubscriberBuilder {
//     fn default() -> Self {
//         SubscriberBuilder {
//             filter: Subscriber::DEFAULT_MAX_LEVEL,
//             inner: Default::default(),
//         }
//         .log_internal_errors(true)
//     }
// }
impl Default for PlaybackBuilder {
    fn default() -> Self {
        PlaybackBuilder { url: Never }
    }
}

impl PlaybackBuilder {
    pub fn new() -> PlaybackBuilder<Never> {
        PlaybackBuilder::default()
    }
}

impl<U> PlaybackBuilder<U> {
    pub fn timestamp_reporting(self, ts_report: TsReport) -> PlaybackBuilder<U> {

    }

    pub fn pause_resume(self, pause_resume_method: PauseResumeMethod) -> PlaybackBuilder<U> {

    }

    pub fn url(self, url: String) -> PlaybackBuilder<StandardUrl> {

    }

    pub fn timestamped_url(self, url: String) -> PlaybackBuilder<TimestampedUrl> {

    }
}

impl<U> PlaybackBuilder<U>
    where U: StreamUrl
{
    pub fn build(self) -> PlaybackStream {
        Playback {}
    }
}

impl PlaybackBuilder<TimestampedUrl>
{
    pub fn timestamp_format(self, format: &str) -> Self {

    }
}

impl PlaybackBuilder<StandardUrl>
{
    pub fn instant_replay_offset(self, offset: Duration) -> Self {

    }
}

// Playback itself can't be generic or else the interface across all playback sessions won't be consistent
// and we'll be unable to serialize this struct from memory when we cross the border from node to rust
// we can probably address this in the future when it's just rust, but for now, it simplifies things if we
// just have a plain 'ol struct
struct PlaybackStream {

}

impl PlaybackStream {

    pub fn start() {

    }

    pub fn pause() {

    }

    pub fn play(start_time: u64) {
        if play_pause == PauseResumeMethod::Reconnect {
            // asdas
        } else {
            // asdas
        }
    }

    pub fn seek(seek_to: u64) {

    }

    pub fn stop() {

    }
}

/*
What's the API that I want?
url is the only required input, everything else defaults
Are there any builder methods that don't work for timestamped or don't work for standard url?
- timestamped: timestamp_format

generics in tracing crate
SubscriberBuilder<format::JsonFields, format::Format<format::Json, T>, F, W>
- Format struct holds a bunch of formatting fields, mostly just bool flags
- Json is a struct that holds a couple json-specific formatting fields
- FormatEvent is implemented for Format<Json>

- So in general, we have A<B<C>>. We only need these two layers of generics if we want to implement
something in particular for both A<B<_>> as well as A<B<C>>.

Although, we could simplify this by just having A<B> and have general impl on A<B> and specific impl for
A<B> where B implements some trait, and then A<B> where B is some specific impl of said trait. I think I like this.
 */

fn make_pipelines(integration_type, url): PlaybackStream {
    if (genetec) {
        PlaybackBuilder::new()
            .url("".into())
            .timestamp_reporting(TsReport::SenderReport)
            .pause_resume(PauseResumeMethod::Rtsp)
            .build();
    }
	
	// let timestamped_playback = PlaybackBuilder::new()
	// 	.timestamp_reporting(TsReport::Inferred)
    //     .timestamped_url("".into()) // implies disconnect/reconnect for pause/play/seek
    //     .timestamp_format("YYYY-mm-dd ...etc") // some valid timestamp formatting string
    //     // .backend::<GStreamer>()
    //     .build()
	// 	;
}