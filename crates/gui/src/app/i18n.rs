use std::{borrow::Cow, env};

use gstreamer::ClockTime;
use serde::{Deserialize, Serialize};

use crate::app::error::ApplicationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    English,
    SimplifiedChinese,
    Russian,
    Japanese,
}

impl Language {
    pub const ALL: [Self; 4] = [
        Self::English,
        Self::SimplifiedChinese,
        Self::Russian,
        Self::Japanese,
    ];

    pub fn detect() -> Self {
        let locale = env::var("LC_ALL")
            .ok()
            .or_else(|| env::var("LANG").ok())
            .or_else(|| env::var("LANGUAGE").ok())
            .unwrap_or_default()
            .to_lowercase();

        if locale.starts_with("zh") {
            Self::SimplifiedChinese
        } else if locale.starts_with("ru") {
            Self::Russian
        } else if locale.starts_with("ja") {
            Self::Japanese
        } else {
            Self::English
        }
    }

    pub fn storage_key(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::SimplifiedChinese => "zh-CN",
            Self::Russian => "ru",
            Self::Japanese => "ja",
        }
    }

    pub fn from_storage_key(value: &str) -> Option<Self> {
        match value {
            "en" => Some(Self::English),
            "zh" | "zh-CN" | "zh_CN" => Some(Self::SimplifiedChinese),
            "ru" => Some(Self::Russian),
            "ja" => Some(Self::Japanese),
            _ => None,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::SimplifiedChinese => "简体中文",
            Self::Russian => "Русский",
            Self::Japanese => "日本語",
        }
    }

    pub fn text<'a>(self, text: &'a str) -> Cow<'a, str> {
        let translated = match self {
            Self::English => text,
            Self::SimplifiedChinese => match text {
                "About + Credits" => "关于与致谢",
                "An error occurred while loading" => "加载时发生错误",
                "Bit depth" => "位深",
                "Browse for a path" => "浏览路径",
                "by " => "作者：",
                "Cancel" => "取消",
                "Check for Updates" => "检查更新",
                "Check for Updates..." => "检查更新...",
                "Checking for updates..." => "正在检查更新...",
                "Codec" => "编码器",
                "Compression level" => "压缩级别",
                "Copy" => "复制",
                "Copy frame" => "复制帧",
                "Dark" => "深色",
                "Delete" => "删除",
                "Destination file:" => "目标文件：",
                "Disable" => "关闭",
                "Download from GitHub ⤴" => "从 GitHub 下载 ⤴",
                "Drop to install presets" => "拖放以安装预设",
                "Drop to load media" => "拖放以加载媒体",
                "Drop to load preset" => "拖放以加载预设",
                "Duration:" => "时长：",
                "Easy mode" => "简易模式",
                "Edit" => "编辑",
                "Effect" => "效果",
                "Effect preview" => "效果预览",
                "Enable" => "启用",
                "Encoding speed" => "编码速度",
                "Error checking for updates:" => "检查更新时出错：",
                "Error creating pipeline" => "创建管线时出错",
                "Error creating preset" => "创建预设时出错",
                "Error creating presets directory" => "创建预设目录时出错",
                "Error creating render job" => "创建渲染任务时出错",
                "Error deleting preset" => "删除预设时出错",
                "Error during render job" => "渲染任务期间出错",
                "Error initializing GStreamer" => "初始化 GStreamer 时出错",
                "Error installing preset" => "安装预设时出错",
                "Error loading presets directory" => "加载预设目录时出错",
                "Error loading video" => "加载视频时出错",
                "Error parsing JSON" => "解析 JSON 时出错",
                "Error reading JSON" => "读取 JSON 时出错",
                "Error renaming preset" => "重命名预设时出错",
                "File" => "文件",
                "Filesystem error" => "文件系统错误",
                "Fit" => "适应",
                "Follow system color theme" => "跟随系统配色主题",
                "fps" => "帧/秒",
                "Help" => "帮助",
                "Image sequences do not support interlaced output." => "图像序列不支持隔行输出。",
                "Interlaced output" => "隔行输出",
                "Language" => "语言",
                "License" => "许可证",
                "Light" => "轻度",
                "Light theme" => "浅色",
                "lines" => "行",
                "Load" => "加载",
                "Load from..." => "从文件加载...",
                "Mute" => "静音",
                "No media loaded" => "未加载媒体",
                "OK" => "确定",
                "Online Documentation ⤴" => "在线文档 ⤴",
                "Only one file at a time can be dropped here" => "这里一次只能拖放一个文件",
                "Open" => "打开",
                "Open containing folder" => "打开所在文件夹",
                "Open folder" => "打开文件夹",
                "Output directory is not empty" => "输出目录非空",
                "Overwrite" => "覆盖",
                "Paste" => "粘贴",
                "Paste JSON" => "粘贴 JSON",
                "Paused" => "已暂停",
                "Presets" => "预设",
                "Quality" => "质量",
                "Quit" => "退出",
                "Random seed" => "随机种子",
                "Randomize seed" => "随机生成种子",
                "Redo" => "重做",
                "Reload" => "重新加载",
                "Render" => "渲染",
                "Rendering..." => "正在渲染...",
                "Rename" => "重命名",
                "Reset" => "重置",
                "Resizing filter" => "缩放滤波器",
                "Save" => "保存",
                "Save as" => "另存为",
                "Save frame" => "保存当前帧",
                "Save to..." => "保存到...",
                "Scale to" => "缩放到",
                "Split" => "分屏",
                "System" => "系统",
                "Theme" => "主题",
                "Third-Party Licenses" => "第三方许可证",
                "Time remaining: " => "剩余时间：",
                "Undo" => "撤销",
                "Unmute" => "取消静音",
                "Up to date" => "已是最新版本",
                "Use dark mode" => "使用深色模式",
                "Use field" => "场使用方式",
                "Use light mode" => "使用浅色模式",
                "Used by:" => "使用于：",
                "View" => "视图",
                "Waiting..." => "等待中...",
                "Zoom" => "缩放",
                "Zoom preview" => "缩放预览",
                "...loosely based on " => "...大致基于 ",
                "...which is a GUI for " => "...而它是这个项目的图形界面：",
                "...which is a port of " => "...而它移植自 ",
                "0 degrees" => "0 度",
                "90 degrees" => "90 度",
                "180 degrees" => "180 度",
                "270 degrees" => "270 度",
                "1-line comb" => "1 行梳状滤波",
                "2-line comb" => "2 行梳状滤波",
                "Additional ringing artifacts, simulated with a notch filter." => {
                    "额外的振铃伪影，使用陷波滤波器模拟。"
                }
                "Alternating" => "交替",
                "Amount of sharpening to apply." => "应用的锐化量。",
                "Apply a low-pass filter to the input chrominance (color) signal." => {
                    "对输入色度（颜色）信号应用低通滤波器。"
                }
                "Apply a low-pass filter to the output chroma signal." => {
                    "对输出色度信号应用低通滤波器。"
                }
                "Apply a notch filter to the input luminance signal. Sharp, but has ringing artifacts." => {
                    "对输入亮度信号应用陷波滤波器。画面较锐利，但会产生振铃伪影。"
                }
                "Apply a simple box filter to the input luminance signal." => {
                    "对输入亮度信号应用简单方框滤波器。"
                }
                "Average the current row with the previous and next ones, phase-cancelling the chrominance (color) signals. Only works if the scanline phase shift is 180 degrees." => {
                    "将当前行与上一行和下一行平均，以相位抵消色度（颜色）信号。仅在扫描线相位偏移为 180 度时有效。"
                }
                "Average the current row with the previous and next ones, phase-cancelling the chrominance signals. Only works if the scanline phase shift is 180 degrees." => {
                    "将当前行与上一行和下一行平均，以相位抵消色度信号。仅在扫描线相位偏移为 180 度时有效。"
                }
                "Average the current row with the previous one, phase-cancelling the chrominance (color) signals. Only works if the scanline phase shift is 180 degrees." => {
                    "将当前行与上一行平均，以相位抵消色度（颜色）信号。仅在扫描线相位偏移为 180 度时有效。"
                }
                "Average the current row with the previous one, phase-cancelling the chrominance signals. Only works if the scanline phase shift is 180 degrees." => {
                    "将当前行与上一行平均，以相位抵消色度信号。仅在扫描线相位偏移为 180 度时有效。"
                }
                "Base wavelength for the horizontal waving." => "水平波动的基础波长。",
                "Base wavelength, in pixels, of the noise." => "噪声的基础波长，以像素为单位。",
                "Boost high frequencies in the NTSC signal, sharpening the image and intensifying colors." => {
                    "增强 NTSC 信号中的高频部分，使图像更锐利并增强颜色。"
                }
                "Both" => "全部",
                "Box" => "方框",
                "Butterworth (sharper)" => "巴特沃斯（更锐利）",
                "Chance that the chrominance (color) signal is completely lost in each scanline." => {
                    "每条扫描线中色度（颜色）信号完全丢失的概率。"
                }
                "Chance that the chrominance signal is completely lost in each scanline." => {
                    "每条扫描线中色度信号完全丢失的概率。"
                }
                "Choose which rows (\"fields\" in NTSC parlance) of the source image will be used." => {
                    "选择使用源图像中的哪些行（NTSC 术语中称为“场”）。"
                }
                "Chroma delay (horizontal)" => "色度延迟（水平）",
                "Chroma delay (vertical)" => "色度延迟（垂直）",
                "Chroma demodulation filter" => "色度解调滤波器",
                "Chroma loss" => "色度丢失",
                "Chroma low-pass in" => "输入色度低通",
                "Chroma low-pass out" => "输出色度低通",
                "Chroma noise" => "色度噪声",
                "Chroma phase error" => "色度相位误差",
                "Chroma phase noise" => "色度相位噪声",
                "Composite signal noise" => "复合信号噪声",
                "Composite signal sharpening" => "复合信号锐化",
                "Constant K (blurry)" => "常数 K（较模糊）",
                "Detail" => "细节",
                "Determines whether the speckles are placed truly randomly or concentrated in certain rows." => {
                    "决定雪花点是真正随机分布，还是集中在某些行中。"
                }
                "Do not filter the luminance signal. Adds rainbow artifacts." => {
                    "不滤波亮度信号。会增加彩虹伪影。"
                }
                "Edge wave" => "边缘波动",
                "Emulate cutoff of high-frequency data at various VHS recording speeds." => {
                    "模拟 VHS 在不同录制速度下对高频数据的截止。"
                }
                "Emulate noise from VHS tracking error." => "模拟 VHS 跟踪误差产生的噪声。",
                "Emulate VHS head-switching artifacts at the bottom of the image." => {
                    "模拟图像底部的 VHS 磁头切换伪影。"
                }
                "EP (Extended Play)" => "EP（超长播放）",
                "FFV1 (Lossless)" => "FFV1（无损）",
                "Filter the input luminance to decrease rainbow artifacts." => {
                    "过滤输入亮度以减少彩虹伪影。"
                }
                "Filter used to modulate the chrominance (color) data out of the composite NTSC signal." => {
                    "用于从复合 NTSC 信号中调制出色度（颜色）数据的滤波器。"
                }
                "Filter with a sharper falloff. Produces sharpened, less blurry results." => {
                    "衰减更锐利的滤波器。结果更锐利，也更少模糊。"
                }
                "Frequency" => "频率",
                "Frequency / radius of the sharpening, relative to the tape speed's cutoff frequency." => {
                    "锐化的频率 / 半径，相对于磁带速度的截止频率。"
                }
                "Frequency of random speckles in the image." => "图像中随机雪花点的频率。",
                "Frequency of speckle-type noise in the artifacts." => {
                    "伪影中雪花型噪声的频率。"
                }
                "Frequency/period of the ringing, in \"rings per pixel\"." => {
                    "振铃的频率 / 周期，以“每像素振铃数”表示。"
                }
                "Full" => "完整",
                "Full-intensity low-pass filter." => "全强度低通滤波器。",
                "H.264" => "H.264",
                "Head switching" => "磁头切换",
                "Height" => "高度",
                "Horizontal offset of the chrominance (color) signal." => {
                    "色度（颜色）信号的水平偏移。"
                }
                "Horizontal position at which the head-switching starts." => {
                    "磁头切换开始的水平位置。"
                }
                "Horizontal scale" => "水平缩放",
                "Horizontal shift" => "水平位移",
                "Horizontal waving of the image." => "图像的水平波动。",
                "Horizontal waving of the image, in pixels." => {
                    "图像的水平波动，以像素为单位。"
                }
                "Horizontally scale the effect by this amount. For 480p video, leave this at 1.0 for the most physically-accurate result." => {
                    "按此数值水平缩放效果。对于 480p 视频，保持 1.0 可得到最符合物理表现的结果。"
                }
                "How much of the head-switching artifact is off-screen." => {
                    "磁头切换伪影位于画面外的程度。"
                }
                "How much the affected scanlines \"wave\" back and forth." => {
                    "受影响扫描线来回“波动”的程度。"
                }
                "How much the head-switching artifact \"jitters\" horizontally." => {
                    "磁头切换伪影水平“抖动”的程度。"
                }
                "How much the head-switching artifact shifts rows horizontally." => {
                    "磁头切换伪影让各行水平位移的程度。"
                }
                "How much the speckles are clustered by scanline." => {
                    "雪花点沿扫描线聚集的程度。"
                }
                "Input luma filter" => "输入亮度滤波器",
                "Intensity" => "强度",
                "Intensity of non-speckle noise." => "非雪花型噪声的强度。",
                "Intensity of the noise." => "噪声强度。",
                "Intensity of the ringing." => "振铃强度。",
                "Interleaved (lower first)" => "交错（下场优先）",
                "Interleaved (upper first)" => "交错（上场优先）",
                "Jitter" => "抖动",
                "Less intense low-pass filter." => "强度较低的低通滤波器。",
                "Lower only" => "仅下场",
                "Lowpass filter type" => "低通滤波器类型",
                "LP (Long Play)" => "LP（长时间播放）",
                "Luma noise" => "亮度噪声",
                "Luma smear" => "亮度拖影",
                "Multiply the scaling factors by the video's height. Prefer scaling the input video to 480p instead, which gives much more accurate-looking results." => {
                    "将缩放系数乘以视频高度。更建议先把输入视频缩放到 480p，这样结果会准确得多。"
                }
                "No low-pass filter." => "无低通滤波器。",
                "Noise applied per-scanline to the phase of the chrominance (color) signal." => {
                    "逐扫描线应用到色度（颜色）信号相位上的噪声。"
                }
                "Noise applied per-scanline to the phase of the chrominance signal." => {
                    "逐扫描线应用到色度信号相位上的噪声。"
                }
                "Noise applied to the chrominance (color) signal." => {
                    "应用于色度（颜色）信号的噪声。"
                }
                "Noise applied to the chrominance signal." => "应用于色度信号的噪声。",
                "Noise applied to the composite NTSC signal." => {
                    "应用于复合 NTSC 信号的噪声。"
                }
                "Noise applied to the luminance signal. Useful for higher-frequency noise than the \"Composite noise\" setting can provide." => {
                    "应用于亮度信号的噪声。适合产生比“复合噪声”设置更高频的噪声。"
                }
                "Noise intensity" => "噪声强度",
                "None" => "无",
                "Notch" => "陷波",
                "Notch filter. Sharper than a box blur, but with ringing artifacts." => {
                    "陷波滤波器。比方框模糊更锐利，但会产生振铃伪影。"
                }
                "Octaves of noise for the waves." => "波动噪声的倍频层数。",
                "Octaves of noise." => "噪声倍频层数。",
                "Offset" => "偏移",
                "Phase error for the chrominance (color) signal." => {
                    "色度（颜色）信号的相位误差。"
                }
                "Phase shift of the chrominance (color) signal each scanline. Usually 180 degrees." => {
                    "每条扫描线中色度（颜色）信号的相位偏移，通常为 180 度。"
                }
                "PNG Sequence" => "PNG 序列",
                "Position" => "位置",
                "Power" => "功率",
                "Ringing" => "振铃",
                "Saturation" => "饱和度",
                "Scale" => "缩放",
                "Scale the effect by these factors." => "按这些系数缩放效果。",
                "Scale with video size" => "随视频尺寸缩放",
                "Scanline phase shift" => "扫描线相位偏移",
                "Scanline phase shift offset" => "扫描线相位偏移量",
                "Sharpen" => "锐化",
                "Sharpening of the image, as done by some VHS decks." => {
                    "模拟某些 VHS 录像机对图像进行的锐化。"
                }
                "Simple constant-k filter. Produces longer, blurry results." => {
                    "简单的常数 K 滤波器。会产生更长、更模糊的结果。"
                }
                "Simple horizontal box blur." => "简单的水平方框模糊。",
                "Skip every lower row, keeping the upper ones." => {
                    "跳过每条下场行，保留上场行。"
                }
                "Skip every other row, alternating between skipping even and odd rows." => {
                    "每隔一行跳过一次，并在跳过偶数行和奇数行之间交替。"
                }
                "Skip every upper row, keeping the lower ones." => {
                    "跳过每条上场行，保留下场行。"
                }
                "Snow" => "雪花",
                "Snow anisotropy" => "雪花各向异性",
                "Snow intensity" => "雪花强度",
                "SP (Standard Play)" => "SP（标准播放）",
                "Speed" => "速度",
                "Speed at which the horizontal waving occurs." => "水平波动发生的速度。",
                "Start mid-line" => "从行中间开始",
                "Start the head-switching artifact mid-scanline, with some static where it begins." => {
                    "让磁头切换伪影从扫描线中间开始，并在起点产生一些静噪。"
                }
                "Tape speed" => "磁带速度",
                "The low-pass filter to use throughout the effect." => {
                    "整个效果使用的低通滤波器。"
                }
                "The power of the notch filter / how far out the ringing extends." => {
                    "陷波滤波器的功率 / 振铃向外延伸的距离。"
                }
                "Total height of the head-switching artifact." => {
                    "磁头切换伪影的总高度。"
                }
                "Total height of the tracking artifacts." => "跟踪伪影的总高度。",
                "Tracking noise" => "跟踪噪声",
                "Treat the video as interlaced, with the lower field as the earlier frame." => {
                    "将视频视为隔行，并以下场作为较早的帧。"
                }
                "Treat the video as interlaced, with the upper field as the earlier frame." => {
                    "将视频视为隔行，并以上场作为较早的帧。"
                }
                "Upper only" => "仅上场",
                "Use all rows; don't skip any." => "使用所有行，不跳过任何一行。",
                "Vertical offset of the chrominance (color) signal. Usually increases with VHS generation loss." => {
                    "色度（颜色）信号的垂直偏移，通常会随 VHS 代际损失增加。"
                }
                "Vertical scale" => "垂直缩放",
                "Vertically blend chroma" => "垂直混合彩色",
                "Vertically blend each scanline's chrominance with the scanline above it." => {
                    "将每条扫描线的色度与其上一条扫描线垂直混合。"
                }
                "Vertically scale the effect by this amount. You should probably leave this at 1.0." => {
                    "按此数值垂直缩放效果。通常应保持为 1.0。"
                }
                "VHS emulation" => "VHS 模拟",
                "Wave intensity" => "波动强度",
                "4:2:0 chroma subsampling" => "4:2:0 色度抽样",
                "8-bit" => "8 位",
                "10-bit" => "10 位",
                "12-bit" => "12 位",
                "Nearest" => "最近邻",
                "Bilinear" => "双线性",
                "Bicubic" => "双三次",
                "Lower-quality but faster scaling filter" => "质量较低但速度更快的缩放滤波器",
                "Sharper scaling filter; ~20% slower than Bilinear" => {
                    "更锐利的缩放滤波器；比双线性慢约 20%。"
                }
                "Nearest-neighbor (pixelated) scaling. Note that this is still slower than Bilinear" => {
                    "最近邻（像素化）缩放。注意它仍然比双线性更慢。"
                }
                "Scale the video prior to applying the effect. Real NTSC footage is 480 lines tall. This applies to both the preview and the final render, and is not saved as part of presets." => {
                    "在应用效果前先缩放视频。真实 NTSC 画面高度为 480 行。此设置同时作用于预览和最终渲染，且不会保存到预设中。"
                }
                "Compression level for PNG encoding. Higher compression levels produce smaller files but take longer to render." => {
                    "PNG 编码的压缩级别。压缩级别越高，文件越小，但渲染时间越长。"
                }
                "Video quality factor, where 0 is the worst quality and 50 is the best. Higher quality videos take up more space." => {
                    "视频质量系数，0 为最差，50 为最佳。质量越高，占用空间越大。"
                }
                "Encoding speed preset. Higher encoding speeds provide a worse compression ratio, resulting in larger videos at a given quality." => {
                    "编码速度预设。编码速度越高，压缩比越差，因此在相同质量下视频文件会更大。"
                }
                "Subsample the chrominance planes to half the resolution of the luminance plane. Increases playback compatibility." => {
                    "将色度平面降采样到亮度平面的一半分辨率，以提高播放兼容性。"
                }
                "Subsample the chrominance planes to half the resolution of the luminance plane. Results in smaller files." => {
                    "将色度平面降采样到亮度平面的一半分辨率，以减小文件体积。"
                }
                "To enable interlaced output, set the \"Use field\" setting to \"Interleaved\"." => {
                    "要启用隔行输出，请将“场使用方式”设置为“交错”。"
                }
                "You're rendering an image sequence into a directory that isn't empty. This will output many individual image files into that directory." => {
                    "你正在将图像序列渲染到一个非空目录中。这会向该目录输出大量单独的图像文件。"
                }
                " (interlaced)" => "（隔行）",
                " (progressive)" => "（逐行）",
                " (telecined)" => "（3:2 拉伸）",
                _ => text,
            },
            Self::Russian => match text {
                "About + Credits" => "О программе и благодарности",
                "An error occurred while loading" => "Во время загрузки произошла ошибка",
                "Bit depth" => "Глубина цвета",
                "Browse for a path" => "Выбрать путь",
                "by " => "автор: ",
                "Cancel" => "Отмена",
                "Check for Updates" => "Проверить обновления",
                "Check for Updates..." => "Проверить обновления...",
                "Checking for updates..." => "Проверка обновлений...",
                "Codec" => "Кодек",
                "Compression level" => "Уровень сжатия",
                "Copy" => "Копировать",
                "Copy frame" => "Копировать кадр",
                "Dark" => "Тёмная",
                "Delete" => "Удалить",
                "Destination file:" => "Файл назначения:",
                "Disable" => "Выключить",
                "Download from GitHub ⤴" => "Скачать с GitHub ⤴",
                "Drop to install presets" => "Перетащите для установки пресетов",
                "Drop to load media" => "Перетащите для загрузки медиа",
                "Drop to load preset" => "Перетащите для загрузки пресета",
                "Duration:" => "Длительность:",
                "Easy mode" => "Простой режим",
                "Edit" => "Правка",
                "Effect" => "Эффект",
                "Effect preview" => "Предпросмотр эффекта",
                "Enable" => "Включить",
                "Encoding speed" => "Скорость кодирования",
                "Error checking for updates:" => "Ошибка при проверке обновлений:",
                "Error creating pipeline" => "Ошибка создания конвейера",
                "Error creating preset" => "Ошибка создания пресета",
                "Error creating presets directory" => "Ошибка создания папки пресетов",
                "Error creating render job" => "Ошибка создания задачи рендера",
                "Error deleting preset" => "Ошибка удаления пресета",
                "Error during render job" => "Ошибка во время рендера",
                "Error initializing GStreamer" => "Ошибка инициализации GStreamer",
                "Error installing preset" => "Ошибка установки пресета",
                "Error loading presets directory" => "Ошибка загрузки папки пресетов",
                "Error loading video" => "Ошибка загрузки видео",
                "Error parsing JSON" => "Ошибка разбора JSON",
                "Error reading JSON" => "Ошибка чтения JSON",
                "Error renaming preset" => "Ошибка переименования пресета",
                "File" => "Файл",
                "Filesystem error" => "Ошибка файловой системы",
                "Fit" => "По размеру",
                "Follow system color theme" => "Следовать системной теме",
                "fps" => "кадр/с",
                "Help" => "Справка",
                "Image sequences do not support interlaced output." => {
                    "Последовательности изображений не поддерживают чересстрочный вывод."
                }
                "Interlaced output" => "Чересстрочный вывод",
                "Language" => "Язык",
                "License" => "Лицензия",
                "Light" => "Лёгкий",
                "Light theme" => "Светлая",
                "lines" => "строк",
                "Load" => "Загрузить",
                "Load from..." => "Загрузить из...",
                "Mute" => "Выключить звук",
                "No media loaded" => "Медиа не загружено",
                "OK" => "ОК",
                "Online Documentation ⤴" => "Онлайн-документация ⤴",
                "Only one file at a time can be dropped here" => "Сюда можно перетаскивать только один файл за раз",
                "Open" => "Открыть",
                "Open containing folder" => "Открыть папку с файлом",
                "Open folder" => "Открыть папку",
                "Output directory is not empty" => "Папка вывода не пуста",
                "Overwrite" => "Перезаписать",
                "Paste" => "Вставить",
                "Paste JSON" => "Вставить JSON",
                "Paused" => "На паузе",
                "Presets" => "Пресеты",
                "Quality" => "Качество",
                "Quit" => "Выход",
                "Random seed" => "Случайное зерно",
                "Randomize seed" => "Сгенерировать случайное зерно",
                "Redo" => "Повторить",
                "Reload" => "Обновить",
                "Render" => "Рендер",
                "Rendering..." => "Рендеринг...",
                "Rename" => "Переименовать",
                "Reset" => "Сбросить",
                "Resizing filter" => "Фильтр масштабирования",
                "Save" => "Сохранить",
                "Save as" => "Сохранить как",
                "Save frame" => "Сохранить кадр",
                "Save to..." => "Сохранить в...",
                "Scale to" => "Масштаб до",
                "Split" => "Разделить",
                "System" => "Система",
                "Theme" => "Тема",
                "Third-Party Licenses" => "Сторонние лицензии",
                "Time remaining: " => "Осталось времени: ",
                "Undo" => "Отменить",
                "Unmute" => "Включить звук",
                "Up to date" => "Установлена последняя версия",
                "Use dark mode" => "Использовать тёмную тему",
                "Use field" => "Использование поля",
                "Use light mode" => "Использовать светлую тему",
                "Used by:" => "Используется в:",
                "View" => "Вид",
                "Waiting..." => "Ожидание...",
                "Zoom" => "Масштаб",
                "Zoom preview" => "Масштаб предпросмотра",
                "...loosely based on " => "...примерно основано на ",
                "...which is a GUI for " => "...а это графический интерфейс для ",
                "...which is a port of " => "...а это порт ",
                "0 degrees" => "0 градусов",
                "90 degrees" => "90 градусов",
                "180 degrees" => "180 градусов",
                "270 degrees" => "270 градусов",
                "1-line comb" => "1-строчный гребенчатый фильтр",
                "2-line comb" => "2-строчный гребенчатый фильтр",
                "Additional ringing artifacts, simulated with a notch filter." => {
                    "Дополнительные артефакты звона, имитируемые режекторным фильтром."
                }
                "Alternating" => "Чередование",
                "Amount of sharpening to apply." => "Степень применяемой резкости.",
                "Apply a low-pass filter to the input chrominance (color) signal." => {
                    "Применять ФНЧ к входному сигналу цветности."
                }
                "Apply a low-pass filter to the output chroma signal." => {
                    "Применять ФНЧ к выходному сигналу цветности."
                }
                "Apply a notch filter to the input luminance signal. Sharp, but has ringing artifacts." => {
                    "Применять режекторный фильтр к входному сигналу яркости. Резко, но с артефактами звона."
                }
                "Apply a simple box filter to the input luminance signal." => {
                    "Применять простой прямоугольный фильтр к входному сигналу яркости."
                }
                "Average the current row with the previous and next ones, phase-cancelling the chrominance (color) signals. Only works if the scanline phase shift is 180 degrees." => {
                    "Усреднять текущую строку с предыдущей и следующей, взаимно подавляя сигналы цветности по фазе. Работает только при фазовом сдвиге строк 180 градусов."
                }
                "Average the current row with the previous and next ones, phase-cancelling the chrominance signals. Only works if the scanline phase shift is 180 degrees." => {
                    "Усреднять текущую строку с предыдущей и следующей, взаимно подавляя сигналы цветности по фазе. Работает только при фазовом сдвиге строк 180 градусов."
                }
                "Average the current row with the previous one, phase-cancelling the chrominance (color) signals. Only works if the scanline phase shift is 180 degrees." => {
                    "Усреднять текущую строку с предыдущей, взаимно подавляя сигналы цветности по фазе. Работает только при фазовом сдвиге строк 180 градусов."
                }
                "Average the current row with the previous one, phase-cancelling the chrominance signals. Only works if the scanline phase shift is 180 degrees." => {
                    "Усреднять текущую строку с предыдущей, взаимно подавляя сигналы цветности по фазе. Работает только при фазовом сдвиге строк 180 градусов."
                }
                "Base wavelength for the horizontal waving." => {
                    "Базовая длина волны для горизонтального колебания."
                }
                "Base wavelength, in pixels, of the noise." => {
                    "Базовая длина волны шума в пикселях."
                }
                "Boost high frequencies in the NTSC signal, sharpening the image and intensifying colors." => {
                    "Усилить высокие частоты в NTSC-сигнале, повышая резкость изображения и насыщенность цветов."
                }
                "Both" => "Оба",
                "Box" => "Прямоугольный",
                "Butterworth (sharper)" => "Баттерворт (резче)",
                "Chance that the chrominance (color) signal is completely lost in each scanline." => {
                    "Вероятность полной потери сигнала цветности в каждой строке развертки."
                }
                "Chance that the chrominance signal is completely lost in each scanline." => {
                    "Вероятность полной потери сигнала цветности в каждой строке развертки."
                }
                "Choose which rows (\"fields\" in NTSC parlance) of the source image will be used." => {
                    "Выбрать, какие строки исходного изображения будут использоваться (в терминологии NTSC это «поля»)."
                }
                "Chroma delay (horizontal)" => "Задержка цветности (по горизонтали)",
                "Chroma delay (vertical)" => "Задержка цветности (по вертикали)",
                "Chroma demodulation filter" => "Фильтр демодуляции цветности",
                "Chroma loss" => "Потеря цветности",
                "Chroma low-pass in" => "Входной ФНЧ цветности",
                "Chroma low-pass out" => "Выходной ФНЧ цветности",
                "Chroma noise" => "Шум цветности",
                "Chroma phase error" => "Ошибка фазы цветности",
                "Chroma phase noise" => "Фазовый шум цветности",
                "Composite signal noise" => "Шум композитного сигнала",
                "Composite signal sharpening" => "Повышение резкости композитного сигнала",
                "Constant K (blurry)" => "Постоянная K (размыто)",
                "Detail" => "Детализация",
                "Determines whether the speckles are placed truly randomly or concentrated in certain rows." => {
                    "Определяет, будут ли точки размещаться действительно случайно или концентрироваться в отдельных строках."
                }
                "Do not filter the luminance signal. Adds rainbow artifacts." => {
                    "Не фильтровать сигнал яркости. Добавляет радужные артефакты."
                }
                "Edge wave" => "Волна по краям",
                "Emulate cutoff of high-frequency data at various VHS recording speeds." => {
                    "Имитировать отсечение высокочастотных данных при разных скоростях записи VHS."
                }
                "Emulate noise from VHS tracking error." => {
                    "Имитировать шум из-за ошибки трекинга VHS."
                }
                "Emulate VHS head-switching artifacts at the bottom of the image." => {
                    "Имитировать артефакты переключения головки VHS внизу изображения."
                }
                "EP (Extended Play)" => "EP (увеличенное время)",
                "FFV1 (Lossless)" => "FFV1 (без потерь)",
                "Filter the input luminance to decrease rainbow artifacts." => {
                    "Фильтровать входную яркость, чтобы уменьшить радужные артефакты."
                }
                "Filter used to modulate the chrominance (color) data out of the composite NTSC signal." => {
                    "Фильтр, используемый для выделения данных цветности из композитного NTSC-сигнала."
                }
                "Filter with a sharper falloff. Produces sharpened, less blurry results." => {
                    "Фильтр с более резким спадом. Даёт более резкий и менее размытый результат."
                }
                "Frequency" => "Частота",
                "Frequency / radius of the sharpening, relative to the tape speed's cutoff frequency." => {
                    "Частота / радиус повышения резкости относительно частоты отсечения скорости ленты."
                }
                "Frequency of random speckles in the image." => {
                    "Частота случайных точек на изображении."
                }
                "Frequency of speckle-type noise in the artifacts." => {
                    "Частота точечного шума в артефактах."
                }
                "Frequency/period of the ringing, in \"rings per pixel\"." => {
                    "Частота / период звона в «кольцах на пиксель»."
                }
                "Full" => "Полный",
                "Full-intensity low-pass filter." => "ФНЧ полной интенсивности.",
                "H.264" => "H.264",
                "Head switching" => "Переключение головки",
                "Height" => "Высота",
                "Horizontal offset of the chrominance (color) signal." => {
                    "Горизонтальное смещение сигнала цветности."
                }
                "Horizontal position at which the head-switching starts." => {
                    "Горизонтальная позиция, с которой начинается переключение головки."
                }
                "Horizontal scale" => "Горизонтальный масштаб",
                "Horizontal shift" => "Горизонтальный сдвиг",
                "Horizontal waving of the image." => "Горизонтальное колебание изображения.",
                "Horizontal waving of the image, in pixels." => {
                    "Горизонтальное колебание изображения в пикселях."
                }
                "Horizontally scale the effect by this amount. For 480p video, leave this at 1.0 for the most physically-accurate result." => {
                    "Масштабировать эффект по горизонтали на эту величину. Для видео 480p оставьте 1.0, чтобы получить наиболее физически точный результат."
                }
                "How much of the head-switching artifact is off-screen." => {
                    "Какая часть артефакта переключения головки находится за пределами кадра."
                }
                "How much the affected scanlines \"wave\" back and forth." => {
                    "Насколько затронутые строки развертки «волнуются» туда-сюда."
                }
                "How much the head-switching artifact \"jitters\" horizontally." => {
                    "Насколько артефакт переключения головки горизонтально «дрожит»."
                }
                "How much the head-switching artifact shifts rows horizontally." => {
                    "Насколько артефакт переключения головки сдвигает строки по горизонтали."
                }
                "How much the speckles are clustered by scanline." => {
                    "Насколько точки сгруппированы по строкам развертки."
                }
                "Input luma filter" => "Входной фильтр яркости",
                "Intensity" => "Интенсивность",
                "Intensity of non-speckle noise." => "Интенсивность шума без точек.",
                "Intensity of the noise." => "Интенсивность шума.",
                "Intensity of the ringing." => "Интенсивность звона.",
                "Interleaved (lower first)" => "Чередование (сначала нижнее поле)",
                "Interleaved (upper first)" => "Чередование (сначала верхнее поле)",
                "Jitter" => "Дрожание",
                "Less intense low-pass filter." => "Менее интенсивный ФНЧ.",
                "Lower only" => "Только нижнее поле",
                "Lowpass filter type" => "Тип ФНЧ",
                "LP (Long Play)" => "LP (долгое воспроизведение)",
                "Luma noise" => "Шум яркости",
                "Luma smear" => "Смаз яркости",
                "Multiply the scaling factors by the video's height. Prefer scaling the input video to 480p instead, which gives much more accurate-looking results." => {
                    "Умножить коэффициенты масштабирования на высоту видео. Лучше масштабировать входное видео до 480p: так результат будет выглядеть гораздо точнее."
                }
                "No low-pass filter." => "Без ФНЧ.",
                "Noise applied per-scanline to the phase of the chrominance (color) signal." => {
                    "Шум, применяемый к фазе сигнала цветности отдельно для каждой строки развертки."
                }
                "Noise applied per-scanline to the phase of the chrominance signal." => {
                    "Шум, применяемый к фазе сигнала цветности отдельно для каждой строки развертки."
                }
                "Noise applied to the chrominance (color) signal." => {
                    "Шум, применяемый к сигналу цветности."
                }
                "Noise applied to the chrominance signal." => {
                    "Шум, применяемый к сигналу цветности."
                }
                "Noise applied to the composite NTSC signal." => {
                    "Шум, применяемый к композитному NTSC-сигналу."
                }
                "Noise applied to the luminance signal. Useful for higher-frequency noise than the \"Composite noise\" setting can provide." => {
                    "Шум, применяемый к сигналу яркости. Полезен для более высокочастотного шума, чем может дать параметр «Композитный шум»."
                }
                "Noise intensity" => "Интенсивность шума",
                "None" => "Нет",
                "Notch" => "Режекторный",
                "Notch filter. Sharper than a box blur, but with ringing artifacts." => {
                    "Режекторный фильтр. Резче прямоугольного размытия, но даёт артефакты звона."
                }
                "Octaves of noise for the waves." => "Октавы шума для волн.",
                "Octaves of noise." => "Октавы шума.",
                "Offset" => "Смещение",
                "Phase error for the chrominance (color) signal." => {
                    "Ошибка фазы сигнала цветности."
                }
                "Phase shift of the chrominance (color) signal each scanline. Usually 180 degrees." => {
                    "Фазовый сдвиг сигнала цветности на каждой строке развертки. Обычно 180 градусов."
                }
                "PNG Sequence" => "Последовательность PNG",
                "Position" => "Позиция",
                "Power" => "Мощность",
                "Ringing" => "Звон",
                "Saturation" => "Насыщенность",
                "Scale" => "Масштаб",
                "Scale the effect by these factors." => {
                    "Масштабировать эффект по этим коэффициентам."
                }
                "Scale with video size" => "Масштабировать по размеру видео",
                "Scanline phase shift" => "Сдвиг фазы строк",
                "Scanline phase shift offset" => "Смещение сдвига фазы строк",
                "Sharpen" => "Резкость",
                "Sharpening of the image, as done by some VHS decks." => {
                    "Повышение резкости изображения, как в некоторых VHS-деках."
                }
                "Simple constant-k filter. Produces longer, blurry results." => {
                    "Простой фильтр с постоянной K. Даёт более длинный и размытый результат."
                }
                "Simple horizontal box blur." => "Простое горизонтальное прямоугольное размытие.",
                "Skip every lower row, keeping the upper ones." => {
                    "Пропускать нижние строки, оставляя верхние."
                }
                "Skip every other row, alternating between skipping even and odd rows." => {
                    "Пропускать каждую вторую строку, чередуя пропуск чётных и нечётных строк."
                }
                "Skip every upper row, keeping the lower ones." => {
                    "Пропускать верхние строки, оставляя нижние."
                }
                "Snow" => "Снег",
                "Snow anisotropy" => "Анизотропия снега",
                "Snow intensity" => "Интенсивность снега",
                "SP (Standard Play)" => "SP (стандартное воспроизведение)",
                "Speed" => "Скорость",
                "Speed at which the horizontal waving occurs." => {
                    "Скорость возникновения горизонтального колебания."
                }
                "Start mid-line" => "Начинать с середины строки",
                "Start the head-switching artifact mid-scanline, with some static where it begins." => {
                    "Начинать артефакт переключения головки в середине строки развертки, добавляя статический шум в месте начала."
                }
                "Tape speed" => "Скорость ленты",
                "The low-pass filter to use throughout the effect." => {
                    "ФНЧ, используемый во всём эффекте."
                }
                "The power of the notch filter / how far out the ringing extends." => {
                    "Мощность режекторного фильтра / насколько далеко распространяется звон."
                }
                "Total height of the head-switching artifact." => {
                    "Общая высота артефакта переключения головки."
                }
                "Total height of the tracking artifacts." => {
                    "Общая высота артефактов трекинга."
                }
                "Tracking noise" => "Шум трекинга",
                "Treat the video as interlaced, with the lower field as the earlier frame." => {
                    "Считать видео чересстрочным, где нижнее поле является более ранним кадром."
                }
                "Treat the video as interlaced, with the upper field as the earlier frame." => {
                    "Считать видео чересстрочным, где верхнее поле является более ранним кадром."
                }
                "Upper only" => "Только верхнее поле",
                "Use all rows; don't skip any." => "Использовать все строки, ничего не пропускать.",
                "Vertical offset of the chrominance (color) signal. Usually increases with VHS generation loss." => {
                    "Вертикальное смещение сигнала цветности. Обычно увеличивается при потере поколений VHS."
                }
                "Vertical scale" => "Вертикальный масштаб",
                "Vertically blend chroma" => "Смешивать цветность по вертикали",
                "Vertically blend each scanline's chrominance with the scanline above it." => {
                    "Смешивать цветность каждой строки развертки со строкой выше."
                }
                "Vertically scale the effect by this amount. You should probably leave this at 1.0." => {
                    "Масштабировать эффект по вертикали на эту величину. Скорее всего, стоит оставить 1.0."
                }
                "VHS emulation" => "Эмуляция VHS",
                "Wave intensity" => "Интенсивность волны",
                "4:2:0 chroma subsampling" => "Цветовая субдискретизация 4:2:0",
                "8-bit" => "8 бит",
                "10-bit" => "10 бит",
                "12-bit" => "12 бит",
                "Nearest" => "Ближайший",
                "Bilinear" => "Билинейный",
                "Bicubic" => "Бикубический",
                "Lower-quality but faster scaling filter" => "Менее качественный, но более быстрый фильтр масштабирования",
                "Sharper scaling filter; ~20% slower than Bilinear" => {
                    "Более резкий фильтр масштабирования; примерно на 20% медленнее билинейного."
                }
                "Nearest-neighbor (pixelated) scaling. Note that this is still slower than Bilinear" => {
                    "Масштабирование ближайшим соседом. Обратите внимание: оно всё ещё медленнее билинейного."
                }
                "Scale the video prior to applying the effect. Real NTSC footage is 480 lines tall. This applies to both the preview and the final render, and is not saved as part of presets." => {
                    "Масштабировать видео перед применением эффекта. Это влияет и на предпросмотр, и на итоговый рендер и не сохраняется в пресетах."
                }
                "Compression level for PNG encoding. Higher compression levels produce smaller files but take longer to render." => {
                    "Уровень сжатия для PNG. Чем выше сжатие, тем меньше файл, но тем дольше рендер."
                }
                "Video quality factor, where 0 is the worst quality and 50 is the best. Higher quality videos take up more space." => {
                    "Коэффициент качества видео: 0 означает худшее качество, 50 - лучшее. Чем выше качество, тем больше размер файла."
                }
                "Encoding speed preset. Higher encoding speeds provide a worse compression ratio, resulting in larger videos at a given quality." => {
                    "Предустановка скорости кодирования. Более высокая скорость означает худшее сжатие и больший размер файла при том же качестве."
                }
                "Subsample the chrominance planes to half the resolution of the luminance plane. Increases playback compatibility." => {
                    "Уменьшить разрешение плоскостей цветности до половины разрешения яркости. Это повышает совместимость воспроизведения."
                }
                "Subsample the chrominance planes to half the resolution of the luminance plane. Results in smaller files." => {
                    "Уменьшить разрешение плоскостей цветности до половины разрешения яркости. Это уменьшает размер файла."
                }
                "To enable interlaced output, set the \"Use field\" setting to \"Interleaved\"." => {
                    "Чтобы включить чересстрочный вывод, установите параметр «Использование поля» в «Чередование»."
                }
                "You're rendering an image sequence into a directory that isn't empty. This will output many individual image files into that directory." => {
                    "Вы рендерите последовательность изображений в непустую папку. В эту папку будет записано много отдельных файлов изображений."
                }
                " (interlaced)" => " (чересстрочно)",
                " (progressive)" => " (прогрессивно)",
                " (telecined)" => " (телесин)",
                _ => text,
            },
            Self::Japanese => match text {
                "About + Credits" => "情報と謝辞",
                "An error occurred while loading" => "読み込み中に問題が起きました",
                "Bit depth" => "色深度",
                "Browse for a path" => "場所を選ぶ",
                "by " => "作者: ",
                "Cancel" => "中止",
                "Check for Updates" => "更新を確認",
                "Check for Updates..." => "更新を確認...",
                "Checking for updates..." => "更新を確認中...",
                "Codec" => "符号化方式",
                "Compression level" => "圧縮の強さ",
                "Copy" => "写す",
                "Copy frame" => "一こまを写す",
                "Dark" => "暗色",
                "Delete" => "削除",
                "Destination file:" => "出力先:",
                "Disable" => "無効",
                "Download from GitHub ⤴" => "GitHub から取得 ⤴",
                "Drop to install presets" => "ここへ置くと設定集を追加",
                "Drop to load media" => "ここへ置くと素材を読み込む",
                "Drop to load preset" => "ここへ置くと設定を読み込む",
                "Duration:" => "長さ:",
                "Easy mode" => "簡易表示",
                "Edit" => "編集",
                "Effect" => "効果",
                "Effect preview" => "効果の試写",
                "Enable" => "有効",
                "Encoding speed" => "符号化の速さ",
                "Error checking for updates:" => "更新確認で問題:",
                "Error creating pipeline" => "処理経路の作成に失敗",
                "Error creating preset" => "設定作成に失敗",
                "Error creating presets directory" => "設定集の場所作成に失敗",
                "Error creating render job" => "書き出し処理の作成に失敗",
                "Error deleting preset" => "設定削除に失敗",
                "Error during render job" => "書き出し中に問題",
                "Error initializing GStreamer" => "GStreamer 初期化に失敗",
                "Error installing preset" => "設定追加に失敗",
                "Error loading presets directory" => "設定集の場所読み込みに失敗",
                "Error loading video" => "映像読み込みに失敗",
                "Error parsing JSON" => "JSON 解析に失敗",
                "Error reading JSON" => "JSON 読み込みに失敗",
                "Error renaming preset" => "設定名変更に失敗",
                "File" => "書類",
                "Filesystem error" => "保存領域の問題",
                "Fit" => "合わせる",
                "Follow system color theme" => "本体の配色に合わせる",
                "fps" => "こま/秒",
                "Help" => "案内",
                "Image sequences do not support interlaced output." => "連番画像は飛び越し出力に対応していません。",
                "Interlaced output" => "飛び越し出力",
                "Language" => "言語",
                "License" => "使用許諾",
                "Light" => "弱め",
                "Light theme" => "明色",
                "lines" => "行",
                "Load" => "読み込む",
                "Load from..." => "書類から読み込む...",
                "Mute" => "消音",
                "No media loaded" => "素材が読み込まれていません",
                "OK" => "OK",
                "Online Documentation ⤴" => "公開文書 ⤴",
                "Only one file at a time can be dropped here" => "ここには一度に 1 つの書類だけ置けます",
                "Open" => "開く",
                "Open containing folder" => "保存先を開く",
                "Open folder" => "場所を開く",
                "Output directory is not empty" => "出力先が空ではありません",
                "Overwrite" => "上書き",
                "Paste" => "貼り付け",
                "Paste JSON" => "JSON を貼り付け",
                "Paused" => "一時停止中",
                "Presets" => "設定集",
                "Quality" => "品質",
                "Quit" => "終了",
                "Random seed" => "乱数種",
                "Randomize seed" => "乱数種を作り直す",
                "Redo" => "やり直し",
                "Reload" => "再読込",
                "Render" => "書き出し",
                "Rendering..." => "書き出し中...",
                "Rename" => "名前を変更",
                "Reset" => "初期値に戻す",
                "Resizing filter" => "大きさ変更の濾波",
                "Save" => "保存",
                "Save as" => "名前を付けて保存",
                "Save frame" => "一こまを保存",
                "Save to..." => "保存先...",
                "Scale to" => "大きさ",
                "Split" => "分割",
                "System" => "本体設定",
                "Theme" => "配色",
                "Third-Party Licenses" => "第三者使用許諾",
                "Time remaining: " => "残り時間: ",
                "Undo" => "元に戻す",
                "Unmute" => "消音を解除",
                "Up to date" => "最新です",
                "Use dark mode" => "暗色表示を使う",
                "Use field" => "使う場",
                "Use light mode" => "明色表示を使う",
                "Used by:" => "使用先:",
                "View" => "表示",
                "Waiting..." => "待機中...",
                "Zoom" => "拡大",
                "Zoom preview" => "試写を拡大",
                "...loosely based on " => "...おおもとは ",
                "...which is a GUI for " => "...これは次の画面操作版: ",
                "...which is a port of " => "...これは次の移植: ",
                "0 degrees" => "0 度",
                "90 degrees" => "90 度",
                "180 degrees" => "180 度",
                "270 degrees" => "270 度",
                "1-line comb" => "1 行くし形濾波",
                "2-line comb" => "2 行くし形濾波",
                "Additional ringing artifacts, simulated with a notch filter." => {
                    "切欠き濾波で再現した余分な輪郭鳴りです。"
                }
                "Alternating" => "交互",
                "Amount of sharpening to apply." => "かける鮮鋭化の量です。",
                "Apply a low-pass filter to the input chrominance (color) signal." => {
                    "入力色信号に低域濾波をかけます。"
                }
                "Apply a low-pass filter to the output chroma signal." => {
                    "出力色信号に低域濾波をかけます。"
                }
                "Apply a notch filter to the input luminance signal. Sharp, but has ringing artifacts." => {
                    "入力輝度信号に切欠き濾波をかけます。鮮鋭ですが輪郭鳴りが出ます。"
                }
                "Apply a simple box filter to the input luminance signal." => {
                    "入力輝度信号に単純な箱型濾波をかけます。"
                }
                "Average the current row with the previous and next ones, phase-cancelling the chrominance (color) signals. Only works if the scanline phase shift is 180 degrees." => {
                    "現在行を前後の行と平均し、色信号を位相で打ち消します。走査線の位相ずれが 180 度のときだけ有効です。"
                }
                "Average the current row with the previous and next ones, phase-cancelling the chrominance signals. Only works if the scanline phase shift is 180 degrees." => {
                    "現在行を前後の行と平均し、色信号を位相で打ち消します。走査線の位相ずれが 180 度のときだけ有効です。"
                }
                "Average the current row with the previous one, phase-cancelling the chrominance (color) signals. Only works if the scanline phase shift is 180 degrees." => {
                    "現在行を前の行と平均し、色信号を位相で打ち消します。走査線の位相ずれが 180 度のときだけ有効です。"
                }
                "Average the current row with the previous one, phase-cancelling the chrominance signals. Only works if the scanline phase shift is 180 degrees." => {
                    "現在行を前の行と平均し、色信号を位相で打ち消します。走査線の位相ずれが 180 度のときだけ有効です。"
                }
                "Base wavelength for the horizontal waving." => "横揺れの基本波長です。",
                "Base wavelength, in pixels, of the noise." => {
                    "雑音の基本波長を画素数で表します。"
                }
                "Boost high frequencies in the NTSC signal, sharpening the image and intensifying colors." => {
                    "NTSC 信号の高周波を強め、絵を鮮鋭にし、色を強めます。"
                }
                "Both" => "両方",
                "Box" => "箱型",
                "Butterworth (sharper)" => "Butterworth（より鮮鋭）",
                "Chance that the chrominance (color) signal is completely lost in each scanline." => {
                    "各走査線で色信号が完全に失われる確率です。"
                }
                "Chance that the chrominance signal is completely lost in each scanline." => {
                    "各走査線で色信号が完全に失われる確率です。"
                }
                "Choose which rows (\"fields\" in NTSC parlance) of the source image will be used." => {
                    "元画像のどの行を使うかを選びます。NTSC ではこれを「場」と呼びます。"
                }
                "Chroma delay (horizontal)" => "色信号遅れ（横）",
                "Chroma delay (vertical)" => "色信号遅れ（縦）",
                "Chroma demodulation filter" => "色信号復調濾波",
                "Chroma loss" => "色抜け",
                "Chroma low-pass in" => "入力色信号の低域濾波",
                "Chroma low-pass out" => "出力色信号の低域濾波",
                "Chroma noise" => "色信号雑音",
                "Chroma phase error" => "色信号位相ずれ",
                "Chroma phase noise" => "色信号位相雑音",
                "Composite signal noise" => "複合信号雑音",
                "Composite signal sharpening" => "複合信号の鮮鋭化",
                "Constant K (blurry)" => "固定 K（ぼかし強め）",
                "Detail" => "細部",
                "Determines whether the speckles are placed truly randomly or concentrated in certain rows." => {
                    "白点を真に無作為に置くか、特定の行に集めるかを決めます。"
                }
                "Do not filter the luminance signal. Adds rainbow artifacts." => {
                    "輝度信号を濾波しません。虹色の乱れが増えます。"
                }
                "Edge wave" => "端の揺れ",
                "Emulate cutoff of high-frequency data at various VHS recording speeds." => {
                    "VHS の録画速度ごとの高周波成分の切り落としを再現します。"
                }
                "Emulate noise from VHS tracking error." => {
                    "VHS の追跡ずれによる雑音を再現します。"
                }
                "Emulate VHS head-switching artifacts at the bottom of the image." => {
                    "画像下部に出る VHS の磁気頭切替による乱れを再現します。"
                }
                "EP (Extended Play)" => "EP（長時間）",
                "FFV1 (Lossless)" => "FFV1（無劣化）",
                "Filter the input luminance to decrease rainbow artifacts." => {
                    "入力輝度を濾波して虹色の乱れを減らします。"
                }
                "Filter used to modulate the chrominance (color) data out of the composite NTSC signal." => {
                    "複合 NTSC 信号から色信号情報を取り出すための濾波です。"
                }
                "Filter with a sharper falloff. Produces sharpened, less blurry results." => {
                    "落ち方が急な濾波です。より鮮鋭で、ぼけの少ない結果になります。"
                }
                "Frequency" => "周波数",
                "Frequency / radius of the sharpening, relative to the tape speed's cutoff frequency." => {
                    "鮮鋭化の周波数 / 半径です。磁気帯速度の切り落とし周波数を基準にします。"
                }
                "Frequency of random speckles in the image." => "画像中の白点雑音の頻度です。",
                "Frequency of speckle-type noise in the artifacts." => {
                    "乱れの中の白点状雑音の頻度です。"
                }
                "Frequency/period of the ringing, in \"rings per pixel\"." => {
                    "輪郭鳴りの周波数 / 周期です。単位は「画素あたりの鳴り数」です。"
                }
                "Full" => "全量",
                "Full-intensity low-pass filter." => "全量の低域濾波です。",
                "H.264" => "H.264",
                "Head switching" => "磁気頭切替",
                "Height" => "高さ",
                "Horizontal offset of the chrominance (color) signal." => {
                    "色信号の横方向のずれです。"
                }
                "Horizontal position at which the head-switching starts." => {
                    "磁気頭切替が始まる横位置です。"
                }
                "Horizontal scale" => "横倍率",
                "Horizontal shift" => "横ずれ",
                "Horizontal waving of the image." => "画像の横方向の揺れです。",
                "Horizontal waving of the image, in pixels." => {
                    "画像の横方向の揺れを画素数で表します。"
                }
                "Horizontally scale the effect by this amount. For 480p video, leave this at 1.0 for the most physically-accurate result." => {
                    "効果を横方向にこの量だけ拡大縮小します。480p の映像では、もっとも実物に近い結果にするため 1.0 のままにします。"
                }
                "How much of the head-switching artifact is off-screen." => {
                    "磁気頭切替の乱れが画面外に出る度合いです。"
                }
                "How much the affected scanlines \"wave\" back and forth." => {
                    "影響を受ける走査線が前後に揺れる度合いです。"
                }
                "How much the head-switching artifact \"jitters\" horizontally." => {
                    "磁気頭切替の乱れが横方向に揺らぐ度合いです。"
                }
                "How much the head-switching artifact shifts rows horizontally." => {
                    "磁気頭切替の乱れが行を横へずらす度合いです。"
                }
                "How much the speckles are clustered by scanline." => {
                    "白点が走査線ごとにまとまる度合いです。"
                }
                "Input luma filter" => "入力輝度濾波",
                "Intensity" => "強さ",
                "Intensity of non-speckle noise." => "白点以外の雑音の強さです。",
                "Intensity of the noise." => "雑音の強さです。",
                "Intensity of the ringing." => "輪郭鳴りの強さです。",
                "Interleaved (lower first)" => "交互（下の場から）",
                "Interleaved (upper first)" => "交互（上の場から）",
                "Jitter" => "揺らぎ",
                "Less intense low-pass filter." => "弱めの低域濾波です。",
                "Lower only" => "下の場のみ",
                "Lowpass filter type" => "低域濾波の種類",
                "LP (Long Play)" => "LP（長時間再生）",
                "Luma noise" => "輝度雑音",
                "Luma smear" => "輝度にじみ",
                "Multiply the scaling factors by the video's height. Prefer scaling the input video to 480p instead, which gives much more accurate-looking results." => {
                    "倍率に映像の高さを掛けます。代わりに入力映像を 480p へ変える方が、はるかに正確に見えます。"
                }
                "No low-pass filter." => "低域濾波なし。",
                "Noise applied per-scanline to the phase of the chrominance (color) signal." => {
                    "色信号の位相へ、走査線ごとに加える雑音です。"
                }
                "Noise applied per-scanline to the phase of the chrominance signal." => {
                    "色信号の位相へ、走査線ごとに加える雑音です。"
                }
                "Noise applied to the chrominance (color) signal." => "色信号にかける雑音です。",
                "Noise applied to the chrominance signal." => "色信号にかける雑音です。",
                "Noise applied to the composite NTSC signal." => {
                    "複合 NTSC 信号にかける雑音です。"
                }
                "Noise applied to the luminance signal. Useful for higher-frequency noise than the \"Composite noise\" setting can provide." => {
                    "輝度信号にかける雑音です。「複合雑音」設定より高周波の雑音に向いています。"
                }
                "Noise intensity" => "雑音の強さ",
                "None" => "なし",
                "Notch" => "切欠き",
                "Notch filter. Sharper than a box blur, but with ringing artifacts." => {
                    "切欠き濾波です。箱型ぼかしより鮮鋭ですが、輪郭鳴りが出ます。"
                }
                "Octaves of noise for the waves." => "波に使う雑音の重ね数です。",
                "Octaves of noise." => "雑音の重ね数です。",
                "Offset" => "ずれ",
                "Phase error for the chrominance (color) signal." => "色信号の位相ずれです。",
                "Phase shift of the chrominance (color) signal each scanline. Usually 180 degrees." => {
                    "走査線ごとの色信号の位相ずれです。通常は 180 度です。"
                }
                "PNG Sequence" => "PNG 連番画像",
                "Position" => "位置",
                "Power" => "強さ",
                "Ringing" => "輪郭鳴り",
                "Saturation" => "彩度",
                "Scale" => "倍率",
                "Scale the effect by these factors." => "これらの係数で効果を拡大縮小します。",
                "Scale with video size" => "映像の大きさに合わせる",
                "Scanline phase shift" => "走査線の位相ずれ",
                "Scanline phase shift offset" => "走査線位相ずれの基準",
                "Sharpen" => "鮮鋭化",
                "Sharpening of the image, as done by some VHS decks." => {
                    "一部の VHS 機器のように画像を鮮鋭化します。"
                }
                "Simple constant-k filter. Produces longer, blurry results." => {
                    "単純な固定 K 濾波です。長く、ぼけた結果になります。"
                }
                "Simple horizontal box blur." => "単純な横方向の箱型ぼかしです。",
                "Skip every lower row, keeping the upper ones." => {
                    "下の行を飛ばして上の行を残します。"
                }
                "Skip every other row, alternating between skipping even and odd rows." => {
                    "一行おきに飛ばし、偶数行と奇数行のどちらを飛ばすかを交互に変えます。"
                }
                "Skip every upper row, keeping the lower ones." => {
                    "上の行を飛ばして下の行を残します。"
                }
                "Snow" => "白点",
                "Snow anisotropy" => "白点の方向むら",
                "Snow intensity" => "白点の強さ",
                "SP (Standard Play)" => "SP（標準再生）",
                "Speed" => "速さ",
                "Speed at which the horizontal waving occurs." => "横揺れが起きる速さです。",
                "Start mid-line" => "行の途中から始める",
                "Start the head-switching artifact mid-scanline, with some static where it begins." => {
                    "磁気頭切替の乱れを走査線の途中から始め、開始位置に砂嵐を少し加えます。"
                }
                "Tape speed" => "磁気帯速度",
                "The low-pass filter to use throughout the effect." => {
                    "効果全体で使う低域濾波です。"
                }
                "The power of the notch filter / how far out the ringing extends." => {
                    "切欠き濾波の強さ、つまり輪郭鳴りがどこまで伸びるかです。"
                }
                "Total height of the head-switching artifact." => {
                    "磁気頭切替による乱れの総高さです。"
                }
                "Total height of the tracking artifacts." => "追跡ずれによる乱れの総高さです。",
                "Tracking noise" => "追跡ずれ雑音",
                "Treat the video as interlaced, with the lower field as the earlier frame." => {
                    "映像を飛び越し方式として扱い、下の場を先のこまとします。"
                }
                "Treat the video as interlaced, with the upper field as the earlier frame." => {
                    "映像を飛び越し方式として扱い、上の場を先のこまとします。"
                }
                "Upper only" => "上の場のみ",
                "Use all rows; don't skip any." => "すべての行を使い、飛ばしません。",
                "Vertical offset of the chrominance (color) signal. Usually increases with VHS generation loss." => {
                    "色信号の縦方向のずれです。VHS の世代劣化で大きくなりがちです。"
                }
                "Vertical scale" => "縦倍率",
                "Vertically blend chroma" => "色信号を縦に混ぜる",
                "Vertically blend each scanline's chrominance with the scanline above it." => {
                    "各走査線の色信号を、その上の走査線と縦方向に混ぜます。"
                }
                "Vertically scale the effect by this amount. You should probably leave this at 1.0." => {
                    "効果を縦方向にこの量だけ拡大縮小します。たぶん 1.0 のままがよいです。"
                }
                "VHS emulation" => "VHS 再現",
                "Wave intensity" => "波の強さ",
                "4:2:0 chroma subsampling" => "4:2:0 色信号間引き",
                "8-bit" => "8 bit",
                "10-bit" => "10 bit",
                "12-bit" => "12 bit",
                "Nearest" => "最近傍",
                "Bilinear" => "双線形",
                "Bicubic" => "三次補間",
                "Lower-quality but faster scaling filter" => "品質は低めですが速い大きさ変更濾波",
                "Sharper scaling filter; ~20% slower than Bilinear" => {
                    "より鮮鋭な大きさ変更濾波です。双線形より約 20% 遅くなります。"
                }
                "Nearest-neighbor (pixelated) scaling. Note that this is still slower than Bilinear" => {
                    "最近傍で大きさを変えます。角ばった見た目になります。なお、双線形より遅いです。"
                }
                "Scale the video prior to applying the effect. Real NTSC footage is 480 lines tall. This applies to both the preview and the final render, and is not saved as part of presets." => {
                    "効果をかける前に映像の大きさを変えます。本物の NTSC 映像は高さ 480 行です。この設定は試写と最終書き出しの両方に働き、設定集には保存されません。"
                }
                "Compression level for PNG encoding. Higher compression levels produce smaller files but take longer to render." => {
                    "PNG 符号化の圧縮の強さです。高くするほど容量は小さくなりますが、書き出し時間は長くなります。"
                }
                "Video quality factor, where 0 is the worst quality and 50 is the best. Higher quality videos take up more space." => {
                    "動画品質係数です。0 が最低、50 が最高です。品質が高いほど容量は大きくなります。"
                }
                "Encoding speed preset. Higher encoding speeds provide a worse compression ratio, resulting in larger videos at a given quality." => {
                    "符号化速度の設定です。速くするほど圧縮率が下がり、同じ品質でも容量が大きくなります。"
                }
                "Subsample the chrominance planes to half the resolution of the luminance plane. Increases playback compatibility." => {
                    "色信号面を輝度面の半分の解像度に間引きます。再生互換性が上がります。"
                }
                "Subsample the chrominance planes to half the resolution of the luminance plane. Results in smaller files." => {
                    "色信号面を輝度面の半分の解像度に間引きます。容量が小さくなります。"
                }
                "To enable interlaced output, set the \"Use field\" setting to \"Interleaved\"." => {
                    "飛び越し出力を有効にするには、「使う場」を「交互」に設定してください。"
                }
                "You're rendering an image sequence into a directory that isn't empty. This will output many individual image files into that directory." => {
                    "空でない場所に連番画像を書き出そうとしています。この場所へ多数の個別画像を出力します。"
                }
                " (interlaced)" => "（飛び越し）",
                " (progressive)" => "（順次）",
                " (telecined)" => "（3:2 変換）",
                _ => text,
            },
        };

        Cow::Borrowed(translated)
    }

    pub fn latest_version_label(self, tag_name: &str) -> String {
        match self {
            Self::English => format!("Latest version: {tag_name}"),
            Self::SimplifiedChinese => format!("最新版本：{tag_name}"),
            Self::Russian => format!("Последняя версия: {tag_name}"),
            Self::Japanese => format!("最新版: {tag_name}"),
        }
    }

    pub fn rendering_progress(self, position: ClockTime, duration: ClockTime) -> String {
        match self {
            Self::English => format!("Rendering... ({position:.2} / {duration:.2})"),
            Self::SimplifiedChinese => format!("正在渲染...（{position:.2} / {duration:.2}）"),
            Self::Russian => format!("Рендеринг... ({position:.2} / {duration:.2})"),
            Self::Japanese => format!("書き出し中...（{position:.2} / {duration:.2}）"),
        }
    }

    pub fn completed_in(self, duration: ClockTime) -> String {
        match self {
            Self::English => format!("Completed in {duration:.2}"),
            Self::SimplifiedChinese => format!("完成，用时 {duration:.2}"),
            Self::Russian => format!("Завершено за {duration:.2}"),
            Self::Japanese => format!("{duration:.2} で完了"),
        }
    }

    pub fn render_error(self, error: &str) -> String {
        match self {
            Self::English => format!("Error: {error}"),
            Self::SimplifiedChinese => format!("错误：{error}"),
            Self::Russian => format!("Ошибка: {error}"),
            Self::Japanese => format!("問題: {error}"),
        }
    }

    pub fn eta_units(self) -> [[&'static str; 2]; 3] {
        match self {
            Self::English => [
                [" hour", " hours"],
                [" minute", " minutes"],
                [" second", " seconds"],
            ],
            Self::SimplifiedChinese => [[" 小时", " 小时"], [" 分钟", " 分钟"], [" 秒", " 秒"]],
            Self::Russian => [
                [" час", " часов"],
                [" минута", " минут"],
                [" секунда", " секунд"],
            ],
            Self::Japanese => [[" 時間", " 時間"], [" 分", " 分"], [" 秒", " 秒"]],
        }
    }

    pub fn format_application_error(self, error: &ApplicationError) -> String {
        match error {
            ApplicationError::GstreamerInit { source } => {
                format!("{}: {source}", self.text("Error initializing GStreamer"))
            }
            ApplicationError::LoadVideo { source } => {
                format!("{}: {source}", self.text("Error loading video"))
            }
            ApplicationError::CreatePipeline { source } => {
                format!("{}: {source}", self.text("Error creating pipeline"))
            }
            ApplicationError::CreateRenderJob { source } => {
                format!("{}: {source}", self.text("Error creating render job"))
            }
            ApplicationError::RenderJobPipeline { source } => {
                format!("{}: {source}", self.text("Error during render job"))
            }
            ApplicationError::JSONRead { source } => {
                format!("{}: {source}", self.text("Error reading JSON"))
            }
            ApplicationError::JSONParse { source } => {
                format!("{}: {source}", self.text("Error parsing JSON"))
            }
            ApplicationError::CreatePresetsDirectory { source } => {
                format!("{}: {source}", self.text("Error creating presets directory"))
            }
            ApplicationError::CreatePresetFile { source } => {
                format!("{}: {source}", self.text("Error creating preset"))
            }
            ApplicationError::CreatePresetJSON { source } => {
                format!("{}: {source}", self.text("Error creating preset"))
            }
            ApplicationError::DeletePreset { source } => {
                format!("{}: {source}", self.text("Error deleting preset"))
            }
            ApplicationError::RenamePreset { source } => {
                format!("{}: {source}", self.text("Error renaming preset"))
            }
            ApplicationError::InstallPreset { source } => {
                format!("{}: {source}", self.text("Error installing preset"))
            }
            ApplicationError::Fs { source } => {
                format!("{}: {source}", self.text("Filesystem error"))
            }
            ApplicationError::DroppedMultipleFiles => {
                self.text("Only one file at a time can be dropped here")
                    .into_owned()
            }
        }
    }
}
