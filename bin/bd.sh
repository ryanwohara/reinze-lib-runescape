#!/usr/bin/env bash
# bd = build & deploy

cargo build && rm -rf ../rust-reinze/plugins/libreinze_lib_runescape_*.so && cp target/debug/libreinze_lib_runescape.so ../rust-reinze/
plugins/libreinze_lib_runescape_$(date "+%Y%m%dT%H%M%S").so
