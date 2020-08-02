use pulldown_cmark::{Event, Tag};

pub fn heading_1_to_2(event: Event) -> Event {
    match event {
        Event::Start(Tag::Heading(1)) => Event::Start(Tag::Heading(2)),
        Event::End(Tag::Heading(1)) => Event::End(Tag::Heading(2)),
        _ => event,
    }
}
