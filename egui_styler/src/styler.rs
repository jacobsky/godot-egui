use egui::{FontDefinitions, Style};
use egui::epaint::text::Fonts;
use gdnative::prelude::*;
use gdnative::api::{Control, Resource};
use godot_egui::*;

#[derive(PartialEq, Clone, Copy)]
enum StylerTab {
    // Visual,
    // TextStyle,
    Colors,
    // Spacing,
    // Widget,
    Fonts
}
#[derive(NativeClass)]
#[inherit(Resource)]
pub struct EguiStyleResource {
    pub style: Style,
    pub font_definitions: FontDefinitions
}
#[methods]
impl EguiStyleResource {
    fn new(_: &Resource) -> Self {
        EguiStyleResource::default()
    }
    fn default() -> Self {
        Self {
            style: Style::default(),
            font_definitions: FontDefinitions::default(),
        }
    }
}

#[derive(NativeClass)]
#[inherit(gdnative::api::Control)]
pub struct GodotEguiStyler {
    tab: StylerTab,
    gui: Option<Instance<GodotEgui, Shared>>,
    style: Option<EguiStyleResource>,
}

#[gdnative::methods]
impl GodotEguiStyler {
    pub fn new(_owner: TRef<Control>) -> Self {
        Self {
            tab: StylerTab::Colors,
            style: None,
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
        let (style, font_definitions) = gui.map(|gui, _|{
            let style_ref = gui.egui_ctx.style().clone();
            let style = style_ref.as_ref().clone();
            let font_defs = gui.egui_ctx.fonts().definitions().clone();
            (style, font_defs)
        }).expect("this should work");
        self.style = Some(
            EguiStyleResource {
                style,
                font_definitions
            }
        );
        self.gui = Some(gui.claim());
    }

    #[export]
    pub fn _process(&mut self, _: TRef<Control>, _: f64) {
        use egui::widgets::Widget;
        let gui = unsafe { self.gui.as_ref().expect("GUI initialized").assume_safe() };
        // A frame can be passed to `update` specifying background color, margin and other properties
        // You may also want to pass in `None` and draw a background using a regular Panel node instead.
        let mut frame = egui::Frame::default();
        frame.margin = egui::vec2(0.0, 0.0);
        
        gui.map_mut(|gui, instance| {
            // We use the `update` method here to just draw a simple UI on the central panel. If you need more
            // fine-grained control, you can use update_ctx to get access to egui's context directly.
            gui.update_ctx(instance, |ctx| {
                egui::TopBottomPanel::top("menu_panel").show(ctx, menu_ui);
                egui::TopBottomPanel::top("top_panel").frame(frame).show(ctx, |ui| self.tab = tab_menu_ui(self.tab, ui));
                egui::CentralPanel::default().frame(frame).show(ctx, |ui|{
                    match self.tab {
                        StylerTab::Colors => {
                            // Needs a few sub menus.
                            // The visual menu already suffers from some serious issues regarding how much information is on screen at once.
                            // For theming, I think having all the widget settings be separated by color and 
                            ctx.style().visuals.clone().ui(ui)
                        }
                        StylerTab::Fonts => {
                            ctx.fonts().definitions().clone().ui(ui);
                            // Will need some way to open a file explorer box to get a new resource file. This might need to reach into the godot editor directly.l
                        }
                    }
                });
            })
        })
        .expect("Map mut should succeed");
    }
}

fn menu_ui(ui: &mut egui::Ui) {
    egui::menu::menu(ui, "File", |ui| {
        ui.add(egui::widgets::SelectableLabel::new(false, "New"));
        ui.add(egui::widgets::SelectableLabel::new(false, "Load"));
        ui.add(egui::widgets::SelectableLabel::new(false, "Save"));
    });
}

fn tab_menu_ui(tab: StylerTab, ui: &mut egui::Ui) -> StylerTab {
    let mut tab = tab;
    // Thingy
    ui.heading("Theme Options");
    // Menu tabs
    ui.horizontal(|ui|{
        if ui.add(egui::widgets::SelectableLabel::new(tab == StylerTab::Colors, "Colors")).clicked() {
            tab = StylerTab::Colors;
        }
        if ui.add(egui::widgets::SelectableLabel::new(tab == StylerTab::Fonts, "Fonts")).clicked() {
            tab = StylerTab::Fonts;
        }
    });
    tab
}
