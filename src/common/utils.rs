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
    match name.len().cmp(&2) {
        Ordering::Equal => format!("{}*", &name[..1]),
        Ordering::Less => name.to_string(),
        Ordering::Greater => format!("{}*{}", &name[..1], &name[name.len() - 1..]),
    }
}
