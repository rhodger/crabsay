//! This file handles all of the complicated logic.
//!
//! In theory only `format` should be called, and it in turn calls each other
//! function as and when is necessary.

use yansi::Paint;

// ASCII crab ASCII crab
const CRAB: &str= r#"
    ( )   @ @    ()
     \  __| |__  /
      -/   "   \-
     /-|       |-\
    / /-\     /-\ \
     / /-`---'-\ \  
      /         \"#;

/// Takes the input string and the desired maximum line length.
///
/// The line length was originally hard-coded, but now is passed by `main`
/// depending on user input (or lack thereof)
pub fn format(text: &str, width: u8) -> String {
    if text.chars().count() < 3 {
        return String::new();
    }

    let mut top_border = String::new();
    let mut bot_border = String::new();
    let mut content;
    
    if text.chars().count() as u8 > width - 2 {
        bot_border.push('-');
        bot_border.push(' ');
        for i in 0..width {
            top_border.push('-');
            if i > 1 {
                bot_border.push('-');
            }
        }
        
        let text = wrap(&text, width - 2);
        let mut lines = text.split("\n");
        content = format_line_width(
          &lines.next().expect("INVALID ARGUMENT STRING").to_string(),
          width);

        for line in lines {
            let mut formatted_line = line.to_string();
            
            formatted_line = format_line_width(&formatted_line, width);

            content = format!("{}\n{}", content, formatted_line);
        }
    } else {
        let width = text.chars().count() as u8 + 2;
        bot_border.push('-');
        bot_border.push(' ');
        for i in 0..width {
            top_border.push('-');
            if i > 1 {
                bot_border.push('-');
            }
        }
        content = format_line_width(text, width);
    }

    // An admittedly untidy way of combining the individual components of the
    // final output.
    format!(
        "{}\n{}\n{}\n\\/\n \\\n{}",
        top_border,
        content,
        bot_border,
        Paint::red(CRAB).bold()
    )
}

/// Pads the line out to the necessary length and adds horizontal borders.
fn format_line_width(line: &str, width: u8) -> String {
    let mut formatted_line = line.to_string();

    for _ in 0..(width - line.chars().count() as u8 - 2) {
        formatted_line.push(' ');
    }

    format!("|{}|", formatted_line)
}

/// Adds linebreaks into the text at appropriate positions
///
/// Linebreaks are put into positions to ensure that each line is no longer
/// than the defined maximum length (even once borders are added for the speech
/// box) and that words are never broken. If a word must be broken for this to
/// work (i.e its longer than can fit onto a single line) then the program fails
/// with an error message - this is functionality to be potentially implemented
/// at a later date.
fn wrap(text: &str, width: u8) -> String {
    let mut wrapped = String::new();
    let mut line = String::new();
    let mut words = text.split(" ");
    line = words.next().expect("INVALID STRING ARGUMENT").to_string();

    for i in words {
        if (i.chars().count() + 1) + line.chars().count() <= width as usize {
            line = format!("{} {}", line, i);
        } else {
            wrapped = format!("{}{}\n", wrapped, line);
            line = i.to_string();
        }
    }
    wrapped = format!("{}{}", wrapped, line);

    return wrapped;
}
