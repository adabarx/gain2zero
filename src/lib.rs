#![allow(non_snake_case)]
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::sync::Arc;
use atomic_float::AtomicF32;

mod editor;


struct GainToZero {
    params: Arc<GainToZeroParams>,
    attenuation: f32,
    attenuation_readout: Arc<AtomicF32>,
}

#[derive(Params)]
struct GainToZeroParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,

    #[id = "threshold"]
    pub threshold: FloatParam,

    #[id = "reset"]
    pub reset: BoolParam,
}

impl Default for GainToZero {
    fn default() -> Self {
        Self {
            params: Arc::new(GainToZeroParams::default()),
            attenuation: 1.,
            attenuation_readout: Arc::new(AtomicF32::new(1.)),
        }
    }
}

impl Default for GainToZeroParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            threshold: FloatParam::new(
                "Threshold",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-36.0),
                    max: util::db_to_gain(36.0),
                    factor: FloatRange::gain_skew_factor(-36.0, 36.0),
                },
            )
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            reset: BoolParam::new("reset", false),
        }
    }
}

impl Plugin for GainToZero {
    const NAME: &'static str = "Gain2Zero";
    const VENDOR: &'static str = "Katlyn Thomas";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "katlyn.c.thomas@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];


    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.attenuation_readout.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let threshold = self.params.threshold.value();
            let reset = self.params.reset.value();

            if !reset {
                for sample in channel_samples {
                    let sample_abs = sample.abs();
                    if sample_abs * self.attenuation > threshold {
                        self.attenuation = threshold / sample_abs;
                    }
                    *sample *= self.attenuation;
                }
            } else {
                self.attenuation = 1.;
            }

            if self.params.editor_state.is_open() {
                self.attenuation_readout
                    .store(
                        util::gain_to_db(self.attenuation),
                        std::sync::atomic::Ordering::Relaxed,
                    )
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for GainToZero {
    const CLAP_ID: &'static str = "com.your-domain.Gain2Zero";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Reactively lowers gain so that samples never exceed the threshold");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Utility];
}

impl Vst3Plugin for GainToZero {
    const VST3_CLASS_ID: [u8; 16] = *b"Exactly16Chars!!";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Tools];
}

nih_export_clap!(GainToZero);
nih_export_vst3!(GainToZero);
