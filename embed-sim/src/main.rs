#![allow(unused)]

use embedded_graphics::{pixelcolor::Rgb565, prelude::Size};
use embedded_graphics_simulator::{
	OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window, sdl2::MouseButton,
};

use embed_ui::{
	Point, Rectangle,
	container::{View, WidgetId},
	style::Style,
	ui::Ui,
	widgets::Widget,
};

#[derive(Debug)]
struct Elements {
	button:  WidgetId,
	button2: WidgetId,
}

fn main() {
	let (mut page, ids) = const { main_page() };

	let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 480));

	let output_settings = OutputSettingsBuilder::new().scale(2).build();
	let mut window = Window::new("Test", &output_settings);

	window.update(&display);

	let mut ui = Ui::new(embed_ui::style::DEFAULT_STYLE);

	let mut pressed = false;
	let mut pointer = None;

	page.get_mut(ids.button).unwrap().held = true;
	page.get_mut(ids.button).unwrap().text = "JOPA";

	'run: loop {
		window.update(&display);

		for event in window.events() {
			match event {
				SimulatorEvent::Quit => break 'run,
				SimulatorEvent::MouseButtonDown {
					mouse_btn: MouseButton::Left,
					point,
				} => {
					pressed = true;
					pointer = Some(point)
				}

				SimulatorEvent::MouseButtonUp {
					mouse_btn: MouseButton::Left,
					point,
				} => {
					pressed = false;
					pointer = Some(point)
				}
				SimulatorEvent::MouseMove { point } => pointer = Some(point),

				_ => (),
			}
		}

		ui.draw_view(&page, &mut display);
	}
}

const fn main_page() -> (View<'static, 20>, Elements) {
	let mut view = View::new();

	let id = view.insert(Widget::button(
		"HUI",
		Rectangle::new(Point::new(0, 0), Size::new(100, 50)),
	));

	let id2 = view.insert(Widget::button(
		"ZALUPA",
		Rectangle::new(Point::new(100, 50), Size::new(100, 50)),
	));

	let elements = Elements {
		button:  id,
		button2: id2,
	};

	(view, elements)
}
