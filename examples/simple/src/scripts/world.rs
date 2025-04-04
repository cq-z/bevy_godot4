use godot_rust_script::{
	godot::prelude::{godot_print, Gd,  Node3D, Object},
	godot_script_impl, GodotScript,
};


#[derive(Debug, GodotScript)]
struct ExampleScript {
	#[export]
	pub flag: bool,
	property: Option<Gd<Object>>,
	base: Gd<Node3D>,
}

#[godot_script_impl]
impl ExampleScript {
	pub fn perform_action(&self, value: i32) -> bool {
		value > 0
	}

	pub fn _process(&mut self, delta: f64) {
		godot_print!(
			"example script doing process stuff: {}, {}",
			delta,
			self.base
		);
	}
}