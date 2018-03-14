fn read_digits<T>(input: T) -> Vec<u32>
    where T: Iterator<Item=String>
{
    input
        .into_iter()
        .flat_map::<Vec<char>, _>(|line| line.chars().collect())
        .map(|c: char| c.to_digit(10).expect("Invalid digit"))
        .collect()
}

pub fn solve<T: Iterator<Item=String>>(input: T) -> u32 {
    let mut digits: Vec<u32> = read_digits(input);
    let first: u32 = *digits.first().unwrap();
    digits.push(first);

    digits
        .windows(2)
        .fold(
            0,
            |sum, window| {
                let a = window[0];
                let b = window[1];
                if a == b { sum + a }
                else { sum }
            }
        )
}
