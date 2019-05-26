/*
bindgen --no-layout-tests  -o ./src/avcodec/mod.rs /usr/include/libavcodec/avcodec.h --raw-line="#[link(name = \"avcodec\")] extern {}"
bindgen --no-layout-tests  -o ./src/avformat/mod.rs /usr/include/libavformat/avformat.h --raw-line="#[link(name = \"avformat\")] extern {}"
bindgen --no-layout-tests  -o ./src/avutil/mod.rs /usr/include/libavutil/avutil.h --raw-line="#[link(name = \"avutil\")] extern {}
bindgen --no-layout-tests  -o ./src/avutil/opt.rs /usr/include/libavutil/opt.h --raw-line="#[link(name = \"avutil\")] extern {}"
bindgen --no-layout-tests  -o ./src/avutil/samplefmt.rs /usr/include/libavutil/samplefmt.h --raw-line="#[link(name = \"avutil\")] extern {}"
bindgen --no-layout-tests  -o ./src/samplerate.rs /usr/include/samplerate.h --raw-line "#[cfg(target_os = \"windows\")]" --raw-line "#[link(name = \"libsamplerate-0\")] extern {}" --raw-line "#[cfg(not(target_os = \"windows\"))]" --raw-line "#[link(name = \"samplerate\")] extern {}"
*/

#![allow(warnings)]

pub mod avcodec;
pub mod avformat;
pub mod avutil;
pub mod samplerate;

