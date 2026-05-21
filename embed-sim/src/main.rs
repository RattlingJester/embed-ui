use embedded_graphics::{pixelcolor::Rgb565, prelude::Size};
use embedded_graphics_simulator::{
	OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
	sdl2::{Keycode, MouseButton},
};

use embed_ui::{
	container::{Align, HorizontalAlign, Page, VerticalAlign, WidgetId},
	ui::Ui,
	widgets::{Widget, WidgetKind, button::Button, checkbox::Checkbox},
};

struct WidgetIDs<const WIDGET_COUNT: usize> {
	pub ids: [WidgetId; WIDGET_COUNT],
}

fn main() {
	let (page, elements) = main_page();

	let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 480));

	let output_settings = OutputSettingsBuilder::new().scale(2).build();
	let mut window = Window::new("Test", &output_settings);

	window.update(&display);

	let mut ui = Ui::new([page], embed_ui::style::DEFAULT_STYLE);

	// let mut pressed = false;
	// let mut pointer = None;
	let mut i = 0;

	'run: loop {
		window.update(&display);

		for event in window.events() {
			match event {
				SimulatorEvent::Quit => break 'run,
				SimulatorEvent::MouseButtonDown {
					mouse_btn: MouseButton::Left,
					point: _,
				} => {
					// pressed = true;
					// pointer = Some(point)
				}

				SimulatorEvent::MouseButtonUp {
					mouse_btn: MouseButton::Left,
					point: _,
				} => {
					// pressed = false;
					// pointer = Some(point)
				}
				// SimulatorEvent::MouseMove { point } => pointer = Some(point),
				SimulatorEvent::KeyDown {
					keycode: Keycode::A,
					keymod: _,
					repeat: _,
				} => {
					let page = ui.get_page_mut(0);
					page.focus_prev();
				}

				SimulatorEvent::KeyDown {
					keycode: Keycode::D,
					keymod: _,
					repeat: _,
				} => {
					let page = ui.get_page_mut(0);
					page.focus_next();
				}

				_ => (),
			}
		}

		let page = ui.get_page_mut(0);
		if let WidgetKind::Button(b) = page.get_mut(elements.ids[0]).unwrap() {
			let text = format!("HUI {}", i);
			b.set_text(&text).unwrap();
		}

		if let WidgetKind::Button(b2) = page.get_mut(elements.ids[1]).unwrap() {
			let text = format!("PIZDA {}", i);
			b2.set_text(&text).unwrap();
		}

		ui.draw(&mut display);

		i += 1;
	}
}

fn main_page() -> (Page<20>, WidgetIDs<4>) {
	let mut page = Page::new(
		Size::new(320, 480),
		true,
		Align {
			horizontal: HorizontalAlign::Center { columns: 2 },
			vertical:   VerticalAlign::Center { rows: 2 },
		},
	);

	let button1 = Button::new("HUI", Size::new(100, 50));
	let button2 = Button::new("PIZDA", Size::new(100, 50));
	let button3 = Button::new("ZALUPA", Size::new(100, 50));
	let checkbox = Checkbox::new("PIDOR", Size::new(50, 50));

	let id1 = page.insert(WidgetKind::Button(button1)).unwrap();
	let id2 = page.insert(WidgetKind::Button(button2)).unwrap();
	let id3 = page.insert(WidgetKind::Button(button3)).unwrap();
	let id4 = page.insert(WidgetKind::Checkbox(checkbox)).unwrap();

	let elements = WidgetIDs {
		ids: [id1, id2, id3, id4],
	};

	(page, elements)
}
