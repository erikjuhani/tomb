use pulldown_cmark::{Event, Options, Parser};

fn parse_events(input: &str, extended: bool) -> Vec<Event<'_>> {
    let mut opts = Options::ENABLE_TASKLISTS;
    if extended {
        opts |= Options::ENABLE_EXTENDED_TASK_MARKERS;
    }
    Parser::new_ext(input, opts).collect()
}

#[test]
fn slash_emits_extended_marker() {
    let events = parse_events("- [/] in progress\n", true);
    assert!(events.iter().any(|e| matches!(e, Event::ExtendedTaskListMarker('/'))));
}

#[test]
fn dash_emits_extended_marker() {
    let events = parse_events("- [-] cancelled\n", true);
    assert!(events.iter().any(|e| matches!(e, Event::ExtendedTaskListMarker('-'))));
}

#[test]
fn standard_markers_emit_task_list_marker_with_extended() {
    let events = parse_events("- [x] done\n- [ ] todo\n", true);
    assert!(events.iter().any(|e| matches!(e, Event::TaskListMarker(true))));
    assert!(events.iter().any(|e| matches!(e, Event::TaskListMarker(false))));
}

#[test]
fn standard_markers_without_extended() {
    let events = parse_events("- [x] done\n- [ ] todo\n", false);
    assert!(events.iter().any(|e| matches!(e, Event::TaskListMarker(true))));
    assert!(events.iter().any(|e| matches!(e, Event::TaskListMarker(false))));
}

#[test]
fn extended_ignored_without_option() {
    let events = parse_events("- [/] not a task\n", false);
    assert!(!events.iter().any(|e| matches!(e, Event::ExtendedTaskListMarker(_))));
    assert!(!events.iter().any(|e| matches!(e, Event::TaskListMarker(_))));
}

#[test]
fn mixed_standard_and_extended_markers() {
    let events = parse_events("- [x] done\n- [ ] todo\n- [/] in progress\n- [-] cancelled\n", true);
    assert!(events.iter().any(|e| matches!(e, Event::TaskListMarker(true))));
    assert!(events.iter().any(|e| matches!(e, Event::TaskListMarker(false))));
    assert!(events.iter().any(|e| matches!(e, Event::ExtendedTaskListMarker('/'))));
    assert!(events.iter().any(|e| matches!(e, Event::ExtendedTaskListMarker('-'))));
}

#[test]
fn empty_brackets_not_a_task() {
    let events = parse_events("- [] nope\n", true);
    assert!(!events.iter().any(|e| matches!(e, Event::ExtendedTaskListMarker(_))));
    assert!(!events.iter().any(|e| matches!(e, Event::TaskListMarker(_))));
}

#[test]
fn any_single_char_emits_extended_marker() {
    for ch in ['/', '-', '!', '?', '>', '<', '*', '~', '+', '1'] {
        let input = format!("- [{}] task\n", ch);
        let events = parse_events(&input, true);
        assert!(
            events
                .iter()
                .any(|e| matches!(e, Event::ExtendedTaskListMarker(c) if *c == ch)),
            "expected ExtendedTaskListMarker('{}') for input: {}",
            ch,
            input.trim()
        );
    }
}

#[test]
fn multi_char_not_a_task() {
    let events = parse_events("- [ab] nope\n", true);
    assert!(!events.iter().any(|e| matches!(e, Event::ExtendedTaskListMarker(_))));
    assert!(!events.iter().any(|e| matches!(e, Event::TaskListMarker(_))));
}
