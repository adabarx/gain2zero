use std::sync::Arc;

use atomic_float::AtomicF32;
use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::*;
use nih_plug_iced::widgets as nih_widgets;

use crate::GainToZeroParams;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(200, 300)
}

pub(crate) fn create(
    params: Arc<GainToZeroParams>,
    attentuation_readout: Arc<AtomicF32>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<G2Xui>(editor_state, (params, attentuation_readout))
}

struct G2Xui {
    params: Arc<GainToZeroParams>,
    context: Arc<dyn GuiContext>,
    attenuation_readout: Arc<AtomicF32>,

    threshold_slider_state: nih_widgets::param_slider::State,
    reset_switch_state: nih_widgets::param_slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for G2Xui {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (Arc<GainToZeroParams>, Arc<AtomicF32>);

    fn new(
        (params, attenuation_readout): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = G2Xui {
            params,
            context,
            attenuation_readout,

            threshold_slider_state: Default::default(),
            reset_switch_state: Default::default(),
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                Text::new("gain2zero")
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(40)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Bottom),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new("current attenuation")
                    .font(assets::NOTO_SANS_BOLD)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
            )
            .push(
                Text::new(format!("{:.1}db", self.attenuation_readout.load(std::sync::atomic::Ordering::Relaxed)))
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new("threshold")
                    .font(assets::NOTO_SANS_BOLD)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
            )
            .push(
                nih_widgets::ParamSlider::new(&mut self.threshold_slider_state, &self.params.threshold)
                    .map(Message::ParamUpdate),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new("reset attenuation")
                    .font(assets::NOTO_SANS_BOLD)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
            )
            .push(
                nih_widgets::ParamSlider::new(&mut self.reset_switch_state, &self.params.reset)
                    .map(Message::ParamUpdate),
            )
            .into()
    }

    fn background_color(&self) -> Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.,
        }
    }
}

