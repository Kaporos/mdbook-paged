mod renderer;
mod process_markdown;

use std::error::Error;
use mdbook::{renderer::RenderContext, BookItem};
use std::{env, fs, io};
use serde_derive::{Deserialize, Serialize};
use crate::process_markdown::Processor;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PdfConfig {
    inject_html: bool,
    always: bool
}

const PRINT_HOOK: &'static str = r#"
<script>
document.location.href = "./output.pdf"
</script>
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    let value: Option<PdfConfig> = ctx.config.get_deserialized_opt("output.paged")?;
    let mut pdf_path = ctx.destination.join("output.pdf");

    if let Some(value) = value {
        if env::var("GEN_PDF").is_err() && !value.always{
            println!("Skipping pdf..");
            return Ok(())
        }
        if value.inject_html {
            let html_print_file =  ctx.destination.parent().unwrap().join("html").join("print.html");
            fs::write(html_print_file, PRINT_HOOK)?;
            pdf_path = ctx.destination.parent().unwrap().join("html").join("output.pdf");
            fs::remove_dir_all(ctx.destination)?;
        }
    }

    let mut content_html = String::new();

    let processor = Processor::new(ctx.root.join("src"));

    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            println!("Processing {}..", ch.name);
            let html = processor.process_chapter(ch)?;
            content_html.push_str(&html);
        }
    };
    let my_renderer = renderer::Renderer::new(renderer::RendererConfig::default())?;
    fs::write(pdf_path, my_renderer.render(content_html)?)?;
    Ok(())
}
