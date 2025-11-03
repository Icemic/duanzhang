use std::mem::transmute;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use hb_subset::*;
use unicode_blocks::UnicodeBlock;

use crate::cli::Preset;

const RECOMMENDED: [UnicodeBlock; 15] = [
    unicode_blocks::ARROWS,
    unicode_blocks::BASIC_LATIN,
    unicode_blocks::BLOCK_ELEMENTS,
    unicode_blocks::BOPOMOFO,
    unicode_blocks::CJK_COMPATIBILITY_FORMS,
    unicode_blocks::CJK_SYMBOLS_AND_PUNCTUATION,
    unicode_blocks::CJK_UNIFIED_IDEOGRAPHS,
    unicode_blocks::GENERAL_PUNCTUATION,
    unicode_blocks::HALFWIDTH_AND_FULLWIDTH_FORMS,
    unicode_blocks::HIRAGANA,
    unicode_blocks::KATAKANA,
    unicode_blocks::LATIN_1_SUPPLEMENT,
    unicode_blocks::VERTICAL_FORMS,
    unicode_blocks::HANGUL_JAMO,
    unicode_blocks::HANGUL_SYLLABLES,
];
const PUNCTUATION: [UnicodeBlock; 5] = [
    unicode_blocks::GENERAL_PUNCTUATION,
    unicode_blocks::CJK_SYMBOLS_AND_PUNCTUATION,
    unicode_blocks::CJK_COMPATIBILITY_FORMS,
    unicode_blocks::HALFWIDTH_AND_FULLWIDTH_FORMS,
    unicode_blocks::VERTICAL_FORMS,
];
const SYMBOLS: [UnicodeBlock; 2] = [unicode_blocks::ARROWS, unicode_blocks::BLOCK_ELEMENTS];
const BOPOMOFO: [UnicodeBlock; 1] = [unicode_blocks::BOPOMOFO];
const CJKUNIFIED_IDEOGRAPHS: [UnicodeBlock; 1] = [unicode_blocks::CJK_UNIFIED_IDEOGRAPHS];
const KANA: [UnicodeBlock; 2] = [unicode_blocks::HIRAGANA, unicode_blocks::KATAKANA];
const HANGUL: [UnicodeBlock; 2] = [
    unicode_blocks::HANGUL_JAMO,
    unicode_blocks::HANGUL_SYLLABLES,
];
const ASCII: [UnicodeBlock; 2] = [
    unicode_blocks::BASIC_LATIN,
    unicode_blocks::LATIN_1_SUPPLEMENT,
];

/// Subset a font file
pub fn subset(
    file: &str,
    output: &Option<String>,
    charset: &Vec<String>,
    preset: &Vec<Preset>,
) -> Result<()> {
    let font_data = std::fs::read(file).unwrap();
    let font = Blob::from_bytes(&font_data)?;
    let font = FontFace::new(font)?;

    // create subset input and append characters
    let mut subset = SubsetInput::new()?;

    // add characters from external charset file
    {
        let mut chars = String::new();

        for charset in charset {
            match std::fs::read_to_string(charset) {
                Ok(c) => chars.push_str(&c),
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Error reading charset file: {}",
                        e.to_string()
                    ));
                }
            }
        }

        for c in chars.chars() {
            subset.unicode_set().insert(c);
        }
    }

    // add characters from preset
    {
        let mut blocks_list = Vec::new();
        for p in preset {
            match p {
                Preset::Recommended => blocks_list.extend(RECOMMENDED),
                Preset::Punctuation => blocks_list.extend(PUNCTUATION),
                Preset::Symbols => blocks_list.extend(SYMBOLS),
                Preset::Bopomofo => blocks_list.extend(BOPOMOFO),
                Preset::CJKUnifiedIdeographs => blocks_list.extend(CJKUNIFIED_IDEOGRAPHS),
                Preset::Kana => blocks_list.extend(KANA),
                Preset::Hangul => blocks_list.extend(HANGUL),
                Preset::Ascii => blocks_list.extend(ASCII),
            }
        }

        for block in blocks_list {
            for c in block.start()..block.end() + 1 {
                subset.unicode_set().insert(unsafe { transmute(c) });
            }
        }
    }

    // create subset font based on subset input
    let new_font = subset.subset_font(&font)?;

    // acquire output path:
    // 1. if output path is provided, use it
    // 2. if output path is not provided, generate a new file name based on the input file name
    let output_path = if let Some(output) = output {
        let p = PathBuf::from_str(output)?;
        if !p.is_file() {
            return Err(anyhow::anyhow!("Output path is not a file"));
        }
        p
    } else {
        let mut p = PathBuf::from_str(file)?;
        p.set_file_name(format!(
            "{}-subset.{}",
            p.file_stem().unwrap().to_str().unwrap(),
            p.extension().unwrap().to_str().unwrap()
        ));
        p
    };

    // write new font to output path
    std::fs::write(output_path, &*new_font.underlying_blob())?;

    Ok(())
}
