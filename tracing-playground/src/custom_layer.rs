use std::collections::BTreeMap;

use serde_json::Serializer;
use tracing::{span, field};
use tracing_subscriber::{Layer, fmt::{format::{Json, self}, SubscriberBuilder, FormatFields}};

pub struct CustomLayer;

impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
	fn on_event(
		&self,
		event: &tracing::Event<'_>,
		ctx: tracing_subscriber::layer::Context<'_, S>
	) {
		let serz = Serializer::new(std::io::stdout());

		let scope = ctx.event_scope(event).unwrap();
		let mut spans = vec![];

		for span in scope.from_root() {
			let extensions = span.extensions();
			let storage = extensions.get::<CustomFieldStorage>().unwrap();
			let field_data: &BTreeMap<String, serde_json::Value> = &storage.0;
			
			spans.push(serde_json::json!({
				"target": span.metadata().target(),
				"name": span.name(),
				"level": format!("{:?}", span.metadata().level()),
				"fields": field_data,
			}));
		}

		let mut fields = BTreeMap::new();
		let mut visitor = JsonVisitor(&mut fields);
		event.record(&mut visitor);

		let output = serde_json::json!({
			"target": event.metadata().target(),
			"name": event.metadata().name(),
			"level": format!("{:?}", event.metadata().level()),
			"fields": fields,
			"spans": spans,
		});

		println!("{}", serde_json::to_string_pretty(&output).unwrap());
	}

	fn on_new_span(
		&self,
		attrs: &span::Attributes<'_>,
		id: &span::Id,
		ctx: tracing_subscriber::layer::Context<'_, S>
	) {
		let mut fields = BTreeMap::new();
		let mut visitor = JsonVisitor(&mut fields);
		attrs.record(&mut visitor);

		let storage = CustomFieldStorage(fields);

		let span = ctx.span(id).unwrap();
		let mut extensions = span.extensions_mut();

		extensions.insert(storage);
	}

	fn on_record(
		&self,
		id: &span::Id,
		values: &span::Record<'_>,
		ctx: tracing_subscriber::layer::Context<'_, S>
	) {
			let span = ctx.span(id).unwrap();

			let mut extensions = span.extensions_mut();
			let custom_field_storage = extensions.get_mut::<CustomFieldStorage>().unwrap();
			let json_data = &mut custom_field_storage.0;
			let visitor = &mut JsonVisitor(json_data);
			
			values.record(visitor);
	}
}

#[derive(Debug)]
struct CustomFieldStorage(BTreeMap<String, serde_json::Value>);

struct JsonVisitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

impl<'a> tracing::field::Visit for JsonVisitor<'a> {
	fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
			self.0.insert(field.name().to_string(), serde_json::json!(value));
	}

	fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
			self.0.insert(field.name().to_string(), serde_json::json!(value));
	}

	fn record_i128(&mut self, field: &tracing::field::Field, value: i128) {
			self.0.insert(field.name().to_string(), serde_json::json!(value));
	}

	fn record_u128(&mut self, field: &tracing::field::Field, value: u128) {
			self.0.insert(field.name().to_string(), serde_json::json!(value));
	}

	fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
			self.0.insert(field.name().to_string(), serde_json::json!(value));
	}

	fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
			self.0.insert(field.name().to_string(), serde_json::json!(value));
	}

	fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
			self.0.insert(field.name().to_string(), serde_json::json!(value));
	}

	fn record_error(&mut self, field: &tracing::field::Field, value: &(dyn std::error::Error + 'static)) {
		self.0.insert(field.name().to_string(), serde_json::json!(format!("{}", value)));
	}

	fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
		self.0.insert(field.name().to_string(), serde_json::json!(format!("{:?}", value)));
	}
}

struct PrintlnVisitor;

impl tracing::field::Visit for PrintlnVisitor {
	fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
			println!("  field={} value={}", field.name(), value)
	}

	fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
		println!("  field={} value={}", field.name(), value)
	}

	fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
		println!("  field={} value={}", field.name(), value)
	}

	fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
		println!("  field={} value={}", field.name(), value)
	}

	fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
		println!("  field={} value={}", field.name(), value)
	}

	fn record_error(
		&mut self,
		field: &tracing::field::Field,
		value: &(dyn std::error::Error + 'static),
	) {
		println!("  field={} value={}", field.name(), value)
	}

	fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
		println!("  field={} value={:?}", field.name(), value)
	}
}
