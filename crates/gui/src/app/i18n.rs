use std::{borrow::Cow, collections::HashMap, env, fmt::Display, sync::LazyLock};

use gstreamer::ClockTime;
use i18n_embed::{LanguageLoader, fluent::FluentLanguageLoader};
use log::debug;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

use crate::app::error::ApplicationError;

#[derive(RustEmbed)]
#[folder = "../../i18n"]
struct Localizations;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    English,
    SimplifiedChinese,
    Russian,
    Japanese,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageInfo {
    pub language: Language,
    pub id: &'static str,
    pub display_name: &'static str,
}

pub const LANGUAGES: &[LanguageInfo] = &[
    LanguageInfo {
        language: Language::English,
        id: "en",
        display_name: "English",
    },
    LanguageInfo {
        language: Language::SimplifiedChinese,
        id: "zh-CN",
        display_name: "\u{7b80}\u{4f53}\u{4e2d}\u{6587}",
    },
    LanguageInfo {
        language: Language::Russian,
        id: "ru",
        display_name: "\u{0420}\u{0443}\u{0441}\u{0441}\u{043a}\u{0438}\u{0439}",
    },
    LanguageInfo {
        language: Language::Japanese,
        id: "ja",
        display_name: "\u{65e5}\u{672c}\u{8a9e}",
    },
];

static LANGUAGE_LOADERS: LazyLock<[FluentLanguageLoader; 4]> = LazyLock::new(|| {
    Language::ALL.map(|language| {
        let fallback = parse_language_identifier("en");
        let loader = FluentLanguageLoader::new("ntsc-rs", fallback.clone());
        let language_id = parse_language_identifier(language.storage_key());
        let requested_languages = if language_id == fallback {
            vec![fallback]
        } else {
            vec![language_id, fallback]
        };

        loader
            .load_languages(&Localizations, &requested_languages)
            .expect("embedded Fluent resources should load");
        loader.set_use_isolating(false);
        loader
    })
});

impl Language {
    pub const ALL: [Self; 4] = [
        Self::English,
        Self::SimplifiedChinese,
        Self::Russian,
        Self::Japanese,
    ];

    pub fn detect() -> Self {
        sys_locale::get_locale()
            .as_deref()
            .and_then(Self::from_locale)
            .or_else(|| {
                ["LC_ALL", "LANG", "LANGUAGE"]
                    .iter()
                    .filter_map(|name| env::var(name).ok())
                    .find_map(|locale| Self::from_locale(&locale))
            })
            .unwrap_or_default()
    }

    pub fn storage_key(self) -> &'static str {
        self.info().id
    }

    pub fn from_storage_key(value: &str) -> Option<Self> {
        let normalized = normalize_language_id(value);
        match normalized.as_str() {
            "zh" | "zh-cn" | "zh-hans" => Some(Self::SimplifiedChinese),
            _ => LANGUAGES
                .iter()
                .find(|info| normalize_language_id(info.id) == normalized)
                .map(|info| info.language),
        }
    }

    pub fn display_name(self) -> &'static str {
        self.info().display_name
    }

    pub fn tr(self, key: &str) -> String {
        let loader = self.loader();
        if loader.has(key) {
            loader.get(key)
        } else {
            debug!(target: "ntsc_rs_gui::i18n", "missing Fluent message id: {key}");
            key.to_owned()
        }
    }

    pub fn tr_args(self, key: &str, args: &[(&str, String)]) -> String {
        let loader = self.loader();
        if !loader.has(key) {
            debug!(target: "ntsc_rs_gui::i18n", "missing Fluent message id: {key}");
            return key.to_owned();
        }

        let args = HashMap::from_iter(args.iter().map(|(name, value)| (*name, value.as_str())));
        loader.get_args(key, args)
    }

    pub fn text<'a>(self, text: &'a str) -> Cow<'a, str> {
        match english_text_to_key(text) {
            Some(key) => Cow::Owned(self.tr(key)),
            None => {
                debug!(
                    target: "ntsc_rs_gui::i18n",
                    "missing compatibility mapping for English source text: {text}"
                );
                Cow::Borrowed(text)
            }
        }
    }

    pub fn latest_version_label(self, tag_name: &str) -> String {
        self.tr_args(
            "update-latest-version",
            &[("tag_name", tag_name.to_owned())],
        )
    }

    pub fn rendering_progress(self, position: ClockTime, duration: ClockTime) -> String {
        self.tr_args(
            "render-rendering-progress",
            &[
                ("position", format!("{position:.2}")),
                ("duration", format!("{duration:.2}")),
            ],
        )
    }

    pub fn completed_in(self, duration: ClockTime) -> String {
        self.tr_args(
            "render-completed-in",
            &[("duration", format!("{duration:.2}"))],
        )
    }

    pub fn render_error(self, error: &str) -> String {
        self.tr_args("render-error", &[("error", error.to_owned())])
    }

    pub fn format_eta(self, time_remaining: f64, separator: &str) -> String {
        let mut time_remaining = time_remaining.ceil() as u64;
        let hours = time_remaining / (60 * 60);
        time_remaining %= 60 * 60;
        let minutes = time_remaining / 60;
        let seconds = time_remaining % 60;

        let mut parts = Vec::new();
        if hours > 0 {
            parts.push(self.tr_count("eta-hour", hours));
        }
        if minutes > 0 {
            parts.push(self.tr_count("eta-minute", minutes));
        }
        parts.push(self.tr_count("eta-second", seconds));
        parts.join(separator)
    }

    pub fn eta_units(self) -> [[&'static str; 2]; 3] {
        match self {
            Self::English => [
                [" hour", " hours"],
                [" minute", " minutes"],
                [" second", " seconds"],
            ],
            Self::SimplifiedChinese => [
                [" \u{5c0f}\u{65f6}", " \u{5c0f}\u{65f6}"],
                [" \u{5206}\u{949f}", " \u{5206}\u{949f}"],
                [" \u{79d2}", " \u{79d2}"],
            ],
            Self::Russian => [
                [
                    " \u{0447}\u{0430}\u{0441}",
                    " \u{0447}\u{0430}\u{0441}\u{043e}\u{0432}",
                ],
                [
                    " \u{043c}\u{0438}\u{043d}\u{0443}\u{0442}\u{0430}",
                    " \u{043c}\u{0438}\u{043d}\u{0443}\u{0442}",
                ],
                [
                    " \u{0441}\u{0435}\u{043a}\u{0443}\u{043d}\u{0434}\u{0430}",
                    " \u{0441}\u{0435}\u{043a}\u{0443}\u{043d}\u{0434}",
                ],
            ],
            Self::Japanese => [
                [" \u{6642}\u{9593}", " \u{6642}\u{9593}"],
                [" \u{5206}", " \u{5206}"],
                [" \u{79d2}", " \u{79d2}"],
            ],
        }
    }

    pub fn format_application_error(self, error: &ApplicationError) -> String {
        match error {
            ApplicationError::GstreamerInit { source } => {
                self.format_source_error("error-gstreamer-init", source)
            }
            ApplicationError::LoadVideo { source } => {
                self.format_source_error("error-load-video", source)
            }
            ApplicationError::CreatePipeline { source } => {
                self.format_source_error("error-create-pipeline", source)
            }
            ApplicationError::CreateRenderJob { source } => {
                self.format_source_error("error-create-render-job", source)
            }
            ApplicationError::RenderJobPipeline { source } => {
                self.format_source_error("error-render-job-pipeline", source)
            }
            ApplicationError::JSONRead { source } => {
                self.format_source_error("error-json-read", source)
            }
            ApplicationError::JSONParse { source } => {
                self.format_source_error("error-json-parse", source)
            }
            ApplicationError::CreatePresetsDirectory { source } => {
                self.format_source_error("error-create-presets-directory", source)
            }
            ApplicationError::CreatePresetFile { source } => {
                self.format_source_error("error-create-preset", source)
            }
            ApplicationError::CreatePresetJSON { source } => {
                self.format_source_error("error-create-preset", source)
            }
            ApplicationError::DeletePreset { source } => {
                self.format_source_error("error-delete-preset", source)
            }
            ApplicationError::RenamePreset { source } => {
                self.format_source_error("error-rename-preset", source)
            }
            ApplicationError::InstallPreset { source } => {
                self.format_source_error("error-install-preset", source)
            }
            ApplicationError::Fs { source } => self.format_source_error("error-filesystem", source),
            ApplicationError::DroppedMultipleFiles => self.tr("error-dropped-multiple-files"),
        }
    }

    fn info(self) -> &'static LanguageInfo {
        LANGUAGES
            .iter()
            .find(|info| info.language == self)
            .expect("Language must be registered in LANGUAGES")
    }

    fn loader(self) -> &'static FluentLanguageLoader {
        &LANGUAGE_LOADERS[self as usize]
    }

    fn tr_count(self, key: &str, count: u64) -> String {
        let loader = self.loader();
        if !loader.has(key) {
            debug!(target: "ntsc_rs_gui::i18n", "missing Fluent message id: {key}");
            return key.to_owned();
        }

        loader.get_args(key, HashMap::from([("count", count)]))
    }

    fn format_source_error(self, key: &str, source: impl Display) -> String {
        self.tr_args(key, &[("source", source.to_string())])
    }

    fn from_locale(locale: &str) -> Option<Self> {
        locale
            .split(':')
            .filter_map(|part| part.split('.').next())
            .filter_map(|part| part.split('@').next())
            .find_map(Self::from_storage_key)
    }
}

fn normalize_language_id(value: &str) -> String {
    value.trim().replace('_', "-").to_ascii_lowercase()
}

fn parse_language_identifier(value: &str) -> LanguageIdentifier {
    value
        .parse()
        .unwrap_or_else(|_| panic!("invalid language identifier: {value}"))
}

fn english_text_to_key(text: &str) -> Option<&'static str> {
    match text {
        " (interlaced)" => Some("ui-interlaced"),
        " (progressive)" => Some("ui-progressive"),
        " (telecined)" => Some("ui-telecined"),
        "...loosely based on " => Some("ui-loosely-based-on"),
        "...which is a GUI for " => Some("ui-which-is-a-gui-for"),
        "...which is a port of " => Some("ui-which-is-a-port-of"),
        "0 degrees" => Some("setting-0-degrees"),
        "1-line comb" => Some("setting-1-line-comb"),
        "10-bit" => Some("setting-10-bit"),
        "12-bit" => Some("setting-12-bit"),
        "180 degrees" => Some("setting-180-degrees"),
        "2-line comb" => Some("setting-2-line-comb"),
        "270 degrees" => Some("setting-270-degrees"),
        "4:2:0 chroma subsampling" => Some("ui-4-2-0-chroma-subsampling"),
        "8-bit" => Some("setting-8-bit"),
        "90 degrees" => Some("setting-90-degrees"),
        "About + Credits" => Some("ui-about-plus-credits"),
        "Additional ringing artifacts, simulated with a notch filter." => {
            Some("setting-additional-ringing-artifacts-simulated-with-a-notch-filter")
        }
        "Alternating" => Some("setting-alternating"),
        "Amount of sharpening to apply." => Some("setting-amount-of-sharpening-to-apply"),
        "An error occurred while loading" => Some("ui-an-error-occurred-while-loading"),
        "Apply a low-pass filter to the input chrominance (color) signal." => {
            Some("setting-apply-a-low-pass-filter-to-the-input-chrominance-color-signal")
        }
        "Apply a low-pass filter to the output chroma signal." => {
            Some("setting-apply-a-low-pass-filter-to-the-output-chroma-signal")
        }
        "Apply a notch filter to the input luminance signal. Sharp, but has ringing artifacts." => {
            Some("setting-apply-a-notch-filter-to-the-input-luminance-signal-sharp-but-ha-d3529ea0")
        }
        "Apply a simple box filter to the input luminance signal." => {
            Some("setting-apply-a-simple-box-filter-to-the-input-luminance-signal")
        }
        "Average the current row with the previous and next ones, phase-cancelling the chrominance (color) signals. Only works if the scanline phase shift is 180 degrees." => {
            Some("setting-average-the-current-row-with-the-previous-and-next-ones-phase-c-aff6759b")
        }
        "Average the current row with the previous and next ones, phase-cancelling the chrominance signals. Only works if the scanline phase shift is 180 degrees." => {
            Some("setting-average-the-current-row-with-the-previous-and-next-ones-phase-c-1174a495")
        }
        "Average the current row with the previous one, phase-cancelling the chrominance (color) signals. Only works if the scanline phase shift is 180 degrees." => {
            Some("setting-average-the-current-row-with-the-previous-one-phase-cancelling-70ba311e")
        }
        "Average the current row with the previous one, phase-cancelling the chrominance signals. Only works if the scanline phase shift is 180 degrees." => {
            Some("setting-average-the-current-row-with-the-previous-one-phase-cancelling-ae21a1da")
        }
        "Base wavelength for the horizontal waving." => {
            Some("setting-base-wavelength-for-the-horizontal-waving")
        }
        "Base wavelength, in pixels, of the noise." => {
            Some("setting-base-wavelength-in-pixels-of-the-noise")
        }
        "Bicubic" => Some("setting-bicubic"),
        "Bilinear" => Some("setting-bilinear"),
        "Bit depth" => Some("render-bit-depth"),
        "Boost high frequencies in the NTSC signal, sharpening the image and intensifying colors." => {
            Some("setting-boost-high-frequencies-in-the-ntsc-signal-sharpening-the-image-27e25392")
        }
        "Both" => Some("setting-both"),
        "Box" => Some("setting-box"),
        "Browse for a path" => Some("action-browse-path"),
        "Butterworth (sharper)" => Some("setting-butterworth-sharper"),
        "Cancel" => Some("action-cancel"),
        "Chance that the chrominance (color) signal is completely lost in each scanline." => {
            Some("setting-chance-that-the-chrominance-color-signal-is-completely-lost-in-2cd0e6b2")
        }
        "Chance that the chrominance signal is completely lost in each scanline." => {
            Some("setting-chance-that-the-chrominance-signal-is-completely-lost-in-each-scanline")
        }
        "Check for Updates" => Some("update-check-title"),
        "Check for Updates..." => Some("update-check-menu"),
        "Checking for updates..." => Some("update-checking"),
        "Choose which rows (\"fields\" in NTSC parlance) of the source image will be used." => {
            Some("setting-choose-which-rows-fields-in-ntsc-parlance-of-the-source-image-w-908d8012")
        }
        "Chroma delay (horizontal)" => Some("setting-chroma-delay-horizontal"),
        "Chroma delay (vertical)" => Some("setting-chroma-delay-vertical"),
        "Chroma demodulation filter" => Some("setting-chroma-demodulation-filter"),
        "Chroma loss" => Some("setting-chroma-loss"),
        "Chroma low-pass in" => Some("setting-chroma-low-pass-in"),
        "Chroma low-pass out" => Some("setting-chroma-low-pass-out"),
        "Chroma noise" => Some("setting-chroma-noise"),
        "Chroma phase error" => Some("setting-chroma-phase-error"),
        "Chroma phase noise" => Some("setting-chroma-phase-noise"),
        "Codec" => Some("render-codec"),
        "Composite signal noise" => Some("setting-composite-signal-noise"),
        "Composite signal sharpening" => Some("setting-composite-signal-sharpening"),
        "Compression level" => Some("render-compression-level"),
        "Compression level for PNG encoding. Higher compression levels produce smaller files but take longer to render." => {
            Some("ui-compression-level-for-png-encoding-higher-compression-levels-pr-cfcac488")
        }
        "Constant K (blurry)" => Some("setting-constant-k-blurry"),
        "Copy" => Some("action-copy"),
        "Copy frame" => Some("action-copy-frame"),
        "Dark" => Some("theme-dark"),
        "Delete" => Some("action-delete"),
        "Destination file:" => Some("render-destination-file"),
        "Detail" => Some("setting-detail"),
        "Determines whether the speckles are placed truly randomly or concentrated in certain rows." => {
            Some("setting-determines-whether-the-speckles-are-placed-truly-randomly-or-co-e81c0876")
        }
        "Disable" => Some("action-disable"),
        "Do not filter the luminance signal. Adds rainbow artifacts." => {
            Some("setting-do-not-filter-the-luminance-signal-adds-rainbow-artifacts")
        }
        "Download from GitHub ⤴" => Some("ui-download-from-github"),
        "Drop to install presets" => Some("dnd-install-presets"),
        "Drop to load media" => Some("dnd-load-media"),
        "Drop to load preset" => Some("dnd-load-preset"),
        "Duration:" => Some("video-duration"),
        "EP (Extended Play)" => Some("setting-ep-extended-play"),
        "Easy mode" => Some("ui-easy-mode"),
        "Edge wave" => Some("setting-edge-wave"),
        "Edit" => Some("menu-edit"),
        "Effect" => Some("tab-effect"),
        "Effect preview" => Some("effect-preview"),
        "Emulate VHS head-switching artifacts at the bottom of the image." => {
            Some("setting-emulate-vhs-head-switching-artifacts-at-the-bottom-of-the-image")
        }
        "Emulate cutoff of high-frequency data at various VHS recording speeds." => {
            Some("setting-emulate-cutoff-of-high-frequency-data-at-various-vhs-recording-speeds")
        }
        "Emulate noise from VHS tracking error." => {
            Some("setting-emulate-noise-from-vhs-tracking-error")
        }
        "Enable" => Some("action-enable"),
        "Encoding speed" => Some("render-encoding-speed"),
        "Encoding speed preset. Higher encoding speeds provide a worse compression ratio, resulting in larger videos at a given quality." => {
            Some("ui-encoding-speed-preset-higher-encoding-speeds-provide-a-worse-co-cfa29612")
        }
        "Error checking for updates:" => Some("update-error-checking"),
        "Error creating pipeline" => Some("error-create-pipeline-title"),
        "Error creating preset" => Some("error-create-preset-title"),
        "Error creating presets directory" => Some("error-create-presets-directory-title"),
        "Error creating render job" => Some("error-create-render-job-title"),
        "Error deleting preset" => Some("error-delete-preset-title"),
        "Error during render job" => Some("error-render-job-pipeline-title"),
        "Error initializing GStreamer" => Some("error-gstreamer-init-title"),
        "Error installing preset" => Some("error-install-preset-title"),
        "Error loading presets directory" => Some("ui-error-loading-presets-directory"),
        "Error loading video" => Some("error-load-video-title"),
        "Error parsing JSON" => Some("error-json-parse-title"),
        "Error reading JSON" => Some("error-json-read-title"),
        "Error renaming preset" => Some("error-rename-preset-title"),
        "FFV1 (Lossless)" => Some("setting-ffv1-lossless"),
        "File" => Some("menu-file"),
        "Filesystem error" => Some("error-filesystem-title"),
        "Filter the input luminance to decrease rainbow artifacts." => {
            Some("setting-filter-the-input-luminance-to-decrease-rainbow-artifacts")
        }
        "Filter used to modulate the chrominance (color) data out of the composite NTSC signal." => {
            Some("setting-filter-used-to-modulate-the-chrominance-color-data-out-of-the-c-9c9b685e")
        }
        "Filter with a sharper falloff. Produces sharpened, less blurry results." => {
            Some("setting-filter-with-a-sharper-falloff-produces-sharpened-less-blurry-results")
        }
        "Fit" => Some("zoom-fit"),
        "Follow system color theme" => Some("theme-system-tooltip"),
        "Frequency" => Some("setting-frequency"),
        "Frequency / radius of the sharpening, relative to the tape speed's cutoff frequency." => {
            Some("setting-frequency-radius-of-the-sharpening-relative-to-the-tape-speed-s-6ece71e3")
        }
        "Frequency of random speckles in the image." => {
            Some("setting-frequency-of-random-speckles-in-the-image")
        }
        "Frequency of speckle-type noise in the artifacts." => {
            Some("setting-frequency-of-speckle-type-noise-in-the-artifacts")
        }
        "Frequency/period of the ringing, in \"rings per pixel\"." => {
            Some("setting-frequency-period-of-the-ringing-in-rings-per-pixel")
        }
        "Full" => Some("setting-full"),
        "Full-intensity low-pass filter." => Some("setting-full-intensity-low-pass-filter"),
        "H.264" => Some("setting-h-264"),
        "Head switching" => Some("setting-head-switching"),
        "Height" => Some("setting-height"),
        "Help" => Some("menu-help"),
        "Horizontal offset of the chrominance (color) signal." => {
            Some("setting-horizontal-offset-of-the-chrominance-color-signal")
        }
        "Horizontal position at which the head-switching starts." => {
            Some("setting-horizontal-position-at-which-the-head-switching-starts")
        }
        "Horizontal scale" => Some("setting-horizontal-scale"),
        "Horizontal shift" => Some("setting-horizontal-shift"),
        "Horizontal waving of the image, in pixels." => {
            Some("setting-horizontal-waving-of-the-image-in-pixels")
        }
        "Horizontal waving of the image." => Some("setting-horizontal-waving-of-the-image"),
        "Horizontally scale the effect by this amount. For 480p video, leave this at 1.0 for the most physically-accurate result." => {
            Some("setting-horizontally-scale-the-effect-by-this-amount-for-480p-video-lea-8fc62d29")
        }
        "How much of the head-switching artifact is off-screen." => {
            Some("setting-how-much-of-the-head-switching-artifact-is-off-screen")
        }
        "How much the affected scanlines \"wave\" back and forth." => {
            Some("setting-how-much-the-affected-scanlines-wave-back-and-forth")
        }
        "How much the head-switching artifact \"jitters\" horizontally." => {
            Some("setting-how-much-the-head-switching-artifact-jitters-horizontally")
        }
        "How much the head-switching artifact shifts rows horizontally." => {
            Some("setting-how-much-the-head-switching-artifact-shifts-rows-horizontally")
        }
        "How much the speckles are clustered by scanline." => {
            Some("setting-how-much-the-speckles-are-clustered-by-scanline")
        }
        "Image sequences do not support interlaced output." => {
            Some("render-image-sequence-no-interlaced")
        }
        "Input luma filter" => Some("setting-input-luma-filter"),
        "Intensity" => Some("setting-intensity"),
        "Intensity of non-speckle noise." => Some("setting-intensity-of-non-speckle-noise"),
        "Intensity of the noise." => Some("setting-intensity-of-the-noise"),
        "Intensity of the ringing." => Some("setting-intensity-of-the-ringing"),
        "Interlaced output" => Some("render-interlaced-output"),
        "Interleaved (lower first)" => Some("setting-interleaved-lower-first"),
        "Interleaved (upper first)" => Some("setting-interleaved-upper-first"),
        "Jitter" => Some("setting-jitter"),
        "LP (Long Play)" => Some("setting-lp-long-play"),
        "Language" => Some("settings-language"),
        "Less intense low-pass filter." => Some("setting-less-intense-low-pass-filter"),
        "License" => Some("help-license"),
        "Light" => Some("theme-light"),
        "Light theme" => Some("theme-light-menu"),
        "Load" => Some("action-load"),
        "Load from..." => Some("action-load-from"),
        "Lower only" => Some("setting-lower-only"),
        "Lower-quality but faster scaling filter" => {
            Some("setting-lower-quality-but-faster-scaling-filter")
        }
        "Lowpass filter type" => Some("setting-lowpass-filter-type"),
        "Luma noise" => Some("setting-luma-noise"),
        "Luma smear" => Some("setting-luma-smear"),
        "Multiply the scaling factors by the video's height. Prefer scaling the input video to 480p instead, which gives much more accurate-looking results." => {
            Some("setting-multiply-the-scaling-factors-by-the-video-s-height-prefer-scali-2e902088")
        }
        "Mute" => Some("ui-mute"),
        "Nearest" => Some("setting-nearest"),
        "Nearest-neighbor (pixelated) scaling. Note that this is still slower than Bilinear" => {
            Some("setting-nearest-neighbor-pixelated-scaling-note-that-this-is-still-slow-585c7ea9")
        }
        "No low-pass filter." => Some("setting-no-low-pass-filter"),
        "No media loaded" => Some("media-empty"),
        "Noise applied per-scanline to the phase of the chrominance (color) signal." => {
            Some("setting-noise-applied-per-scanline-to-the-phase-of-the-chrominance-color-signal")
        }
        "Noise applied per-scanline to the phase of the chrominance signal." => {
            Some("setting-noise-applied-per-scanline-to-the-phase-of-the-chrominance-signal")
        }
        "Noise applied to the chrominance (color) signal." => {
            Some("setting-chroma-noise-color-description")
        }
        "Noise applied to the chrominance signal." => Some("setting-chroma-noise-description"),
        "Noise applied to the composite NTSC signal." => {
            Some("setting-noise-applied-to-the-composite-ntsc-signal")
        }
        "Noise applied to the luminance signal. Useful for higher-frequency noise than the \"Composite noise\" setting can provide." => {
            Some("setting-noise-applied-to-the-luminance-signal-useful-for-higher-frequen-b0f85576")
        }
        "Noise intensity" => Some("setting-noise-intensity"),
        "None" => Some("setting-none"),
        "Notch" => Some("setting-notch"),
        "Notch filter. Sharper than a box blur, but with ringing artifacts." => {
            Some("setting-notch-filter-sharper-than-a-box-blur-but-with-ringing-artifacts")
        }
        "OK" => Some("action-ok"),
        "Octaves of noise for the waves." => Some("setting-octaves-of-noise-for-the-waves"),
        "Octaves of noise." => Some("setting-octaves-of-noise"),
        "Offset" => Some("setting-offset"),
        "Online Documentation ⤴" => Some("ui-online-documentation"),
        "Only one file at a time can be dropped here" => Some("error-dropped-multiple-files-title"),
        "Open" => Some("action-open"),
        "Open containing folder" => Some("action-open-containing-folder"),
        "Open folder" => Some("action-open-folder"),
        "Output directory is not empty" => Some("render-output-directory-not-empty"),
        "Overwrite" => Some("action-overwrite"),
        "PNG Sequence" => Some("setting-png-sequence"),
        "Paste" => Some("action-paste"),
        "Paste JSON" => Some("preset-paste-json"),
        "Paused" => Some("render-paused"),
        "Phase error for the chrominance (color) signal." => {
            Some("setting-phase-error-for-the-chrominance-color-signal")
        }
        "Phase shift of the chrominance (color) signal each scanline. Usually 180 degrees." => {
            Some("setting-phase-shift-of-the-chrominance-color-signal-each-scanline-usual-045a7c3c")
        }
        "Position" => Some("setting-position"),
        "Power" => Some("setting-power"),
        "Presets" => Some("presets"),
        "Quality" => Some("render-quality"),
        "Quit" => Some("action-quit"),
        "Random seed" => Some("setting-random-seed"),
        "Randomize seed" => Some("setting-random-seed-tooltip"),
        "Redo" => Some("action-redo"),
        "Reload" => Some("action-reload"),
        "Rename" => Some("action-rename"),
        "Render" => Some("action-render"),
        "Rendering..." => Some("render-rendering"),
        "Reset" => Some("action-reset"),
        "Resizing filter" => Some("ui-resizing-filter"),
        "Ringing" => Some("setting-ringing"),
        "SP (Standard Play)" => Some("setting-sp-standard-play"),
        "Saturation" => Some("setting-saturation"),
        "Save" => Some("action-save"),
        "Save as" => Some("action-save-as"),
        "Save frame" => Some("action-save-frame"),
        "Save to..." => Some("action-save-to"),
        "Scale" => Some("setting-scale"),
        "Scale the effect by these factors." => Some("setting-scale-the-effect-by-these-factors"),
        "Scale the video prior to applying the effect. Real NTSC footage is 480 lines tall. This applies to both the preview and the final render, and is not saved as part of presets." => {
            Some("ui-scale-the-video-prior-to-applying-the-effect-real-ntsc-footage-ecf48175")
        }
        "Scale to" => Some("video-scale-to"),
        "Scale with video size" => Some("setting-scale-with-video-size"),
        "Scanline phase shift" => Some("setting-scanline-phase-shift"),
        "Scanline phase shift offset" => Some("setting-scanline-phase-shift-offset"),
        "Sharpen" => Some("setting-sharpen"),
        "Sharpening of the image, as done by some VHS decks." => {
            Some("setting-sharpening-of-the-image-as-done-by-some-vhs-decks")
        }
        "Sharper scaling filter; ~20% slower than Bilinear" => {
            Some("setting-sharper-scaling-filter-20-percent-slower-than-bilinear")
        }
        "Simple constant-k filter. Produces longer, blurry results." => {
            Some("setting-simple-constant-k-filter-produces-longer-blurry-results")
        }
        "Simple horizontal box blur." => Some("setting-simple-horizontal-box-blur"),
        "Skip every lower row, keeping the upper ones." => {
            Some("setting-skip-every-lower-row-keeping-the-upper-ones")
        }
        "Skip every other row, alternating between skipping even and odd rows." => {
            Some("setting-skip-every-other-row-alternating-between-skipping-even-and-odd-rows")
        }
        "Skip every upper row, keeping the lower ones." => {
            Some("setting-skip-every-upper-row-keeping-the-lower-ones")
        }
        "Snow" => Some("setting-snow"),
        "Snow anisotropy" => Some("setting-snow-anisotropy"),
        "Snow intensity" => Some("setting-snow-intensity"),
        "Speed" => Some("setting-speed"),
        "Speed at which the horizontal waving occurs." => {
            Some("setting-speed-at-which-the-horizontal-waving-occurs")
        }
        "Split" => Some("preview-split"),
        "Start mid-line" => Some("setting-start-mid-line"),
        "Start the head-switching artifact mid-scanline, with some static where it begins." => {
            Some("setting-start-the-head-switching-artifact-mid-scanline-with-some-static-ad4bb3ad")
        }
        "Subsample the chrominance planes to half the resolution of the luminance plane. Increases playback compatibility." => {
            Some("ui-subsample-the-chrominance-planes-to-half-the-resolution-of-the-7f07e175")
        }
        "Subsample the chrominance planes to half the resolution of the luminance plane. Results in smaller files." => {
            Some("ui-subsample-the-chrominance-planes-to-half-the-resolution-of-the-b5ec1880")
        }
        "System" => Some("theme-system"),
        "Tape speed" => Some("setting-tape-speed"),
        "The low-pass filter to use throughout the effect." => {
            Some("setting-the-low-pass-filter-to-use-throughout-the-effect")
        }
        "The power of the notch filter / how far out the ringing extends." => {
            Some("setting-the-power-of-the-notch-filter-how-far-out-the-ringing-extends")
        }
        "Theme" => Some("settings-theme"),
        "Third-Party Licenses" => Some("help-third-party-licenses"),
        "Time remaining: " => Some("render-time-remaining-prefix"),
        "To enable interlaced output, set the \"Use field\" setting to \"Interleaved\"." => {
            Some("ui-to-enable-interlaced-output-set-the-use-field-setting-to-interleaved")
        }
        "Total height of the head-switching artifact." => {
            Some("setting-total-height-of-the-head-switching-artifact")
        }
        "Total height of the tracking artifacts." => {
            Some("setting-total-height-of-the-tracking-artifacts")
        }
        "Tracking noise" => Some("setting-tracking-noise"),
        "Treat the video as interlaced, with the lower field as the earlier frame." => {
            Some("setting-treat-the-video-as-interlaced-with-the-lower-field-as-the-earlier-frame")
        }
        "Treat the video as interlaced, with the upper field as the earlier frame." => {
            Some("setting-treat-the-video-as-interlaced-with-the-upper-field-as-the-earlier-frame")
        }
        "Undo" => Some("action-undo"),
        "Unmute" => Some("ui-unmute"),
        "Up to date" => Some("update-up-to-date"),
        "Upper only" => Some("setting-upper-only"),
        "Use all rows; don't skip any." => Some("setting-use-all-rows-don-t-skip-any"),
        "Use dark mode" => Some("theme-dark-tooltip"),
        "Use field" => Some("setting-use-field"),
        "Use light mode" => Some("theme-light-tooltip"),
        "Used by:" => Some("ui-used-by"),
        "VHS emulation" => Some("setting-vhs-emulation"),
        "Vertical offset of the chrominance (color) signal. Usually increases with VHS generation loss." => {
            Some("setting-vertical-offset-of-the-chrominance-color-signal-usually-increas-41c52095")
        }
        "Vertical scale" => Some("setting-vertical-scale"),
        "Vertically blend chroma" => Some("setting-vertically-blend-chroma"),
        "Vertically blend each scanline's chrominance with the scanline above it." => {
            Some("setting-vertically-blend-each-scanline-s-chrominance-with-the-scanline-above-it")
        }
        "Vertically scale the effect by this amount. You should probably leave this at 1.0." => {
            Some("setting-vertically-scale-the-effect-by-this-amount-you-should-probably-9de3dbe9")
        }
        "Video quality factor, where 0 is the worst quality and 50 is the best. Higher quality videos take up more space." => {
            Some("ui-video-quality-factor-where-0-is-the-worst-quality-and-50-is-the-e5bce8c9")
        }
        "View" => Some("menu-view"),
        "Waiting..." => Some("render-waiting"),
        "Wave intensity" => Some("setting-wave-intensity"),
        "You're rendering an image sequence into a directory that isn't empty. This will output many individual image files into that directory." => {
            Some("render-output-directory-not-empty-description")
        }
        "Zoom" => Some("zoom"),
        "Zoom preview" => Some("zoom-preview"),
        "by " => Some("ui-by"),
        "fps" => Some("ui-fps"),
        "lines" => Some("ui-lines"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::Language;

    #[test]
    fn fluent_resources_load_for_all_languages() {
        for language in Language::ALL {
            assert_eq!(language.tr("app-title"), "ntsc-rs");
            assert!(!language.tr("render-rendering").is_empty());
            assert!(language.render_error("boom").contains("boom"));
        }
    }
}
