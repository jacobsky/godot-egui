use gdnative::prelude::*;
use godot_egui::*;

#[derive(NativeClass)]
#[inherit(gdnative::api::Control)]
pub struct GodotEguiStyler {
    gui: Option<Instance<GodotEgui, Shared>>,
}

#[gdnative::methods]
impl GodotEguiStyler {
    pub fn new(_owner: TRef<Control>) -> Self {
        Self {
            gui: None,
        }
    }

    #[export]
    pub unsafe fn _ready(&mut self, owner: TRef<Control>) {
        godot_print!("Initializing godot egui");
        let gui = owner
            .get_node("GodotEgui")
            .and_then(|godot_egui| godot_egui.assume_safe().cast::<Control>())
            .and_then(|godot_egui| godot_egui.cast_instance::<GodotEgui>())
            .expect("Expected a `GodotEgui` child with the GodotEgui nativescript class.");

        self.gui = Some(gui.claim());
    }

    #[export]
    pub fn _process(&mut self, _owner: TRef<Control>, _: f64) {
        let gui = unsafe { self.gui.as_ref().expect("GUI initialized").assume_safe() };

        // A frame can be passed to `update` specifying background color, margin and other properties
        // You may also want to pass in `None` and draw a background using a regular Panel node instead.
        let mut frame = egui::Frame::default();
        frame.margin = egui::vec2(20.0, 20.0);

        gui.map_mut(|gui, instance| {
            // We use the `update` method here to just draw a simple UI on the central panel. If you need more
            // fine-grained control, you can use update_ctx to get access to egui's context directly.
            gui.update_ctx(instance, |ctx| {
                egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                    ctx.settings_ui(ui)
                });
            })
        })
        .expect("Map mut should succeed");
    }
}