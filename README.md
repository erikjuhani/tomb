<p align="center">
  <img src="https://github.com/user-attachments/assets/6e5461df-fc66-4f67-b2f1-7989cae921fa"/>
</p>

<h1 align="center">Tomb</h1>

<p align="center">
  Personal todo list and task helper CLI tool
</p>

<p align="center">
  <i>To</i> (Todo) - <i>M</i> (Markdown) - <i>B</i> (Barrow)
</p>

> [!IMPORTANT]
> The first iteration of `tomb` is built by composing Unix tools like `grep` and `sed`. However, the next version will be properly developed using a statically-typed programming language, [🦀](https://www.rust-lang.org/).

<!--toc:start-->
- [Background](#background)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Special Keywords](#special-keywords)
    - [Dates](#dates)
    - [Tasks](#tasks)
  - [Commands](#commands)
    - [Open](#open)
    - [Tasks](#tasks)
- [Sketchybar Integration](#sketchybar-integration)
<!--toc:end-->

## Background

I've struggled to find a suitable tool that feels great to use for tracking my daily tasks. The most essential feature is the ability to edit tasks in a buffer with an editor I’m comfortable with ([<img width="9px" src="https://upload.wikimedia.org/wikipedia/commons/3/3a/Neovim-mark.svg"/>](https://neovim.io/)). Inspired by my coworker [@ssiltanen](https://github.com/ssiltanen), I decided to create my own!

Introducing `tomb` — a simple Markdown-based todo "barrow" to unearth your buried tasks and keep you accountable when new tasks need to be "laid to rest." You are the taskyard keeper!

## Features

- Task management in a single markdown file
- Open todo file buffer directly to your editor
- Easily integrated with other tools (Like version control or [sketchybar](https://felixkratz.github.io/SketchyBar/))

## Installation

The easiest way to install `tomb` is with [shm](https://github.com/erikjuhani/shm).

To install `shm`, run one of the following commands:

**Using curl:**

```sh
curl -sSL https://raw.githubusercontent.com/erikjuhani/shm/main/shm.sh | sh
```

**Using wget:**

```sh
wget -qO- https://raw.githubusercontent.com/erikjuhani/shm/main/shm.sh | sh
```

Then, run the following command to get the latest version of `tomb`:

```sh
shm get erikjuhani/tomb
```

## Usage

`tomb` is designed to be used alongside other tools like `tmux` and `sketchybar`, though it can also be used on its own.

To set up a `tmux` popup for `tomb`, add the following to `~/.tmux.conf`:

```.tmux.conf
bind-key -n C-t run-shell "tmux popup -E 'tomb open || exit 0'"
```

This binds the `tomb open` command to <kbd>C-t</kbd>, opening the `tomb` Markdown file in a `tmux` popup using the `neovim` editor.

### Special Keywords

`tomb` searches for specific patterns in the `tomb` tasks file. The file can also contain additional headings or notes.

Example `.tomb.md` file:

```md
## Notes

- Remember to check the README for grammar errors!

## Tasks

### 03.11.2024

- [ ] Open task
- [.] Task in-progress
- [x] Completed task
```

To move tasks to other dates, edit the file directly and manually move tasks to a new date heading. For example, if a task is not completed, simply copy it to the next date heading:

```md
### 04.11.2024

- [.] Task I started working on

### 03.11.2024

- [.] Task I started working on
```

#### Dates

Currently, all tasks need to be grouped under a specific date heading using this format: `dd.mm.yyyy`.

```md
### 03.11.2024

- [ ] Task
```

#### Tasks

There are three task types, following GitHub Markdown syntax for tasks with one exception: "in-progress" tasks.

1. **Open tasks** use this pattern:

```md
- [ ] Open task
```

2. **In-progress tasks** use a `.` (dot):

```md
- [.] In-progress task
```

3. **Completed tasks** use an `x`:

```md
- [x] Completed task
```

### Commands

#### Open

```sh
tomb open [date] [-h|--help]
```

Opens the `.tomb.md` file in the editor. If a date argument is given, the file will open at the specified line if the date exists (works only in `neovim` and `vim`).


Change the default editor (nvim) to another by providing EDITOR=vim variable when calling the open command.

```sh
EDITOR=vim tomb open
```

#### Tasks

```sh
tomb tasks [date] [-c|--completed] [-o|--open] [-p|--progress] [--count] [-h|--help]
```

Lists tasks from the `.tomb.md` file based on the provided options. If no options are specified, it returns all tasks for the current date. The `--count` option returns a count of tasks, which can be used in combination with task type options like `--completed`.

## Sketchybar Integration

To display the number of tasks in `sketchybar`, copy the plugin from `sketchybar/plugins/tomb-items.sh` using one of the following commands:

```sh
# Using curl
curl -sSL https://raw.githubusercontent.com/erikjuhani/tomb/main/sketchybar/plugins/tomb-plugin.sh -o "${HOME}/.config/sketchybar/plugins/tomb-plugin.sh" && chmod +x "${HOME}/.config/sketchybar/plugins/tomb-plugin.sh"

# Using wget
wget -q https://raw.githubusercontent.com/erikjuhani/tomb/main/sketchybar/plugins/tomb-plugin.sh -O "${HOME}/.config/sketchybar/plugins/tomb-plugin.sh" && chmod +x "${HOME}/.config/sketchybar/plugins/tomb-plugin.sh"
```

Then, add the following configuration to your `sketchybarrc` file:

```sketchybarrc
sketchybar --add item tomb right \
           --set tomb update_freq=300 script="$PLUGIN_DIR/tomb-plugin.sh" \
           --add item tomb.completed right \
           --add item tomb.progress right \
           --add item tomb.open right
```

The update frequency is set to five minutes (300 seconds). To see updates more quickly when modifying the file, set the update frequency to a smaller value, like 1 second.

Finally, reload `sketchybar`:

```sh
sketchybar --reload
```

You should now see something similar on your bar:

<img width="346" src="https://github.com/user-attachments/assets/15f89ed3-037b-4a6c-80c5-25af85864f88">
