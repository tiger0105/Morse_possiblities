type CODE = (&'static str, &'static str);
const CODES: &'static [CODE] = &[
    ("A", ".-"),
    ("B", "-..."),
    ("C", "-.-."),
    ("D", "-.."),
    ("E", "."),
    ("F", "..-."),
    ("G", "--."),
    ("H", "...."),
    ("I", ".."),
    ("J", ".---"),
    ("K", "-.-"),
    ("L", ".-.."),
    ("M", "--"),
    ("N", "-."),
    ("O", "---"),
    ("P", ".--."),
    ("Q", "--.-"),
    ("R", ".-."),
    ("S", "..."),
    ("T", "-"),
    ("U", "..-"),
    ("V", "...-"),
    ("W", ".--"),
    ("X", "-..-"),
    ("Y", "-.--"),
    ("Z", "--.."),
];
fn possibilities(signals: &str) -> Vec<String> {
    let mut cases = vec![signals.to_string()];
    loop {
        if !cases.iter().any(|it| it.contains("?")) { break; }
        cases = cases.iter()
            .map(|it| {
                if !it.contains("?") {
                    vec!(it.clone())
                } else {
                    vec!(
                        it.replacen("?", ".", 1),
                        it.replacen("?", "-", 1)
                    )
                }
            })
            .flatten()
            .collect();
    }
    
    CODES.into_iter()
        .filter(move |it| cases.iter().any(|c| *c == it.1) )
        .map(|it| it.0.to_string())
        .collect()
}
fn main() {
    let result0 = possibilities(".");
    let result1 = possibilities(".-");
    let result2 = possibilities("?");
    let result3 = possibilities("?.");
    let result4 = possibilities(".?");
    let result5 = possibilities("?-?");
    println!("Result0 = {:?}", result0);
    println!("Result1 = {:?}", result1);
    println!("Result2 = {:?}", result2);
    println!("Result3 = {:?}", result3);
    println!("Result4 = {:?}", result4);
    println!("Result5 = {:?}", result5);
