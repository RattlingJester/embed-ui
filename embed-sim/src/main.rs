use embedded_graphics_simulator::{
	OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
	sdl2::{Keycode, MouseButton},
};

use embed_ui::{
	DrawTarget, Error, FrameBuf, PixelColor, Rgb666, RgbColor, Size,
	alloc::{Allocator, Arena},
	input::{Event, Interaction},
	page::{Align, HorizontalAlign, Page, VerticalAlign, WidgetId},
	painter::{Painter, SplitPainter},
	style::DEFAULT_STYLE_666,
	ui::Ui,
	widgets::{
		Horizontal, TextAlignment, Vertical, button::Button, label::Label,
		radio_button::RadioButton, textbox::Textbox,
	},
};
use profont::{PROFONT_18_POINT, PROFONT_24_POINT};

use static_cell::ConstStaticCell;
use vector_protocol::KinematicState;

pub const SCREEN_W: usize = 320;
pub const SCREEN_H: usize = 480;

pub const STRIP_COUNT: usize = 8;
pub const STRIP_H: usize = SCREEN_H.saturating_div(STRIP_COUNT);
pub const STRIP_PIXEL_COUNT: usize = SCREEN_W * STRIP_H;

const STEPS_STR: [&str; 4] = ["0.01", "0.1", "1", "10"];
pub const STEPS_VAL: [f32; 4] = [0.01, 0.1, 1.0, 10.0];

#[derive(Debug, Default)]
pub struct UiState {
	pub focused_joint: usize,
	pub step_idx:      usize,
}

static ARENA: ConstStaticCell<Arena<2048>> = ConstStaticCell::new(Arena::new());
static FRAMEBUF: ConstStaticCell<[Rgb666; STRIP_PIXEL_COUNT]> =
	ConstStaticCell::new([Rgb666::WHITE; STRIP_PIXEL_COUNT]);

fn main() {
	let (page_main, main_elements) = build_main_page::<_, 30, _>(ARENA.take()).unwrap();

	let mut ui_state = UiState {
		focused_joint: 0,
		step_idx:      2,
	};

	let mut display = SimulatorDisplay::<Rgb666>::new(Size::new(320, 480));
	let output_settings = OutputSettingsBuilder::new().scale(2).build();
	let mut window = Window::new("Test", &output_settings);

	window.update(&display);
	display.clear(embed_ui::style::DEFAULT_STYLE_666.screen_bg);

	let fb = FRAMEBUF.take();
	let mut buf = FrameBuf::new(*fb, SCREEN_W, STRIP_H);

	let painter: SplitPainter<STRIP_COUNT, SCREEN_W, STRIP_H> = SplitPainter::new();

	let mut ui = Ui::new([page_main], painter, DEFAULT_STYLE_666);

	let kin_state = KinematicState::default();
	let mut new_state = KinematicState::default();
	let mut interaction = None;

	if let Some((w, _)) = ui
		.current_page_mut()
		.get_mut(main_elements.steps_radio[ui_state.step_idx])
	{
		w.set_active(true);
	}

	ui.current_page_mut()
		.focus_set(main_elements.joint_textboxes[ui_state.focused_joint]);

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
					ui_state.focused_joint = main_elements
						.joint_textboxes
						.iter()
						.position(|&i| i == page.focused())
						.unwrap();
				}

				SimulatorEvent::KeyDown {
					keycode: Keycode::D,
					keymod: _,
					repeat: _,
				} => {
					let page = ui.current_page_mut();
					page.focus_next();
					ui_state.focused_joint = main_elements
						.joint_textboxes
						.iter()
						.position(|&i| i == page.focused())
						.unwrap();
				}

				SimulatorEvent::KeyDown {
					keycode: Keycode::W,
					keymod: _,
					repeat: _,
				} => {
					new_state.positions[ui_state.focused_joint] +=
						STEPS_VAL[ui_state.step_idx].to_radians();

					update_joint_angle(&mut ui, &main_elements, &new_state, ui_state.focused_joint)
						.unwrap();
				}

				SimulatorEvent::KeyDown {
					keycode: Keycode::S,
					keymod: _,
					repeat: _,
				} => {
					new_state.positions[ui_state.focused_joint] -=
						STEPS_VAL[ui_state.step_idx].to_radians();

					update_joint_angle(&mut ui, &main_elements, &new_state, ui_state.focused_joint)
						.unwrap();
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
			let Event {
				page_idx,
				widget_id,
			} = event;
			if page_idx == 0 {
				if widget_id == main_elements.btn_menu {
					ui.switch_to_page(1);
				} else if widget_id == main_elements.btn_left {
					ui.prev_page();
				} else if widget_id == main_elements.btn_right {
					ui.next_page();
				} else if widget_id == main_elements.btn_reset {
					new_state = kin_state;
					for i in 0..6 {
						update_joint_angle(&mut ui, &main_elements, &new_state, i).unwrap();
					}
				} else if widget_id == main_elements.btn_run {
					// kin_state = new_state;

					todo!("Send JOG to controller");
				} else if let Some(joint) = main_elements
					.joint_textboxes
					.iter()
					.position(|&i| i == widget_id)
				{
					ui.current_page_mut().focus_set(widget_id);
					ui_state.focused_joint = joint;
				} else if let Some(joint_zero) = main_elements
					.joint_zeros
					.iter()
					.position(|&i| i == widget_id)
				{
					new_state.positions[joint_zero] = 0.0;
					update_joint_angle(&mut ui, &main_elements, &new_state, joint_zero).unwrap();
				} else if widget_id == main_elements.steps_radio[ui_state.step_idx] {
					if let Some((b, _)) = ui.current_page_mut().get_mut(widget_id) {
						b.set_active(true);
					}
				} else {
					if let Some((b, _)) = ui
						.current_page_mut()
						.get_mut(main_elements.steps_radio[ui_state.step_idx])
					{
						b.set_active(false);
					}
					if let Some((b, _)) = ui.current_page_mut().get_mut(widget_id) {
						b.set_active(true);
						ui_state.step_idx = main_elements
							.steps_radio
							.iter()
							.position(|&e| e == widget_id)
							.unwrap();
					}
					if let Some(step_index) = main_elements
						.steps_radio
						.iter()
						.position(|&id| id == widget_id)
					{
						ui_state.step_idx = step_index;
					}
				}
			}
		}

		ui.begin_frame(interaction.take());

		for strip in 0..STRIP_COUNT {
			let rect = ui.draw(strip, &mut buf).unwrap();

			display.fill_contiguous(&rect, buf.data).unwrap();
		}

		ui.end_frame();
	}
}

pub struct MainElements {
	pub btn_left:        WidgetId,
	pub btn_menu:        WidgetId,
	pub btn_right:       WidgetId,
	pub joint_textboxes: [WidgetId; 6],
	pub joint_zeros:     [WidgetId; 6],
	pub steps_radio:     [WidgetId; 4],
	pub status_textbox:  WidgetId,
	pub mode_textbox:    WidgetId,
	pub btn_run:         WidgetId,
	pub btn_reset:       WidgetId,
}

pub fn build_main_page<'a, A: Allocator<'a>, const W: usize, const F: usize>(
	alloc: &'a mut A,
) -> Result<(Page<'a, Rgb666, A, W, F>, MainElements), embed_ui::Error> {
	let mut page = Page::new(
		alloc,
		Size::new(SCREEN_W as u32, SCREEN_H as u32),
		true,
		Align {
			horizontal: HorizontalAlign::Left,
			vertical:   VerticalAlign::Top,
		},
	);

	let btn_left = page.insert(Button::new(
		"<",
		&PROFONT_24_POINT,
		TextAlignment {
			horizontal: Horizontal::Center,
			vertical:   Vertical::Center,
		},
		Size::new(50, 50),
		false,
	)?)?;
	let btn_menu = page.insert(Button::new(
		"Joint jog",
		&PROFONT_24_POINT,
		TextAlignment {
			horizontal: Horizontal::Center,
			vertical:   Vertical::Center,
		},
		Size::new(220, 50),
		false,
	)?)?;
	let btn_right = page.insert(Button::new(
		">",
		&PROFONT_24_POINT,
		TextAlignment {
			horizontal: Horizontal::Center,
			vertical:   Vertical::Center,
		},
		Size::new(50, 50),
		false,
	)?)?;

	let mut joint_textboxes = [WidgetId::default(); 6];
	let mut joint_zeros = [WidgetId::default(); 6];
	for row in 0..6 {
		let text: heapless::String<10> = heapless::format!("J{}", row + 1).unwrap_or_default();
		let _ = page.insert(Label::new(
			&text,
			&PROFONT_24_POINT,
			TextAlignment {
				horizontal: Horizontal::Center,
				vertical:   Vertical::Center,
			},
			Size::new(50, 50),
		)?)?;

		let textbox_id = page.insert(Button::new(
			"0.000",
			&PROFONT_24_POINT,
			TextAlignment {
				horizontal: Horizontal::Center,
				vertical:   Vertical::Center,
			},
			Size::new(220, 50),
			true,
		)?)?;
		joint_textboxes[row] = textbox_id;

		let zero_id = page.insert(Button::new(
			"<0>",
			&PROFONT_18_POINT,
			TextAlignment {
				horizontal: Horizontal::Center,
				vertical:   Vertical::Center,
			},
			Size::new(50, 50),
			false,
		)?)?;
		joint_zeros[row] = zero_id;
	}

	let mut steps_radio = [WidgetId::default(); 4];
	for row in 0..4 {
		let radio_id = page.insert(RadioButton::new(
			STEPS_STR[row],
			&PROFONT_24_POINT,
			Size::new(80, 50),
			false,
		)?)?;
		steps_radio[row] = radio_id;
	}

	let status_textbox = page.insert(Textbox::new(
		"DISCONNECTED",
		&PROFONT_18_POINT,
		TextAlignment {
			horizontal: Horizontal::Left,
			vertical:   Vertical::Center,
		},
		Size::new(240, 30),
		false,
	)?)?;
	let mode_textbox = page.insert(Textbox::new(
		"JOG",
		&PROFONT_18_POINT,
		TextAlignment {
			horizontal: Horizontal::Center,
			vertical:   Vertical::Center,
		},
		Size::new(80, 30),
		false,
	)?)?;

	let btn_run = page.insert(Button::new(
		"RUN",
		&PROFONT_18_POINT,
		TextAlignment {
			horizontal: Horizontal::Center,
			vertical:   Vertical::Center,
		},
		Size::new(160, 50),
		false,
	)?)?;
	let btn_reset = page.insert(Button::new(
		"RESET",
		&PROFONT_18_POINT,
		TextAlignment {
			horizontal: Horizontal::Center,
			vertical:   Vertical::Center,
		},
		Size::new(160, 50),
		false,
	)?)?;

	Ok((
		page,
		MainElements {
			btn_left,
			btn_menu,
			btn_right,
			joint_textboxes,
			joint_zeros,
			steps_radio,
			status_textbox,
			mode_textbox,
			btn_run,
			btn_reset,
		},
	))
}

fn update_joint_angle<
	'a,
	const W: usize,
	const PAGES: usize,
	const FB_SIZE: usize,
	A: Allocator<'a>,
	C: PixelColor,
	P: Painter<'a>,
>(
	ui: &mut Ui<'a, W, PAGES, FB_SIZE, A, C, P>,
	elements: &MainElements,
	kin_state: &KinematicState,
	joint_idx: usize,
) -> Result<(), Error> {
	let text: heapless::String<32> =
		heapless::format!("{:.3}", kin_state.positions[joint_idx].to_degrees()).unwrap_or_default();

	if let Some((t, _)) = ui
		.get_page_mut(0)
		.get_mut(elements.joint_textboxes[joint_idx])
	{
		t.set_text(&text)?;
	}

	Ok(())
}

// fn update_status_textbox<
// 	const W: usize,
// 	const PAGES: usize,
// 	const FB_SIZE: usize,
// 	A: Allocator + 'static,
// 	C: PixelColor,
// 	P: Painter,
// >(
// 	ui: &mut Ui<W, PAGES, FB_SIZE, A, C, P>,
// 	elements: &MainElements,
// 	kin_state: &KinematicState,
// ) -> Result<(), Error> {
// 	if let Some((t, _)) = ui.get_page_mut(0).get_mut(elements.status_textbox) {
// 		t.set_text(kin_state.state.as_ref())?;
// 	}

// 	Ok(())
// }
