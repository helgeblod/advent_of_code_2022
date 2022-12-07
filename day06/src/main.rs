use std::fs;

fn main() {
    let message: String = fs::read_to_string("./src/secret-message.txt").expect("Could not open input file");
    let packet_marker = find_marker(message.as_str(), 4).expect("Could not find marker");
    let message_marker = find_marker(message.as_str(), 14).expect("Could not find marker");
    println!("📦 Packet marker is: {}", packet_marker);
    println!("✉️ Message marker is: {}", message_marker);
}

fn find_marker(message: &str, num_distinct_chars: i32) -> Option<i32> {
    let mut marker_value: Option<i32> = None;
    for i in 0..message.len() {
        let char_range = &message[i..i + num_distinct_chars as usize];
        let mut potential_marker = char_range.chars().collect::<Vec<char>>();
        potential_marker.sort();
        potential_marker.dedup();
        if potential_marker.len() == num_distinct_chars as usize {
            marker_value = Some(i as i32 + num_distinct_chars);
            break;
        }
    }
    marker_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker1() {
        let message = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = find_marker(message, 4).unwrap();
        assert_eq!(marker, 5);
    }


    #[test]
    fn test_marker2() {
        let message = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = find_marker(message, 4).unwrap();
        assert_eq!(marker, 6);
    }

    #[test]
    fn test_marker3() {
        let message = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = find_marker(message, 4).unwrap();
        assert_eq!(marker, 10);
    }

    #[test]
    fn test_marker4() {
        let message = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = find_marker(message, 4).unwrap();
        assert_eq!(marker, 11);
    }

    #[test]
    fn test_marker5() {
        let message = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let marker = find_marker(message, 14).unwrap();
        assert_eq!(marker, 19);
    }

    #[test]
    fn test_marker6() {
        let message = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = find_marker(message, 14).unwrap();
        assert_eq!(marker, 23);
    }

    #[test]
    fn test_marker7() {
        let message = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = find_marker(message, 14).unwrap();
        assert_eq!(marker, 23);
    }

    #[test]
    fn test_marker8() {
        let message = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = find_marker(message, 14).unwrap();
        assert_eq!(marker, 29);
    }

    #[test]
    fn test_marker9() {
        let message = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = find_marker(message, 14).unwrap();
        assert_eq!(marker, 26);
    }
}

