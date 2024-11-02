#!/usr/bin/env sh

# Include tomb in PATH when using shm
export PATH="${PATH}:${HOME}/.shm"

item() {
  sketchybar --set "$1" icon="$2" label="$4" \
    label.drawing="$5" icon.drawing="$5" \
    label.font="BlexMono Nerd Font Mono:Regular:11.0" label.y_offset=1 \
    icon.font="BlexMono Nerd Font Mono:Regular:28.0" icon.color="$3" \
    click_script="open -a Alacritty && tmux popup -E 'tomb open'"
}

no_tasks_drawing() {
  item "${NAME}" "󱍥" "0xffe67e80" "No tasks set for today |" "$1"
}

in_progress_drawing() {
  item "${NAME}.progress" "󱞎" "0xff7fbbb3" "${in_progress_count} in progress |" "$1"
}

completed_drawing() {
  item "${NAME}.completed" "󱍧" "0xffa7c080" "${completed_count} completed |" "$1"
}

open_drawing() {
  item "${NAME}.open" "󱍤" "0xffd3c6aa" "${open_count} open |" "$1"
}

completed_count="$(tomb tasks --count --completed)"
open_count="$(tomb tasks --count --open)"
in_progress_count="$(tomb tasks --count --progress)"

if [ "${completed_count}" -eq 0 ] && [ "${open_count}" -eq 0 ] && [ "${in_progress_count}" -eq 0 ]; then
  no_tasks_drawing "on"
else
  no_tasks_drawing "off"
fi

if [ "${in_progress_count}" -gt 0 ]; then
  in_progress_drawing "on"
else
  in_progress_drawing "off"
fi

if [ "${completed_count}" -gt 0 ]; then
  completed_drawing "on"
else
  completed_drawing "off"
fi

if [ "${open_count}" -gt 0 ]; then
  open_drawing "on"
else
  open_drawing "off"
fi
