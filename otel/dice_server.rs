use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use opentelemetry::KeyValue;
use opentelemetry::global::{self};
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::sdk::trace::TracerProvider;
use opentelemetry::trace::{Tracer, SpanKind, Status, Span, mark_span_as_active};
use opentelemetry_stdout::SpanExporter;
use rand::Rng;
use std::{convert::Infallible, net::SocketAddr};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    let tracer = global::tracer("dice_server");

    let mut span = tracer
        .span_builder(format!("{} {}", req.method(), req.uri().path()))
        .with_kind(SpanKind::Server)
        .start(&tracer);

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/rolldice") => {
            let random_number = rand::thread_rng().gen_range(1..7);

            span.add_event("dice_role", vec![KeyValue::new("dice_value", random_number)]);

            {
                let mut child_span = tracer.span_builder("child_span")
                    .with_kind(SpanKind::Server)
                    .start(&tracer);
                // mark_span_as_active(child_span)
                child_span.

                child_span.add_event("child_event", vec![]);
            }
            
            *response.body_mut() = Body::from(random_number.to_string());
            span.set_status(Status::Ok);
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            span.set_status(Status::error("Not Found"));
        }
    };

    Ok(response)
}

fn init_tracer() {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let provider = TracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();
    global::set_tracer_provider(provider);
}

#[tokio::main]
async fn main() {
    init_tracer();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on {addr}");
    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }
}
