use std::io;
fn main() {
    // Holds user input.
    let mut s: String = String::new();

    // Holds the user number.
    let mut nword: String = String::new();

    // We get the sentence from the user.
    println!("Please enter a sentence:");
    io::stdin().read_line(&mut s).expect("faild to read line");

    s += " ";

    // We get the number from the user.
    println!("Which word do you want?");
    io::stdin().read_line(&mut nword).expect("faild to read line");

    // we convert the number holder to u32.
    let nword:u32 = nword.trim().parse().expect("Please enter number!");

    // We call the function to give us the desired word.
    let b = n_word(&s, nword);

    // We show the word to the user.
    println!("{b}");
}

fn n_word(s: &str, int: u32) -> &str {
    // We change the string to bytes. 
    let s_byte = s.as_bytes();

    // We define the counter.
    let mut enu: u32 = 0;

    // We define the holder of the previous position.
    let mut b: usize = 0;

    for (i, &item) in s_byte.iter().enumerate(){
        if item == b' ' {
            enu += 1;
            if enu == int {
                return &s[b+1..i];
            }
            
            b = i;
        }
    }
    println!("It does not have the required number of words, Sentence:");
    s

}
