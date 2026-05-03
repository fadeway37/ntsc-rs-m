<p align="center">
    <a href="https://ntsc.rs">
        <picture>
            <source media="(prefers-color-scheme: dark)" srcset="./docs/img/logo-darkmode.svg">
            <img alt="ntsc-rs logo" src="./docs/img/logo-lightmode.svg" width="1216">
        </picture>
    </a>
</p>

---

**ntsc-rs-m** is a fork of [ntsc-rs](https://github.com/ntsc-rs/ntsc-rs), a video effect which emulates NTSC and VHS video artifacts. It can be used as an After Effects, Premiere, or OpenFX plugin, or as a standalone application.

This fork keeps the original ntsc-rs functionality and adds multilingual UI support. Translations have currently been added for Simplified Chinese, Russian, and Japanese.

![Screenshot of the ntsc-rs standalone application](./docs/img/appdemo.png)

## Download and Install

The latest version of ntsc-rs can be downloaded from [the releases page](https://github.com/valadaptive/ntsc-rs/releases).

After downloading, [read the documentation for how to run it](https://ntsc.rs/docs/standalone-installation/). In particular, ntsc-rs will not work properly on Linux unless you install all of the GStreamer packages listed in the documentation.

## More information

ntsc-rs is a rough Rust port of [ntscqt](https://github.com/JargeZ/ntscqt), a PyQt-based GUI for [ntsc](https://github.com/zhuker/ntsc), itself a Python port of [composite-video-simulator](https://github.com/joncampbell123/composite-video-simulator). Reimplementing the image processing in multithreaded Rust allows it to run at (mostly) real-time speeds.

It's not an exact port--some processing passes have visibly different results, and some new ones have been added.

## Contributing translations

Additional language contributions are welcome. GUI translations are stored as Fluent resources in `i18n/<lang>/ntsc-rs.ftl`.

To add another language:

1. Copy `i18n/en/ntsc-rs.ftl` to `i18n/<lang>/ntsc-rs.ftl` and translate the Fluent values.
2. Register the language in `crates/gui/src/app/i18n.rs` by adding a `Language` variant and a `LANGUAGES` entry with its locale code and display name.
3. Add locale detection/storage-key compatibility if the language has common aliases.
4. Build and run the GUI, switch to the new language from the language menu, and check the main workflows for missing or overflowing text.
5. For CJK or less common scripts, confirm that egui font fallback still displays the language correctly.

Do not add a large translation match to `Language::text`; it exists only as a temporary compatibility layer for older English source-text call sites. New UI code should use stable Fluent ids such as `language.tr("action-open")` or `language.tr_args("render-error", &[("error", error.to_string())])`.

Pull requests should include the language name, locale code, and any notes about incomplete or uncertain translations.
