#![allow(unused)]

use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{
	OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
	sdl2::{Keycode, MouseButton},
};

use embed_ui::{
	DrawTarget, Rgb666,
	input::{Event, Interaction},
	page::{Align, HorizontalAlign, Page, VerticalAlign, WidgetId},
	painter::SplitPainter,
	style::DEFAULT_STYLE_666,
	ui::Ui,
	widgets::{
		WidgetKind, button::Button, checkbox::Checkbox, label::Label, radio_button::RadioButton,
		separator::Separator, textbox::Textbox,
	},
};
use profont::{PROFONT_18_POINT, PROFONT_24_POINT};

const SCREEN_W: usize = 320;
const SCREEN_H: usize = 480;
const SCREEN_PIXELS_COUNT: usize = SCREEN_W * SCREEN_H;

const STRIP_COUNT: usize = 10;
const STRIP_H: usize = SCREEN_H.saturating_div(STRIP_COUNT);
const STRIP_PIXEL_COUNT: usize = SCREEN_W * STRIP_H;

const JOINT_STEPS: [&str; 4] = ["0.01", "0.1", "1", "10"];
const MODES: [&str; 2] = ["JOG", "CART"];

struct Elements<const B: usize, const T: usize> {
	buttons:         [WidgetId; B],
	joint_textboxes: [WidgetId; T],
	steps:           [WidgetId; 4],
	status_id:       WidgetId,
	mode_id:         WidgetId,
}

#[derive(Debug, Default)]
struct UiState {
	focused_joint: usize,
	mode:          usize,
	step:          f32,
	positions:     [f32; 6],
	status:        RobotState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum RobotState {
	#[default]
	Init,
	Idle,
	Disconnected,
	Moving {
		cmd_id: u8,
	},
	MoveError,
	EStop,
}

impl RobotState {
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::Init => "Init",
			Self::Idle => "Idle",
			Self::Disconnected => "Disconnected",
			Self::Moving { cmd_id: _ } => "Moving",
			Self::MoveError => "MoveError",
			Self::EStop => "EStop",
		}
	}
}

struct WidgetIDs<const WIDGET_COUNT: usize> {
	pub ids: [WidgetId; WIDGET_COUNT],
}

fn main() {
	let mut state = UiState {
		focused_joint: 0,
		mode: 0,
		step: 0.01,
		..Default::default()
	};

	let (page_main, mut elements_main) = main_page::<50>(&state).unwrap();
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

	if let Some(WidgetKind::RadioButton(b)) = ui.current_page_mut().get_mut(elements_main.steps[0])
	{
		b.set_toggle(true);
	}

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
				Event::RadioButtonToggled {
					page_idx: 0,
					widget_id,
				} if widget_id != elements_main.mode_id => {
					if let Some(WidgetKind::RadioButton(b)) =
						ui.current_page_mut().get_mut(elements_main.mode_id)
					{
						b.set_toggle(false);
					}

					if let Some(WidgetKind::RadioButton(b)) =
						ui.current_page_mut().get_mut(widget_id)
					{
						b.set_toggle(true);
						elements_main.mode_id = widget_id;
					}
				}

				_ => (),
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

fn main_page<const W: usize>(
	state: &UiState,
) -> Result<(Page<W>, Elements<3, 6>), embed_ui::Error> {
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
		let text: heapless::String<10> = heapless::format!("J{}", row + 1).unwrap_or_default();
		let label = Label::new(&text, &PROFONT_24_POINT, Size::new(50, 50))?;
		let textbox = Textbox::new("", &PROFONT_24_POINT, Size::new(220, 50))?;
		let button = Button::new("<0>", &PROFONT_18_POINT, Size::new(50, 50))?;

		let _ = page.insert(WidgetKind::Label(label))?;
		let textbox_id = page.insert(WidgetKind::Textbox(textbox))?;
		let _button = page.insert(WidgetKind::Button(button))?;

		joint_textboxes[row] = textbox_id;
	}

	let mut steps = [0; 4];
	for row in 0..4 {
		let button = RadioButton::new(
			JOINT_STEPS[row],
			&PROFONT_24_POINT,
			Size::new(80, 50),
			false,
		)?;
		let id = page.insert(WidgetKind::RadioButton(button))?;
		steps[row] = id;
	}

	let status = Textbox::new(state.status.as_str(), &PROFONT_18_POINT, Size::new(240, 30))?;
	let status_id = page.insert(WidgetKind::Textbox(status))?;

	let mode = Textbox::new(MODES[state.mode], &PROFONT_18_POINT, Size::new(80, 30))?;
	let mode_id = page.insert(WidgetKind::Textbox(mode))?;

	let run = Button::new("RUN", &PROFONT_18_POINT, Size::new(160, 50))?;
	let reset = Button::new("RESET", &PROFONT_18_POINT, Size::new(160, 50))?;

	let _run_id = page.insert(WidgetKind::Button(run))?;
	let _reset_id = page.insert(WidgetKind::Button(reset))?;

	let e = Elements {
		buttons: [left_button_id, menu_id, right_button_id],
		joint_textboxes,
		steps,
		status_id,
		mode_id,
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
