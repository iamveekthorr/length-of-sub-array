fn main() {
    let string = "0101010101";
    let length = find_length(string);

    println!("{}", length);
}

fn find_length(st: &str) -> usize {
    let mut left = 0;
    let mut current = 0;
    let mut ans = 0;

    for right in 0..st.len() {
        if st.as_bytes()[right] == b'0' {
            current += 1;
        }

        while current > 1 {
            if st.as_bytes()[left] == b'0' {
                current -= 1;
            }
            left += 1;
        }

        ans = ans.max(right - left + 1);
    }

    ans
}
