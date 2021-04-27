use html5ever::parse_document;
use html5ever::tendril::StrTendril;
use html5ever::tendril::TendrilSink;
use markup5ever::interface::Attribute;
use markup5ever_rcdom::Handle;
use markup5ever_rcdom::NodeData;
use markup5ever_rcdom::RcDom;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::default::Default;
use textwrap::fill;
use textwrap::NoHyphenation;
use textwrap::Options;

// This go_children/walk is stupid, but I shot myself in the foot by adding
// things after the children, link on links.
//
// So, I'm rethinking this, and I'll, basically redo the same thing Tendril is
// doing by building a tree of elements.
//
// So, say, if we have
//
// <p>Text text<a href="link"><span visible>the link</span><span
// invisible>other text</span></a>, <i>for italics</i>, <pre>is for code</pre>
// </p>
//
// That would build
//                  root --------------------------\
//                 / |  \                           Code()
//   Text(Text text) |   ------ Italic()             |
//                Link(link)        |             Text(is for code)
//                   |        Text(for italics)
//              Text(the link)
//
// Tree things to do, then:
// 1. Walk the DOM tree and build the text tree.
// 2. Tree elements could return if the DOM three should continue processing or
//    ignore incoming children (that would cut the "invisible span" processing,
//    for example).
// 3. Build a walker for the new tree, to produce the final text. And, on that,
//    we could work on the text wrap, 'cause there are elements that can't be
//    wrapped (for example, Links)

/// Nodes in the text tree
enum NodeType {
    /// The root element; produces nothing, but has the base content.
    Root,
    /// A text block. Contains the text itself.
    Text(String),
    /// A link to somewhere. Contains the link.
    Link(String),
    /// Italics
    Italic,
    /// Code block
    Code,
    /// A line break
    LineBreak,
}

struct Node {
    r#type: NodeType,
    children: Vec<Node>,
}

impl Node {
    /// Build the root node
    fn root() -> Self {
        Self {
            r#type: NodeType::Root,
            children: Vec::new(),
        }
    }

    /// Build a text node
    fn text(text: &str) -> Self {
        Self {
            r#type: NodeType::Text(text.into()),
            children: Vec::new(), // XXX text nodes will never have children
        }
    }

    /// Build a link node
    fn link(href: &str) -> Self {
        Self {
            r#type: NodeType::Link(href.into()),
            children: Vec::new(),
        }
    }

    /// Build a linebreak node
    fn line_break() -> Self {
        Self {
            r#type: NodeType::LineBreak,
            children: Vec::new(), // XXX linebreaks will never have children
        }
    }

    /// Add a child node to this node
    fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }
}

fn handle_text(node: &mut Node, contents: &RefCell<StrTendril>) -> bool {
    let text = contents.borrow().to_string();
    node.add_child(Node::text(&text));
    true
}

fn handle_line_break(node: &mut Node) -> bool {
    node.add_child(Node::line_break());
    true
}

fn handle_span(node: &mut Node, attrs: &RefCell<Vec<Attribute>>) -> bool {
    let attrs = attrs.borrow();
    let classes = attrs
        .iter()
        .find(|attr| attr.name.local.to_string() == "class");
    if let Some(class) = classes {
        let classes = class.value.to_string();
        // just keep going if not invisible
        !classes.contains("invisible")

        // if !classes.contains("invisible") {
        //     true
        //     if classes.contains("ellipsis") {
        //         result.push_str("...");
        //     }
        // }
    } else {
        // with no classes, we consider the element visible and just keep
        // processing the list.
        true
    }
}

fn walk(input: &Handle, parent: &mut Node) {
    println!(">>> {:?}", input.data);
    let process_children = match input.data {
        NodeData::Text { ref contents } => handle_text(parent, contents),
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let tag = name.local.to_string();
            println!("Tag: {:?}", tag);
            match tag.as_ref() {
                "html" | "head" | "body" => true, // just keep going
                "p" => handle_line_break(parent),
                "span" => handle_span(parent, attrs),
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
                _ => false,
            }
        }
        _ => true, // if we can't deal with it, just keep going
    };

    if process_children {
        for child in input.children.borrow().iter() {
            walk(child.borrow(), parent);
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
    let mut tree = Node::root();
    walk(&dom.document, &mut result);
    println!("---------------------------------");
    let options = Options::new(70)
        .initial_indent("  ")
        .subsequent_indent("  ")
        .splitter(NoHyphenation);
    println!("{}", fill(&result.trim(), &options));
}
