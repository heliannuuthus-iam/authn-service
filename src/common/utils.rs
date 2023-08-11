pub fn desensitize_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() == 2 {
        let username = &parts[0];
        let domain = &parts[1];
        if username.len() > 4 {
            format!(
                "{}@{}",
                format!("{}****{}", &username[..2], &username[username.len() - 2..]),
                domain
            )
        } else {
            format!(
                "{}@{}",
                format!("{}****{}", &username[..1], &username[username.len() - 1..]),
                domain
            )
        }
    } else {
        email.to_string()
    }
}

pub fn desensitize_text(name: &str) -> String {
    let chars = name;
    println!("length: {}", chars.len());
    if chars.len() == 2 {
        format!("{}*", &name[..1])
    } else if chars.len() > 2 {
        format!("{}*{}", &name[..1], &name[name.len() - 1..])
    } else {
        name.to_string()
    }
}
