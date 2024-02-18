use actix_web::web::Bytes;

pub(crate) struct Event<T: EventTrait> {
	val: T,
}

pub(crate) trait EventTrait {
	fn get_id(&self) -> Option<&'static str> {
		None
	}
	fn get_event(&self) -> Option<&'static str> {
		None
	}
	fn get_data(&self) -> Option<&'static str> {
		None
	}
}

impl<T: EventTrait> Event<T> {
	pub fn new(val: T) -> Self {
		Event { val }
	}
}

impl<T: EventTrait> ToString for Event<T> {
	fn to_string(&self) -> String {
		let mut output = "".to_string();

		if let Some(value) = self.val.get_id() {
			output += &format!("id: {}\n", value);
		}
		if let Some(value) = self.val.get_event() {
			output += &format!("event: {}\n", value);
		}
		if let Some(value) = self.val.get_data() {
			output += &format!("data: {}\n", value);
		}

		if !output.is_empty() {
			output += "\n";
		}

		output
	}
}

impl<T: EventTrait> From<Event<T>> for Bytes {
	fn from(val: Event<T>) -> Self {
		Bytes::from(val.to_string())
	}
}

impl<T: EventTrait> From<T> for Event<T> {
	fn from(value: T) -> Self {
		Event { val: value }
	}
}

pub(crate) struct EventBuilder {
	id: Option<&'static str>,
	event: Option<&'static str>,
	data: Option<&'static str>,
}

#[allow(dead_code)]
impl EventBuilder {
	pub fn new() -> Self {
		EventBuilder {
			id: None,
			event: None,
			data: None,
		}
	}
	pub fn with_id(mut self, id: &'static str) -> Self {
		self.id = Some(id);
		self
	}
	pub fn with_event(mut self, event: &'static str) -> Self {
		self.event = Some(event);
		self
	}
	pub fn with_data(mut self, data: &'static str) -> Self {
		self.data = Some(data);
		self
	}
}

impl EventTrait for EventBuilder {
	fn get_data(&self) -> Option<&'static str> {
		self.data
	}
	fn get_event(&self) -> Option<&'static str> {
		self.event
	}
	fn get_id(&self) -> Option<&'static str> {
		self.id
	}
}
