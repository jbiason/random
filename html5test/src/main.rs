use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::Handle;
use markup5ever_rcdom::NodeData;
use markup5ever_rcdom::RcDom;
use std::borrow::Borrow;
use std::default::Default;
use textwrap::fill;
use textwrap::Options;

fn go_children(input: &Handle, result: &mut String) {
    for child in input.children.borrow().iter() {
        walk(child.borrow(), result);
    }
}

fn walk(input: &Handle, result: &mut String) {
    println!(">>> {:?}", input.data);
    match input.data {
        NodeData::Text { ref contents } => {
            let text = contents.borrow().to_string();
            println!("Text: {:?}", text);
            result.push_str(&text);
        }
        NodeData::Element { ref name, .. } => {
            let tag = name.local.to_string();
            println!("Tag: {:?}", tag);
            match tag.as_ref() {
                "html" | "head" | "body" => {
                    println!("\tIgnored tag");
                    go_children(input, result);
                }
                "p" => {
                    println!("\tParagraph");
                    result.push_str("\n\n");
                    go_children(input, result);
                }
                "span" => {
                    println!("\tSpan");
                    if let NodeData::Element { ref attrs, .. } = input.data {
                        let attrs = attrs.borrow();
                        let classes = attrs
                            .iter()
                            .find(|attr| attr.name.local.to_string() == "class");
                        if let Some(class) = classes {
                            let classes = class.value.to_string();
                            if !classes.contains("invisible") {
                                go_children(input, result);
                                if classes.contains("ellipsis") {
                                    result.push_str("...");
                                }
                            }
                        } else {
                            go_children(input, result);
                        }
                    }
                }
                "a" => {
                    println!("\tAnchor");
                    if let NodeData::Element { ref attrs, .. } = input.data {
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
                                    result.push_str(&href.value.to_string());
                                    result.push_str("][");
                                    go_children(input, result);
                                    result.push_str("]]");
                                } else {
                                    go_children(input, result);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {
            go_children(input, result);
        }
    }
}

fn main() {
    let source = String::from(
        r#"<p>Today I finally moved with my contact and calendar management into the terminal with <a href="https://fosstodon.org/tags/vdirsyncer" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>vdirsyncer</span></a> <a href="https://fosstodon.org/tags/khal" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>khal</span></a> and <a href="https://fosstodon.org/tags/khard" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>khard</span></a>.</p><p>Thank you <span class="h-card"><a href="https://fosstodon.org/@hund" class="u-url mention" rel="nofollow noopener noreferrer" target="_blank">@<span>hund</span></a></span> for your great post: <a href="https://hund.tty1.se/2020/08/12/how-to-sync-and-manage-your-caldav-and-carddav-via-the-terminal.html" rel="nofollow noopener noreferrer" target="_blank"><span class="invisible">https://</span><span class="ellipsis">hund.tty1.se/2020/08/12/how-to</span><span class="invisible">-sync-and-manage-your-caldav-and-carddav-via-the-terminal.html</span></a></p><p><a href="https://fosstodon.org/tags/carddav" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>carddav</span></a> <a href="https://fosstodon.org/tags/caldav" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>caldav</span></a> <a href="https://fosstodon.org/tags/terminal" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>terminal</span></a></p>"#,
    );
    println!("Source: {}", &source);
    println!("---------------------------------");

    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut source.as_bytes())
        .unwrap();
    let mut result = String::new();
    walk(&dom.document, &mut result);
    println!("---------------------------------");
    let options = Options::new(70)
        .initial_indent("  ")
        .subsequent_indent("  ");
    println!("{}", fill(&result.trim(), &options));
}
