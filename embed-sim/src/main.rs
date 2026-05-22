use embedded_graphics::{pixelcolor::Rgb565, prelude::Size};
use embedded_graphics_simulator::{
	OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
	sdl2::{Keycode, MouseButton},
};

use embed_ui::{
	DrawTarget,
	container::{Align, HorizontalAlign, Page, VerticalAlign, WidgetId},
	ui::Ui,
	widgets::{
		WidgetKind, button::Button, checkbox::Checkbox, label::Label, separator::Separator,
		textbox::Textbox,
	},
};

// macro_rules! to_page {
// 	($page:expr) => {
// 		unsafe { core::mem::transmute::<u8, Pages>($page) }
// 	};
// }

// #[derive(Debug, Clone, Copy)]
// #[repr(u8)]
// pub enum Pages {
// 	Main     = 0,
// 	Settings = 1,
// }

struct WidgetIDs<const WIDGET_COUNT: usize> {
	pub ids: [WidgetId; WIDGET_COUNT],
}

fn main() {
	let (page_main, elements_main) = main_page().unwrap();
	let (page_settings, _elements_sett) = settings_page().unwrap();

	let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 480));

	let output_settings = OutputSettingsBuilder::new().scale(2).build();
	let mut window = Window::new("Test", &output_settings);

	window.update(&display);
	display.clear(embed_ui::style::DEFAULT_STYLE.screen_bg);

	let mut ui = Ui::new([page_main, page_settings], embed_ui::style::DEFAULT_STYLE);

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
					let page = ui.current_page_mut();
					page.focus_prev();
				}

				SimulatorEvent::KeyDown {
					keycode: Keycode::D,
					keymod: _,
					repeat: _,
				} => {
					let page = ui.current_page_mut();
					page.focus_next();
				}

				SimulatorEvent::KeyDown {
					keycode: Keycode::E,
					keymod: _,
					repeat: _,
				} => {
					ui.next_page();
				}

				SimulatorEvent::KeyDown {
					keycode: Keycode::Q,
					keymod: _,
					repeat: _,
				} => {
					ui.prev_page();
				}

				_ => (),
			}
		}

		let page = ui.get_page_mut(0);
		if let WidgetKind::Button(b) = page.get_mut(elements_main.ids[0] as WidgetId).unwrap() {
			let text = format!("HUI {}", i);
			b.set_text(&text).unwrap();
		}

		if let WidgetKind::Button(b2) = page.get_mut(elements_main.ids[1] as WidgetId).unwrap() {
			let text = format!("PIZDA {}", i);
			b2.set_text(&text).unwrap();
		}

		ui.draw(&mut display);

		i += 1;
	}
}

fn main_page() -> Result<(Page<10>, WidgetIDs<3>), embed_ui::Error> {
	let mut page = Page::new(
		Size::new(320, 480),
		true,
		Align {
			horizontal: HorizontalAlign::Center { columns: 2 },
			vertical:   VerticalAlign::Center { rows: 2 },
		},
	);

	let button1 = Button::new("HUI", Size::new(100, 50))?;
	let checkbox = Checkbox::new(Size::new(50, 50));
	let label = Label::new("ZALUPA", Size::new(50, 50))?;

	let id1 = page.insert(WidgetKind::Button(button1))?;
	let id2 = page.insert(WidgetKind::Checkbox(checkbox))?;
	let id3 = page.insert(WidgetKind::Label(label))?;

	let elements = WidgetIDs {
		ids: [id1, id2, id3],
	};

	Ok((page, elements))
}

fn settings_page() -> Result<(Page<10>, WidgetIDs<5>), embed_ui::Error> {
	let mut page = Page::new(
		Size::new(320, 480),
		true,
		Align {
			horizontal: HorizontalAlign::Left,
			vertical:   VerticalAlign::Top,
		},
	);

	let button1 = Button::new("SADAW", Size::new(100, 50))?;
	let checkbox = Checkbox::new(Size::new(50, 50));
	let label = Label::new("KJASD", Size::new(50, 50))?;
	let separator = Separator::new(Size::new(320, 6));
	let textbox = Textbox::new("ADAW", Size::new(320, 100))?;

	let id1 = page.insert(WidgetKind::Button(button1))?;
	let id2 = page.insert(WidgetKind::Checkbox(checkbox))?;
	let id3 = page.insert(WidgetKind::Label(label))?;
	let id4 = page.insert_next_row(WidgetKind::Separator(separator))?;
	let id5 = page.insert_next_row(WidgetKind::Textbox(textbox))?;

	let elements = WidgetIDs {
		ids: [id1, id2, id3, id4, id5],
	};

	Ok((page, elements))
}
