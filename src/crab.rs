pub fn format(text: &str, width: u8) -> String {
    let mut h_border = String::new();

    for _ in 0..width {
        h_border.push('=');
    }

    format!("{}\n#{}#\n{}", h_border, wrap(text, (width - 2)), h_border)
}

fn wrap(text: &str, width: u8) -> String {
    let mut wrapped = String::new();
    let mut line = String::new();

    for i in text.split(" ") {
        if (i.chars().count() + 1) + line.chars().count() <= width as usize {
            line = format!("{} {}", line, i);
        } else {
            wrapped = format!("{}\n{}", wrapped, line);
            line = i.to_string();
        }
    }
    wrapped = format!("{}\n{}", wrapped, line);

    return wrapped;
}
