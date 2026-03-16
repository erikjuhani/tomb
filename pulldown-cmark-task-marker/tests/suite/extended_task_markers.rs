use super::test_markdown_html;

#[test]
fn extended_slash_marker_html() {
    let original = r##"- [/] in progress
"##;
    let expected = r##"<ul>
<li><input disabled="" type="checkbox" checked=""/>
in progress</li>
</ul>
"##;
    test_markdown_html(original, expected, false, false, false, false, false);
}

#[test]
fn extended_dash_marker_html() {
    let original = r##"- [-] cancelled
"##;
    let expected = r##"<ul>
<li><input disabled="" type="checkbox" checked=""/>
cancelled</li>
</ul>
"##;
    test_markdown_html(original, expected, false, false, false, false, false);
}

#[test]
fn standard_markers_still_work_html() {
    let original = r##"- [ ] todo
- [x] done
"##;
    let expected = r##"<ul>
<li><input disabled="" type="checkbox"/>
todo</li>
<li><input disabled="" type="checkbox" checked=""/>
done</li>
</ul>
"##;
    test_markdown_html(original, expected, false, false, false, false, false);
}
