use rand::Rng;
use std::cmp::Ordering;
use std::io;
// panic!(text) = failwith text

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    let (mut a, mut b) = (a0, a1);
    for _ in 0..n {
        (a, b) = (b, a + b)
    }
    return a
}
/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn je_prestopno (y: u32) -> bool {
    if (y % 400 == 0) || (y % 100 != 0 && y % 4 == 0) {return true}
    return false
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

fn je_veljaven_datum (date: Date) -> bool {
    let (d, m, y) = date;
    if m == 2 {
        if je_prestopno(y) {return d <= 29}
        else {return d <= 28}
    }
    else {
        return match m {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 1 <= d && d <= 31,
            2 => 1 <= d && d <= (if je_prestopno(y) {29} else {28}),
            4 | 6 | 9 | 11 => 1 <= d && d <= 30,
            _ => false
        }
    }
}
/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    loop {
        if cond(start) {return start}
        start = fun (start);
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    let mut c = (a + b) / 2.;
    while (b - a) >= prec && (fun(c).abs() >= prec) {
        if fun(a) * fun(c) < 0. {(b, c) = (c, (a + c) / 2.)}
        else {(a, c) = (c, (c + b) / 2.)}
    }
    return  c;
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
} // Not done, didnt understand the instructions

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    return [[a[0][0] * b[0][0] + a[0][1] * b[1][0], a[0][0] * b[0][1] + a[0][1] * b[1][1]], [a[1][0] * b[0][0] + a[1][1] * b[1][0], a[1][0] * b[0][1] + a[1][1] * b[1][1]]]
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    if arr.len() <= 1 {return  true;}
    let (mut prev, mut curr) = (arr[0], arr[1]);
    let mut smer = 0;

    if prev < curr {smer = 1;}
    else if prev > curr {smer = -1;}

    for i in 2..arr.len() {
        (prev, curr) = (curr, arr[i]);
        if (prev < curr && smer == -1) || (prev > curr && smer == 1) {return false;}
        if smer == 0 && prev != curr {
            if prev < curr {smer = 1;}
            else {smer = -1;}
        }
    }
    return true;
}

fn vsebuje<T : PartialEq>(v: &Vec<T>, x : &T) -> bool {
    for y in v {
      if x == y {
        return true
      }
    }
    return false
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

fn pow(mut x: u32, mut n: u32) -> u32 {
    loop {
        if n == 0 {return 1;}
        if n == 1 {return x;}
        if n % 2 == 0 {(x, n) = (x * x, n / 2);}
        else {(x, n) = (x * x * x, (n - 1) / 2);}
    }
}

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32 {
    panic!("Not working");
    if n == 0 {return 1;}
    if n == 1 {return x % m;}
    if n % 2 == 0 {return pow(pow_mod(x, n / 2, m), 2) % m;}
    else {return  pow(pow_mod(x, (n - 1) / 2, m), 3) % m;}
}

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) -> &mut [u32] {
    let mut min = 0;
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[j] < arr[min] {min = j;}
        }
        (arr[min], arr[i]) = (arr[i], arr[min]);
    }
    return arr;
}

/// ------------------------------------------------------------------------------------------------
///// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {
    for i in 0..n {
        let mut row = String::new();
        for j in 0..(2 * n - 1) {
            if j < (n - i - 1) || j > (n + i - 1)
            { row.push(' '); }
            else { row.push('*'); }
        }
        println!("{row}");
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///       A
///     A B A
///   A B C B A
/// A B C D C B A

fn alphabet_pyramid(n: u8) {
    let A: u8 = b'A';
    // to_ascii_uppercase();
    for i in 0..n {
        let mut row = String::new();
        let mut counter = 0;
        let mut next_letter = true;
        for j in 0..(4 * n - 3) {
            if j > (2 * (n + i)) {break}
        if j < (2 * (n - i - 1)) {row.push(' ');}
            else {
                if next_letter {
                    row.push((A + counter) as char);
                    if j > (2 * n - 2) {counter -= 1;}
                    else {counter += 1;}
                    next_letter = false;
                }
                else {
                    row.push(' ');
                    next_letter = true;
                }
            }
        }
        println!("{row}");
    }
}

fn main() {
    let a = fib(0, 1, 10);
    println!("Deseto stevilo: {a}");
    let nicla = bisekcija(0.1, 3.9, |x|(x*x - 2.), 0.0001);
    println!("Nicla: {nicla}");
    let date = (29, 2, 2001);
    let veljaven = je_veljaven_datum(date);
    println!("Dani datum je veljaven: {veljaven}");
    let m = mat_mul([[1, 0], [0, 1]], [[0, 1], [0, 1]]);
    println!("Zmnozeni matriki: {:?}", m);
    let urejen = ordered(&[9, 8, 5, 4, 3, 3, 1, 0]);
    println!("Seznam je urejen: {urejen}");
    let potenca = pow(2, 20);
    println!("Potenca je: {potenca}");
    //let ostanek = pow_mod(5, 1, 4);
    //println!("Ostanek potence je: {ostanek}");
    pyramid(8);
    alphabet_pyramid(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {
    }
}
