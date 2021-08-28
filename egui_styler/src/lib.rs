use gdnative::prelude::*;
use godot_egui::GodotEgui;

mod styler;
pub use styler::GodotEguiStyler;

pub fn load_texture(path: &str) -> Ref<Texture> {
    let loader = ResourceLoader::godot_singleton();
    loader.load(path, "Texture", false).expect("Texture found").cast().expect("Is texture")
}

fn init(handle: InitHandle) {
    handle.add_tool_class::<GodotEguiExample>();
    handle.add_tool_class::<GodotEguiStyler>();
    godot_egui::register_classes(handle);
}

godot_init!(init);
