pub fn format(text: &str, width: u8) -> String {
    let mut h_border = String::new();
    let mut content = String::new();

    for _ in 0..width {
        h_border.push('*');
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

    format!("{}\n{}\n{}", h_border, content, h_border)
}

fn format_line_width(line: &str, width: u8) -> String {
    let mut formatted_line = line.to_string();

    for _ in 0..(width - line.chars().count() as u8 - 2) {
        formatted_line.push(' ');
    }

    format!("*{}*", formatted_line)
}

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
