pub fn build_banner(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.is_empty() {
        return None; 
    }

    let max_len_line = lines.iter().map(|line| line.len()).max().unwrap();
    let border = format!("+{}+", "-".repeat(max_len_line + 2));
    
    let content: Vec<String> = lines.iter()
        .map(|line| format!("| {:<width$} |", line, width = max_len_line))
        .collect();

    let banner = vec![border.clone(), content.join("\n"), border];
    
    Some(banner.join("\n"))
}