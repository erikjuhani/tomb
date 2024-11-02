#!/usr/bin/env sh

set -e

err() {
  printf >&2 "error: %s\n" "$@"
  exit 1
}

TOMB_LOCATION="${HOME}/.tomb.md"
EDITOR=${EDITOR:-nvim}

help() {
  cat <<EOF
tomb
a simple Markdown-based todo "barrow" to unearth your buried tasks and keep you
accountable when new tasks need to be "laid to rest." You are the taskyard
keeper!

USAGE

	tomb [<command>] [<args>] [-h | --help]

COMMANDS
	open	Opens the .tomb.md file in the editor
	tasks	Lists tasks from the .tomb.md file based on the provided options

OPTIONS
	-h --help	Show help

For additional help use tomb <command> -h
EOF
  exit 0
}

help_open() {
  cat <<EOF
tomb open
Opens the .tomb.md file in the editor. If a date argument is given, the file
will open at the specified line if the date exists (works only in neovim and vim).

Change the default editor (nvim) to another by providing EDITOR=vim variable
when calling the open command.

	EDITOR=vim tomb open

USAGE
	open [<date>]

OPTIONS
	-h --help	Show help

EOF
  exit 2
}

help_tasks() {
  cat <<EOF
tomb tasks
Lists tasks from the .tomb.md file based on the provided options. If no options
are specified, it returns all tasks for the current date. The --count option
returns a count of tasks, which can be used in combination with task type
options like --completed.

USAGE
	tasks [<date>] [<option>]

OPTIONS
	-c --completed	Lists completed tasks
	-o --open	Lists open tasks
	-p --progress	Lists tasks in-progress
	--count	Returns the number of tasks. Works in conjuction with other tasks options
	-h --help	Show help

EOF
  exit 2
}

get_tasks() {
  for arg; do
    [ -n "${option}" ] && err "Too many options, got ${option} and ${arg}, expected 1"
    case "${arg}" in
      -o | --open) option="open" ;;
      -c | --completed) option="completed" ;;
      -p | --progress) option="in progress" ;;
      --count) count=1 ;;
      -h | --help) help_tasks ;;
      -*) err "Unknown option $1" ;;
      *)
        [ -n "${input_date}" ] && err "Too many arguments, got ${input_date} and ${arg}, expected 1"
        input_date="${arg}"
        ;;
    esac
  done

  # Task patterns
  # - [ ] Open
  # - [.] In Progress
  # - [x] Complete

  case "${option}" in
    open) task_pattern="\[\ \]" ;;
    completed) task_pattern="\[x\]" ;;
    "in progress") task_pattern="\[\.\]" ;;
  esac

  # Defaults to pattern that finds every type of task
  task_pattern="${task_pattern:-\[.\]}"
  tasks="$(sed -n "/${input_date:-$(date '+%d.%m.%Y')}/,/###/p" "${TOMB_LOCATION}" | sed '$d')"

  if [ "${count:-0}" -eq 1 ]; then
    printf "%b" "${tasks}" | grep -c "${task_pattern}" || exit 0
  else
    printf "%b" "${tasks}" | grep "${task_pattern}" || printf "%b\n" "No ${option} tasks found for date ${input_date}" && exit 2
  fi
}

open_tomb() {
  if ! command -v "${EDITOR}" >/dev/null; then
    err "Cannot find editor ${EDITOR}"
  fi

  for arg; do
    case "${arg}" in
      -h | --help) help_open ;;
      -*) err "Unknown option $1" ;;
      *)
        [ -n "${input_date}" ] && err "Too many arguments, got ${input_date} and ${arg}, expected 1"
        input_date="${arg}"
        ;;
    esac
  done

  input_date="${input_date:-$(date '+%d.%m.%Y')}"

  line_number="$(grep -e "${input_date}" -n "${TOMB_LOCATION}" | cut -d : -f 1)"
  line_number="${line_number:-0}"

  "${EDITOR}" "${TOMB_LOCATION}" "+:${line_number}"
}

enum() {
  enum_str="$1"
  shift
  for arg; do
    enum_str="${enum_str}|${arg}"
  done

  printf "^(%s)$" "${enum_str}"
}

parse_command() {
  if printf "%s" "$1" | grep -qE "$2"; then
    readonly CMD="$1"
  fi
}

tomb() {
  [ "$#" -eq 0 ] && help

  commands="$(enum open tasks)"

  for arg; do
    if [ -z "${CMD}" ]; then
      parse_command "${arg}" "${commands}"
      shift
    fi
    case "${arg}" in
      -h | --help) help ;;
    esac
  done

  case "${CMD}" in
    open) open_tomb "$@" ;;
    tasks) get_tasks "$@" ;;
    *)
      printf "error: %s\n" "Unknown command"
      help
      ;;
  esac
}

tomb "$@"
