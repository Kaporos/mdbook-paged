use std::error::Error;
use std::path::{Component, Path, PathBuf};
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
                        let absolute_path = self.path_resolve(source, relative_dir);
                        let mut file_path = String::from("file://");
                        file_path.push_str(&absolute_path);
                        Tag::Image(ltype, file_path.into(), title)
                    },
                    //Tag::Link(ltype, source, title) => {
                    //    Tag::Link(ltype, self.path_resolve(source, relative_dir).into(), title)
                    //},
                    _ => {
                        tag
                    }
                })
            }
           _ => event
        });
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);


        Ok(html_output)
    }

    fn path_resolve<'a>(&self, mut old_source: CowStr<'a>, relative_dir: &Path) -> String {
        let old_copy = old_source.clone();
        let mut old_source: &str = &*old_source;
        if old_source.starts_with("/") {
            old_source = &old_source[1..old_source.len()]
        }
        let old_path = Path::new(&*old_source);
        let old_path_absolute = normalize_path(relative_dir.join(old_path));
        if std::fs::metadata(old_path_absolute.clone()).is_ok() {
            let path = old_path_absolute.to_str().unwrap();
            return path.into();
        }
       return old_copy.into_string();
    }
}


//Copy pasted function from Cargo's source code.
//This function computes paths like /root/theo/../toto to /root/toto
// https://github.com/rust-lang/cargo/blob/fede83ccf973457de319ba6fa0e36ead454d2e20/src/cargo/util/paths.rs#L61
fn normalize_path(path: PathBuf) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}