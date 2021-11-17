pub fn update_if_changed<T: PartialEq>(prev: &mut T, new: T) -> bool {
    if *prev == new {
        false
    } else {
        *prev = new;
        true
    }
}
