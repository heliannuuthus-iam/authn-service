use std::cmp::Ordering;

pub fn desensitize_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() == 2 {
        let username = &parts[0];
        let domain = &parts[1];
        if username.len() > 4 {
            format!(
                "{}@{}",
                format_args!("{}****{}", &username[..2], &username[username.len() - 2..]),
                domain
            )
        } else {
            format!(
                "{}@{}",
                format_args!("{}****{}", &username[..1], &username[username.len() - 1..]),
                domain
            )
        }
    } else {
        email.to_string()
    }
}

pub fn desensitize_text(name: &str) -> String {
    let mut left = 1;
    while !name.is_char_boundary(left) {
        left += 1
    }
    match name.chars().count().cmp(&2) {
        Ordering::Equal => format!("{}*", &name[..left]),
        Ordering::Less => name.to_string(),
        Ordering::Greater => {
            let mut right = name.len() - 1;
            while !name.is_char_boundary(right) {
                right -= 1
            }
            format!("{}*{}", &name[..left], &name[right..])
        }
    }
}
