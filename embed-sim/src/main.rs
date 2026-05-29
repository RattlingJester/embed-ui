use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{
	OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
	sdl2::{Keycode, MouseButton},
};

use embed_ui::{
	DrawTarget, Rgb666,
	container::{Align, HorizontalAlign, Page, VerticalAlign, WidgetId},
	input::{Event, Interaction},
	painter::SplitPainter,
	style::DEFAULT_STYLE_666,
	ui::Ui,
	widgets::{
		WidgetKind, button::Button, checkbox::Checkbox, label::Label, separator::Separator,
		textbox::Textbox,
	},
};
use profont::{PROFONT_18_POINT, PROFONT_24_POINT};

const SCREEN_W: usize = 320;
const SCREEN_H: usize = 480;
const SCREEN_PIXELS_COUNT: usize = SCREEN_W * SCREEN_H;

const STRIP_COUNT: usize = 10;
const STRIP_H: usize = SCREEN_H.saturating_div(STRIP_COUNT);
const STRIP_PIXEL_COUNT: usize = SCREEN_W * STRIP_H;

struct Elements<const B: usize, const T: usize> {
	buttons:         [WidgetId; B],
	joint_textboxes: [WidgetId; T],
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Pages {
	Main     = 0,
	Settings = 1,
}

struct WidgetIDs<const WIDGET_COUNT: usize> {
	pub ids: [WidgetId; WIDGET_COUNT],
}

fn main() {
	let (page_main, elements_main) = main_page::<50>().unwrap();
	let (page_settings, _elements_sett) = settings_page().unwrap();

	let mut display = SimulatorDisplay::<Rgb666>::new(Size::new(320, 480));

	let output_settings = OutputSettingsBuilder::new().scale(2).build();
	let mut window = Window::new("Test", &output_settings);

	window.update(&display);
	display.clear(embed_ui::style::DEFAULT_STYLE_666.screen_bg);

	println!(
		"Screen pixels count: {SCREEN_PIXELS_COUNT}, strip count: {STRIP_COUNT}, strip height: {STRIP_H}"
	);

	let mut buf = [0; STRIP_PIXEL_COUNT * 3];

	let painter: SplitPainter<10, SCREEN_W, STRIP_H, _> = SplitPainter::new();

	let mut ui = Ui::new([page_main, page_settings], painter, DEFAULT_STYLE_666);

	let mut interaction = None;

	'run: loop {
		window.update(&display);

		for event in window.events() {
			match event {
				SimulatorEvent::Quit => break 'run,

				SimulatorEvent::MouseButtonDown {
					mouse_btn: MouseButton::Left,
					point,
				} => interaction = Some(Interaction::Click(point)),

				SimulatorEvent::MouseButtonUp {
					mouse_btn: MouseButton::Left,
					point,
				} => interaction = Some(Interaction::Release(point)),

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

				SimulatorEvent::KeyDown {
					keycode: Keycode::NUM_1,
					keymod: _,
					repeat: _,
				} => {
					ui.switch_to_page(0);
				}

				SimulatorEvent::KeyDown {
					keycode: Keycode::NUM_2,
					keymod: _,
					repeat: _,
				} => {
					ui.switch_to_page(1);
				}

				_ => (),
			}
		}

		while let Some(event) = ui.drain_events() {
			match event {
				Event::ButtonClicked {
					page_idx,
					widget_id,
				} => {
					println!("Button ID: {widget_id} clicked at page: {page_idx}");
				}
				Event::CheckboxToggled {
					page_idx,
					widget_id,
				} => {
					println!("Checkbox ID: {widget_id} checked at page: {page_idx}");
				}
			}
		}

		ui.begin_frame(interaction.take());

		let pixel_slice = unsafe { &mut *(buf.as_mut_ptr() as *mut [Rgb666; STRIP_PIXEL_COUNT]) };

		for strip in 0..STRIP_COUNT {
			let rect = ui.paint_strip(strip, pixel_slice).unwrap();
			display.fill_contiguous(&rect, *pixel_slice).unwrap();
		}

		ui.end_frame();
	}
}

fn main_page<const W: usize>() -> Result<(Page<W>, Elements<3, 6>), embed_ui::Error> {
	let mut page = Page::new(
		Size::new(SCREEN_W as u32, SCREEN_H as u32),
		true,
		Align {
			horizontal: HorizontalAlign::Left,
			vertical:   VerticalAlign::Top,
		},
	);

	let left_button = Button::new("<", &PROFONT_24_POINT, Size::new(50, 50))?;
	let menu_button = Button::new("Joint jog", &PROFONT_24_POINT, Size::new(220, 50))?;
	let right_button = Button::new(">", &PROFONT_24_POINT, Size::new(50, 50))?;

	let left_button_id = page.insert(WidgetKind::Button(left_button))?;
	let menu_id = page.insert(WidgetKind::Button(menu_button))?;
	let right_button_id = page.insert(WidgetKind::Button(right_button))?;

	let mut joint_textboxes = [0; 6];
	for row in 0..6 {
		let text: heapless::String<10> = heapless::format!("J{}", row).unwrap_or_default();
		let label = Label::new(&text, &PROFONT_18_POINT, Size::new(50, 50))?;
		let textbox = Textbox::new("", &PROFONT_24_POINT, Size::new(200, 50))?;
		let button = Button::new("<0>", &PROFONT_18_POINT, Size::new(50, 50))?;

		let _ = page.insert(WidgetKind::Label(label))?;
		let textbox_id = page.insert(WidgetKind::Textbox(textbox))?;
		let button = page.insert(WidgetKind::Button(button))?;

		joint_textboxes[row] = textbox_id;
	}

	let e = Elements {
		buttons: [left_button_id, menu_id, right_button_id],
		joint_textboxes,
	};

	Ok((page, e))
}

fn settings_page<const W: usize>() -> Result<(Page<W>, WidgetIDs<5>), embed_ui::Error> {
	let mut page = Page::new(
		Size::new(320, 480),
		true,
		Align {
			horizontal: HorizontalAlign::Left,
			vertical:   VerticalAlign::Top,
		},
	);

	let button = Button::new("SADAW", &embed_ui::ascii::FONT_10X20, Size::new(100, 50))?;
	let checkbox = Checkbox::new(Size::new(50, 50));
	let label = Label::new("KJASD", &embed_ui::ascii::FONT_10X20, Size::new(50, 50))?;
	let separator = Separator::new(Size::new(320, 6));
	let textbox = Textbox::new("ADAW", &embed_ui::ascii::FONT_10X20, Size::new(320, 100))?;

	let id1 = page.insert(WidgetKind::Button(button))?;
	let id2 = page.insert(WidgetKind::Checkbox(checkbox))?;
	let id3 = page.insert(WidgetKind::Label(label))?;
	let id4 = page.insert_next_row(WidgetKind::Separator(separator))?;
	let id5 = page.insert_next_row(WidgetKind::Textbox(textbox))?;

	let elements = WidgetIDs {
		ids: [id1, id2, id3, id4, id5],
	};

	Ok((page, elements))
}
