

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "byte index 2 is not a char boundary")]
    fn string_slice_into_middle_of_character_panics() {
        let s = "თhello world";
        let sub_s = &s[0..2];
    }
}
