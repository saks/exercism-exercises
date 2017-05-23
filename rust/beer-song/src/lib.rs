const ENDING: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";

pub fn verse(n: i32) -> String {
    match n {
        0 => String::from(ENDING),
        _ => sign_one(n),
    }
}

fn bottles(n: i32) -> String {
    let number = match n {
        0 => "no more".to_string(),
        _ => n.to_string(),
    };

    let word = match n {
        1 => "bottle",
        _ => "bottles",
    };

    format!("{} {}", number, word)
}

fn sign_one(n: i32) -> String {
    let take = match n {
        1 => "it",
        _ => "one",
    };
    format!("{now_bottles} of beer on the wall, {now_bottles} of beer.\nTake {take} down and pass it around, {later_bottles} of beer on the wall.\n",
            now_bottles = bottles(n),
            later_bottles = bottles(n - 1),
            take = take)
}

pub fn sing(from: i32, to: i32) -> String {
    let mut result = String::new();

    for i in (to..from + 1).rev() {
        result = result + verse(i).as_str() + "\n";
    }

    result.pop();

    result
}
