use std::error::Error;
use std::path::{Path, PathBuf};
use mdbook::book::Chapter;
use pulldown_cmark::{CowStr, Event, Tag};

pub struct Processor {
    root: PathBuf
}

impl Processor {
    pub fn new(root: PathBuf) -> Processor {
        Processor {
            root
        }
    }

    pub fn process_chapter(&self, ch: &Chapter) -> Result<String, Box<dyn Error>>{
        let chapter_location = self.root.join(ch.path.clone().unwrap());
        let relative_dir = chapter_location.parent().unwrap();

        let parser = pulldown_cmark::Parser::new(&ch.content);

        let parser = parser.map(|event| match event {
            Event::Start(tag) => {
                Event::Start(match tag.clone() {
                    Tag::Image(ltype, source, title) => {
                        Tag::Image(ltype, self.compute_image_source(source, relative_dir).into(), title)
                    }
                    _ => tag
                })
            }
           _ => event
        });
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);


        Ok(html_output)
    }

    fn compute_image_source<'a>(&self, mut old_source: CowStr<'a>, relative_dir: &Path) -> String {
        let prefix = "file://";
        let mut new_source = String::from(prefix);
        if !old_source.starts_with("/"){
            new_source.push_str(relative_dir.to_str().unwrap());
            new_source.push_str("/");
        }
        if (old_source.starts_with("./")) {
            old_source = old_source.chars().skip(2).collect::<String>().into();
        }
        new_source.push_str(&*old_source);
        println!("Old: {old_source} - new: {new_source}");
        new_source

    }
}

