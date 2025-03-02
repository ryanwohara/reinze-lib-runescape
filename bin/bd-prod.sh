#!/usr/bin/env bash
# bd = build & deploy

cargo build --release && rm -rf ../rust-reinze/plugins/libreinze_lib_runescape_*.so && rm -rf ../rust-reinze/plugins/libreinze_lib_runescape*.so && cp target/release/libreinze_lib_runescape.so ../rust-reinze/plugins/libreinze_lib_runescape_$(date "+%Y%m%dT%H%M%S").so
