use html5ever::parse_document;
// use html5ever::tendril::StrTendril;
use html5ever::tendril::TendrilSink;
// use markup5ever::interface::Attribute;
use markup5ever_rcdom::Handle;
use markup5ever_rcdom::NodeData;
use markup5ever_rcdom::RcDom;
use std::borrow::Borrow;
// use std::cell::RefCell;
use std::default::Default;

/// Simplify the process of keep walking through the results
macro_rules! keep_going {
    ($source:ident, $target:ident) => {
        for child in $source.children.borrow().iter() {
            walk(child.borrow(), $target);
        }
    };
}

fn walk(input: &Handle, result: &mut String) {
    match input.data {
        NodeData::Text { ref contents } => {
            let text = contents.borrow().to_string();
            result.push_str(&text);
            keep_going!(input, result);
        }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let tag = name.local.to_string();
            match tag.as_ref() {
                "html" | "head" | "body" => keep_going!(input, result),
                "p" => {
                    keep_going!(input, result);
                    result.push_str("\n");
                }
                "span" => {
                    let attrs = attrs.borrow();
                    let classes_attr = attrs
                        .iter()
                        .find(|attr| attr.name.local.to_string() == "class");
                    match classes_attr {
                        Some(classes) => {
                            if classes.value.contains("ellipsis") {
                                keep_going!(input, result);
                                result.push_str("...");
                            } else if !classes.value.contains("invisible") {
                                keep_going!(input, result);
                            }
                        }
                        None => keep_going!(input, result),
                    }
                }
                "a" => {
                    let attrs = attrs.borrow();
                    let rels = attrs
                        .iter()
                        .find(|attr| attr.name.local.to_string() == "rel");
                    let hrefs = attrs
                        .iter()
                        .find(|attr| attr.name.local.to_string() == "href");
                    match (rels, hrefs) {
                        (Some(rel), Some(href)) => {
                            if !rel.value.to_string().contains("tag") {
                                result.push_str("[[");
                                result.push_str(&href.value);
                                result.push_str("][");
                                keep_going!(input, result);
                                result.push_str("]]");
                            } else {
                                keep_going!(input, result);
                            }
                        }
                        _ => keep_going!(input, result),
                    }
                }
                _ => {}
            }
        }
        _ => {
            keep_going!(input, result);
        }
    };
}

fn build_nodes(source: &str) {
    println!("Source: {}", source);
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut source.as_bytes())
        .unwrap();
    let mut result = String::new();
    walk(&dom.document, &mut result);
    println!("Result: {}", result);
}

fn main() {
    let example_1 = String::from(r#"<p>A simple text component</p>"#);
    build_nodes(&example_1);

    let example_2 = String::from(
        r#"<p><span class="invisible">THis is not visible</span><span class="ellipsis">but this is</span></p>"#,
    );
    build_nodes(&example_2);

    let example_3 = String::from(
        r#"<p><a href="mention" class="u-url mention" rel="nofollow noopener noreferrer" target="_blank">@This is a mention</a> and <a href="tag" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#this is a tag</a></p>"#,
    );
    build_nodes(&example_3);

    let example_full_1 = String::from(
        r#"<p>Today I finally moved with my contact and calendar management into the terminal with <a href="https://fosstodon.org/tags/vdirsyncer" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>vdirsyncer</span></a> <a href="https://fosstodon.org/tags/khal" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>khal</span></a> and <a href="https://fosstodon.org/tags/khard" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>khard</span></a>.</p><p>Thank you <span class="h-card"><a href="https://fosstodon.org/@hund" class="u-url mention" rel="nofollow noopener noreferrer" target="_blank">@<span>hund</span></a></span> for your great post: <a href="https://hund.tty1.se/2020/08/12/how-to-sync-and-manage-your-caldav-and-carddav-via-the-terminal.html" rel="nofollow noopener noreferrer" target="_blank"><span class="invisible">https://</span><span class="ellipsis">hund.tty1.se/2020/08/12/how-to</span><span class="invisible">-sync-and-manage-your-caldav-and-carddav-via-the-terminal.html</span></a></p><p><a href="https://fosstodon.org/tags/carddav" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>carddav</span></a> <a href="https://fosstodon.org/tags/caldav" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>caldav</span></a> <a href="https://fosstodon.org/tags/terminal" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>terminal</span></a></p>"#,
    );
    build_nodes(&example_full_1);
}
