pub fn time33_key_fn<T: ToString + PartialEq>(t: &T) -> usize {
    t.to_string()
        .chars()
        .fold(0, |acc, c| acc + (c as usize) * 33)
}

#[cfg(test)]
mod test {}
