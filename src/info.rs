use std::collections::HashSet;
use std::num::NonZeroU16;

use anyhow::Result;
use ttf_parser::PlatformId;

/// Get font file information
pub fn info(file: &str) -> Result<()> {
    let font_data = std::fs::read(file).unwrap();
    // use first font face
    let face = ttf_parser::Face::parse(&font_data, 0)?;

    println!("Raw Names:\n");

    face.names().into_iter().for_each(|name| {
        if let Some(name_str) = name.to_string() {
            println!("({}, {}) {}", name.language(), name.name_id, name_str);
        }
    });

    println!("\nParsed Names:\n");

    let names = face
        .names()
        .into_iter()
        .filter(|n| n.platform_id == PlatformId::Windows)
        .collect::<Vec<_>>();

    println!(
        "Family Name:      {}",
        names
            .get(1)
            .map(|n| n.to_string())
            .flatten()
            .unwrap_or_default()
    );
    println!(
        "Subfamily Name:   {}",
        names
            .get(2)
            .map(|n| n.to_string())
            .flatten()
            .unwrap_or_default()
    );
    println!(
        "Unique ID:        {}",
        names
            .get(3)
            .map(|n| n.to_string())
            .flatten()
            .unwrap_or_default()
            .escape_default()
    );
    println!(
        "Full Name:        {}",
        names
            .get(4)
            .map(|n| n.to_string())
            .flatten()
            .unwrap_or_default()
            .escape_default()
    );
    println!(
        "Version:          {}",
        names
            .get(5)
            .map(|n| n.to_string())
            .flatten()
            .unwrap_or_default()
            .escape_default()
    );
    println!(
        "Copyright:        {}",
        names
            .get(0)
            .map(|n| n.to_string())
            .flatten()
            .unwrap_or_default()
            .escape_default()
    );

    let mut charsets: HashSet<char> = HashSet::new();
    if let Some(subtable) = face.tables().cmap {
        for subtable in subtable.subtables {
            subtable.codepoints(|codepoint| {
                if let Some(mapping) = subtable.glyph_index(codepoint) {
                    if let Some(_) = NonZeroU16::new(mapping.0) {
                        charsets.insert(char::from_u32(codepoint).unwrap());
                    }
                }
            })
        }
    }

    println!("\nGlyphs Count: {}", charsets.len());

    let mut names = HashSet::new();
    for c in charsets {
        names.insert(unicode_blocks::find_unicode_block(c).unwrap().name());
    }

    let mut names = names.into_iter().collect::<Vec<_>>();
    names.sort();

    println!("\nUnicode Blocks:");

    for name in names {
        println!("{:indent$}{name}", "", indent = 4);
    }

    Ok(())
}
