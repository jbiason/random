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
#[derive(Debug)]
enum NodeType {
    /// The root element; produces nothing, but has the base content.
    Root,
    /// A text block. Contains the text itself.
    Text(String),
    /// A line break
    LineBreak,
    // /// A link to somewhere. Contains the link.
    // Link(String),
    // /// Italics
    // Italic,
    // /// Code block
    // Code,
    // /// A block with an ellipsis at the end
    // Ellipsis,
}

#[derive(Debug)]
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
            children: Vec::new(),
        }
    }

    /// Build a linebreak node
    fn line_break() -> Self {
        Self {
            r#type: NodeType::LineBreak,
            children: Vec::new(),
        }
    }

    // /// Build a link node
    // fn link(href: &str) -> Self {
    //     Self {
    //         r#type: NodeType::Link(href.into()),
    //         children: Vec::new(),
    //     }
    // }

    // /// Build a ellipsis node
    // fn ellipsis() -> Self {
    //     Self {
    //         r#type: NodeType::Ellipsis,
    //         children: Vec::new(),
    //     }
    // }

    /// Add a child node to this node
    fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }
}

// Handle functions can return a three state result:
// 1. Do not process the children of the current Handle
// 2. Process the children and add to the same parent
// 3. Use the new Node as parent for future children.

/// Result of the handling functions
enum HandleResult {
    /// Stop processing, don't continue generating nodes
    Stop,
    /// Follow the children, but don't add any nodes in the current level
    Follow,
    // /// Produce a new node, but don't attach any children to it
    // AddAndStay(Node),
    /// Assume a new parent node
    AddAndAdopt(Node),
}

/// Handle a simple block of text
fn handle_text(node: &mut Node, contents: &RefCell<StrTendril>) -> HandleResult {
    let text = contents.borrow().to_string();
    node.add_child(Node::text(&text));
    HandleResult::Stop
}

/// Handle an incoming line break
fn handle_line_break() -> HandleResult {
    let line_break = Node::line_break();
    HandleResult::AddAndAdopt(line_break)
}

/// Process the span content
fn handle_span(attrs: &RefCell<Vec<Attribute>>) -> HandleResult {
    let attrs = attrs.borrow();
    let classes_attr = attrs
        .iter()
        .find(|attr| attr.name.local.to_string() == "class");
    match classes_attr {
        Some(classes) => {
            if classes.value.contains("invisible") {
                HandleResult::Stop
            } else {
                HandleResult::Follow
            }
        }
        None => HandleResult::Follow,
    }
}

// fn handle_anchor(node: &mut Node, attrs: &RefCell<Vec<Attribute>>) -> HandleResult {
//     let attrs = attrs.borrow();
//     let rels = attrs
//         .iter()
//         .find(|attr| attr.name.local.to_string() == "rel");
//     let hrefs = attrs
//         .iter()
//         .find(|attr| attr.name.local.to_string() == "href");
//     match (rels, hrefs) {
//         (Some(rel), Some(href)) => {
//             if !rel.value.to_string().contains("tag") {
//                 let new_node = Node::link(&href.value);
//                 node.add_child(new_node);
//                 HandleResult::NewNode(new_node)
//             } else {
//                 HandleResult::Keep
//             }
//         }
//         _ => HandleResult::Stop,
//     }
// }

fn walk(input: &Handle, parent: &mut Node) {
    // println!(">>> {:?}", input.data);
    let element = match input.data {
        NodeData::Text { ref contents } => handle_text(parent, contents),
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let tag = name.local.to_string();
            match tag.as_ref() {
                "html" | "head" | "body" => HandleResult::Follow,
                "p" => handle_line_break(),
                "span" => handle_span(attrs),
                // "a" => handle_anchor(parent, attrs),
                _ => HandleResult::Stop,
            }
        }
        _ => HandleResult::Follow, // if we can't deal with it, just keep going
    };

    match element {
        HandleResult::Stop => {}
        HandleResult::Follow => {
            for child in input.children.borrow().iter() {
                walk(child.borrow(), parent);
            }
        }
        // HandleResult::AddAndStay(new_node) => {
        //     parent.add_child(new_node);
        //     for child in input.children.borrow().iter() {
        //         walk(child.borrow(), parent);
        //     }
        // }
        HandleResult::AddAndAdopt(mut new_node) => {
            for child in input.children.borrow().iter() {
                walk(child.borrow(), &mut new_node);
            }
            parent.add_child(new_node);
        }
    }
}

fn build_nodes(text: &str) {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut text.as_bytes())
        .unwrap();
    let mut tree = Node::root();
    walk(&dom.document, &mut tree);
    println!("Tree: {:?}", tree);
}

fn main() {
    let example_1 = String::from(r#"<p>A simple text component</p>"#);
    build_nodes(&example_1);

    let example_2 = String::from(
        r#"<p><span class="invisible">THis is not visible</span><span class="ellipsis">but this is</span></p>"#,
    );
    build_nodes(&example_2);

    // let example_1 = String::from(
    //     r#"<p>Today I finally moved with my contact and calendar management into the terminal with <a href="https://fosstodon.org/tags/vdirsyncer" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>vdirsyncer</span></a> <a href="https://fosstodon.org/tags/khal" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>khal</span></a> and <a href="https://fosstodon.org/tags/khard" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>khard</span></a>.</p><p>Thank you <span class="h-card"><a href="https://fosstodon.org/@hund" class="u-url mention" rel="nofollow noopener noreferrer" target="_blank">@<span>hund</span></a></span> for your great post: <a href="https://hund.tty1.se/2020/08/12/how-to-sync-and-manage-your-caldav-and-carddav-via-the-terminal.html" rel="nofollow noopener noreferrer" target="_blank"><span class="invisible">https://</span><span class="ellipsis">hund.tty1.se/2020/08/12/how-to</span><span class="invisible">-sync-and-manage-your-caldav-and-carddav-via-the-terminal.html</span></a></p><p><a href="https://fosstodon.org/tags/carddav" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>carddav</span></a> <a href="https://fosstodon.org/tags/caldav" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>caldav</span></a> <a href="https://fosstodon.org/tags/terminal" class="mention hashtag" rel="tag nofollow noopener noreferrer" target="_blank">#<span>terminal</span></a></p>"#,
    // );
    // println!("Source: {}", &example_1);
    // println!("---------------------------------");

    // let dom = parse_document(RcDom::default(), Default::default())
    //     .from_utf8()
    //     .read_from(&mut example_1.as_bytes())
    //     .unwrap();
    // let mut tree = Node::root();
    // walk(&dom.document, &mut tree);
}
