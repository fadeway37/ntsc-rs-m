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

Additional language contributions are welcome. Translations are currently implemented in `crates/gui/src/app/i18n.rs`, where English UI strings are used as the source text.

To add another language:

1. Add a new variant to the `Language` enum and include it in `Language::ALL`.
2. Add locale detection, a storage key, and a display name for the new language.
3. Add a new match arm in `Language::text` and translate the English source strings there.
4. Translate the language-specific formatting helpers, including update labels, render progress, completion messages, errors, and ETA units.
5. Build and run the GUI, switch to the new language from the language menu, and check the main workflows for missing or overflowing text.

Pull requests should include the language name, locale code, and any notes about incomplete or uncertain translations.
