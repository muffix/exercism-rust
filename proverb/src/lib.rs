pub fn build_proverb(list: Vec<&str>) -> String {
    if list.len() == 0 {
        return String::new();;
    }

    let mut verses = Vec::new();

    for (i, object) in list.iter().enumerate() {
        if i == list.len() - 1 {
            break;
        }

        verses.push(format!(
            "For want of a {} the {} was lost.",
            object,
            list[i + 1]
        ));
    }

    verses.push(format!("And all for the want of a {}.", list[0]));

    return verses.join("\n");
}
