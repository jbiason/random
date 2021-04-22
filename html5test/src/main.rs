use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;
use std::default::Default;

fn main() {
    let source = String::from(
        r#"<p>Today I finally moved with my contact and calendar management into
  the terminal with <a href="https://fosstodon.org/tags/vdirsyncer"
  class="mention hashtag" rel="tag nofollow noopener
  noreferrer" target="_blank">#<span>vdirsyncer</span></a>
  <a href="https://fosstodon.org/tags/khal" class="mention
  hashtag" rel="tag nofollow noopener noreferrer"
  target="_blank">#<span>khal</span></a> and <a
  href="https://fosstodon.org/tags/khard" class="mention
  hashtag" rel="tag nofollow noopener noreferrer"
  target="_blank">#<span>khard</span></a>.</p><p>Thank you
  <span class="h-card"><a href="https://fosstodon.org/@hund"
  class="u-url mention" rel="nofollow noopener noreferrer"
  target="_blank">@<span>hund</span></a></span> for your
  great post: <a href="https://hund.tty1.se/2020/08/12/how-
  to-sync-and-manage-your-caldav-and-carddav-via-the-
  terminal.html" rel="nofollow noopener noreferrer"
  target="_blank"><span class="invisible">https://</span><span
  class="ellipsis">hund.tty1.se/2020/08/12/how-to</span><span
  class="invisible">-sync-and-manage-your-caldav-and-
  carddav-via-the-terminal.html</span></a></p><p><a
  href="https://fosstodon.org/tags/carddav"
  class="mention hashtag" rel="tag nofollow noopener
  noreferrer" target="_blank">#<span>carddav</span></a>
  <a href="https://fosstodon.org/tags/caldav"
  class="mention hashtag" rel="tag nofollow noopener
  noreferrer" target="_blank">#<span>caldav</span></a> <a
  href="https://fosstodon.org/tags/terminal" class="mention
  hashtag" rel="tag nofollow noopener noreferrer"
  target="_blank">#<span>terminal</span></a></p>"#,
    );
    println!("Source: {}", &source);

    let _dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut source.as_bytes())
        .unwrap();
}
