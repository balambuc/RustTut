use std::io::stdin;

fn main() {
    println!("\r\n\t\t\t Kreditindex kalkulátor");
    let n_targy = beolv("Felvett tárgyak száma:".to_string());
    let mut s_kredit = 0;
    let mut a_sulyozott = 0;
    for i in 1..n_targy + 1 {
        let kredit = beolv(i.to_string() + ". tárgy kreditértéke:");
        s_kredit += kredit;
        a_sulyozott += kredit * beolv(i.to_string() + ". tárgy érdemjegye:");
    }
    let i_kredit: f64 = (a_sulyozott as f64) / 30.0;
    println!("Felvett kreditek száma: {:?} \nKreditindex: {:.3}",
             s_kredit,
             i_kredit);
}

fn beolv(szoveg: String) -> u8 {
    let mut n = String::new();
    println!("{}", szoveg);
    loop {
        stdin()
            .read_line(&mut n)
            .expect("ERROR");

        let s: u8 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("NaN!");
                n = String::new();
                0
            }
        };
        if s != 0 {
            break;
        };
    }
    match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}
