use crate::models::StringArgs;

pub fn colorize_letters(strs: Vec<String>, StringArgs { letters }: StringArgs) -> Vec<String> {
    strs.into_iter()
        .map(|str| {
            str.chars()
                .into_iter()
                .map(|c| {
                    if letters.contains(c) {
                        colorize_letter(c.to_string())
                    } else {
                        c.to_string()
                    }
                })
                .collect::<String>()
        })
        .collect()
}

fn colorize_letter(c: String) -> String {
    let mut s = "".to_owned();
    s.push_str("\x1b[31m");
    s.push_str(&c);
    s.push_str("\x1b[0m");
    s
}
