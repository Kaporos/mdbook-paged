mod renderer;

use std::error::Error;
use markdown::{CompileOptions, Options};
use mdbook::{renderer::RenderContext, BookItem};
use std::{fs, io};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PdfConfig {
    inject_html: bool
}

const PRINT_HOOK: &'static str = r#"
<script>
document.location.href = "./output.pdf"
</script>
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    let value: Option<PdfConfig> = ctx.config.get_deserialized_opt("output.pdf")?;
    println!("{:?}", value);
    println!("{:?}", ctx.config);
    let mut pdf_path = ctx.destination.join("output.pdf");

    if let Some(value) = value {
        if value.inject_html {
            let html_print_file =  ctx.destination.parent().unwrap().join("html").join("print.html");
            fs::write(html_print_file, PRINT_HOOK)?;
            pdf_path = ctx.destination.parent().unwrap().join("html").join("output.pdf");
            fs::remove_dir_all(ctx.destination)?;
        }
    }

    println!("{}", PRINT_HOOK);
    let mut content_html = String::new();
    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            println!("Processing {}..", ch.name);
            let html = markdown::to_html_with_options(
                &ch.content,
                &Options {
                    compile: CompileOptions {
                        allow_dangerous_html: true,
                        allow_dangerous_protocol: true,
                        ..CompileOptions::default()
                    },
                    ..Options::default()
                },
            )
            .unwrap();
            content_html.push_str(&html);
        }
    };
    let my_renderer = renderer::Renderer::new(renderer::RendererConfig::default())?;
    fs::write(pdf_path, my_renderer.render(content_html)?)?;
    Ok(())
}
