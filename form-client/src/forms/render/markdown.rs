use dioxus::prelude::*;

#[component]
pub fn MarkdownDescription(markdown: String) -> Element {
    let blocks = parse_markdown(&markdown);

    rsx! {
        div { class: "markdown",
            for (index, block) in blocks.iter().enumerate() {
                match block {
                    MarkdownBlock::Heading { level, text } => rsx! {
                        if *level <= 2 {
                            h2 { key: "{index}", "{text}" }
                        } else {
                            h3 { key: "{index}", "{text}" }
                        }
                    },
                    MarkdownBlock::Paragraph(text) => rsx! {
                        p { key: "{index}", "{text}" }
                    },
                    MarkdownBlock::List(items) => rsx! {
                        ul { key: "{index}",
                            for (item_index, item) in items.iter().enumerate() {
                                li { key: "{item_index}", "{item}" }
                            }
                        }
                    },
                    MarkdownBlock::Quote(text) => rsx! {
                        blockquote { key: "{index}",
                            p { "{text}" }
                        }
                    },
                }
            }
        }
    }
}

enum MarkdownBlock {
    Heading { level: usize, text: String },
    Paragraph(String),
    List(Vec<String>),
    Quote(String),
}

fn parse_markdown(markdown: &str) -> Vec<MarkdownBlock> {
    let mut blocks = Vec::new();
    let mut paragraph = Vec::new();
    let mut list = Vec::new();
    let mut quote = Vec::new();

    for line in markdown.lines() {
        let line = line.trim();

        if line.is_empty() {
            flush_markdown_buffers(&mut blocks, &mut paragraph, &mut list, &mut quote);
            continue;
        }

        if let Some(item) = line.strip_prefix("- ") {
            flush_paragraph(&mut blocks, &mut paragraph);
            flush_quote(&mut blocks, &mut quote);
            list.push(item.trim().to_string());
            continue;
        }

        if let Some(text) = line.strip_prefix("> ") {
            flush_paragraph(&mut blocks, &mut paragraph);
            flush_list(&mut blocks, &mut list);
            quote.push(text.trim().to_string());
            continue;
        }

        if let Some((level, text)) = parse_heading(line) {
            flush_markdown_buffers(&mut blocks, &mut paragraph, &mut list, &mut quote);
            blocks.push(MarkdownBlock::Heading { level, text });
            continue;
        }

        flush_list(&mut blocks, &mut list);
        flush_quote(&mut blocks, &mut quote);
        paragraph.push(line.to_string());
    }

    flush_markdown_buffers(&mut blocks, &mut paragraph, &mut list, &mut quote);
    blocks
}

fn parse_heading(line: &str) -> Option<(usize, String)> {
    let level = line
        .chars()
        .take_while(|character| *character == '#')
        .count();

    if (1..=3).contains(&level) && line.chars().nth(level) == Some(' ') {
        Some((level, line[level + 1..].trim().to_string()))
    } else {
        None
    }
}

fn flush_markdown_buffers(
    blocks: &mut Vec<MarkdownBlock>,
    paragraph: &mut Vec<String>,
    list: &mut Vec<String>,
    quote: &mut Vec<String>,
) {
    flush_paragraph(blocks, paragraph);
    flush_list(blocks, list);
    flush_quote(blocks, quote);
}

fn flush_paragraph(blocks: &mut Vec<MarkdownBlock>, paragraph: &mut Vec<String>) {
    if !paragraph.is_empty() {
        blocks.push(MarkdownBlock::Paragraph(paragraph.join(" ")));
        paragraph.clear();
    }
}

fn flush_list(blocks: &mut Vec<MarkdownBlock>, list: &mut Vec<String>) {
    if !list.is_empty() {
        blocks.push(MarkdownBlock::List(std::mem::take(list)));
    }
}

fn flush_quote(blocks: &mut Vec<MarkdownBlock>, quote: &mut Vec<String>) {
    if !quote.is_empty() {
        blocks.push(MarkdownBlock::Quote(quote.join(" ")));
        quote.clear();
    }
}
