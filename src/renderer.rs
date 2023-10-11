use std::error::Error;
use std::fs;
use headless_chrome::Browser;
use tempfile::{NamedTempFile, tempfile};
#[derive(Default)]
pub struct RendererConfig {}

pub struct Renderer {
    browser: Browser,
    config: RendererConfig
}

impl Renderer {
    pub fn new(config: RendererConfig) -> Result<Renderer, Box<dyn Error>> {
        println!("Starting browser env..");
        let browser = Browser::default()?;
        Ok(Renderer {
            browser,
            config
        })
    }

    pub fn render(&self, raw_html_content: String) -> Result<Vec<u8>, Box<dyn Error>> {
        println!("Rendering pdf..");
        let mut file = tempfile::Builder::new()
            .suffix(".html")
            .tempfile()?;
        fs::write(file.path(), raw_html_content)?;
        //write!(file, "{raw_html_content}")?;
        let tab= self.browser.new_tab()?;
        println!("{:?}", file.path());
        let url = format!("file://{}", file.path().display());
        tab.navigate_to(&url)?
            .wait_until_navigated()?;
        let data = tab.print_to_pdf(None)?;
        Ok(data)
    }
}
