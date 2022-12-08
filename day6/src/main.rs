use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let first_packet = find_marker(&input, &MarkerType::StartOfPacket);
    let first_message = find_marker(&input, &MarkerType::StartOfMessage);
    println!("{}, {}", first_packet, first_message);
}

#[derive(Clone, Copy)]
enum MarkerType {
    StartOfPacket = 4,
    StartOfMessage = 14,
}

fn find_marker(buffer: &str, marker: &MarkerType) -> usize {
    let marker_length = *marker as usize;
    let char_vec = buffer.chars().collect::<Vec<char>>();

    for i in 0..buffer.len() {
        let start = i;
        let end = i+marker_length;
        let comparison_window = &char_vec[start..end];
        let comparison_set: HashSet<&char> = HashSet::from_iter(comparison_window.iter());

        if comparison_set.len() == marker_length {
            return end 
        }
    }

    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_input_returns_five() {
        let test_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = find_marker(&test_input, &MarkerType::StartOfPacket);
        assert_eq!(result, 5);
    }
}
