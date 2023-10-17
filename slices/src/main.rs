fn main() {
    let w1 = "FIRST SECOND";
    let first_word = first_word(w1);
    println!("The first word is {first_word}");

    let w2 = "FRIST SECOND THIRD";
    let second_word = second_word(w2);
    println!("The second word is {second_word}");
    //string = "String", string slice = "str"
    //you should use "str" instead of "String" when possible

    let arr = [4, 5, 7, 2, 3, 1, 10];
    let arr_slice = &arr[2..5];
    assert_eq!(arr_slice, &arr[2..5]);
}

fn first_word(text: &str) -> &str {
    for (i, &byte) in text.as_bytes().iter().enumerate() {
        if byte == 32 {
            return &text[..i];
        }
    }
    text
}

fn second_word(text: &str) -> &str {
    for (mut start, &byte_start) in text.as_bytes().iter().enumerate() {
        if byte_start == 32 {
            start += 1;
            for (end, &byte_end) in text[start..].as_bytes().iter().enumerate() {
                if byte_end == 32 {
                    return &text[start..start + end];
                }
            }
        }
    }
    text
}
