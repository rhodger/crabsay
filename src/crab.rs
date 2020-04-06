pub fn format(text: &str) -> String {
    let mut h_border = String::new();
    for i in 0..text.chars().count() {
        h_border.push('=');
    }
    format!("{}\n{}\n{}", h_border, text, h_border)
}
