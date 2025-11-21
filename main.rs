fn main()
{
    // Create a buffer to store our inputs
    let mut buffer: [[char; 5]; 6] = [['0'; 5]; 6];

    let word_to_find: &'static str = "harsh";

    buffer[1][1] = 'a';

    for elem in buffer
    {
        // println!("{:?}", elem);
        for char in elem
        {
            print!("{}", char);
            print!(" ");
        }
        println!("");
    }

    println!("Random element is {}", buffer[1][1]);
    println!("The word to find is {}", word_to_find);
}
