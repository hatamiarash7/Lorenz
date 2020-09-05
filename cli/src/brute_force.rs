#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::{thread_rng, RngCore};
    use std::io::Write;
    use std::time::Instant;

    #[test]
    fn brute_force_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut random_data = vec![0; (1 << 10) * 100]; // 100KiB
        thread_rng().fill_bytes(&mut random_data);
        let mut temp_file = std::env::temp_dir();
        temp_file.push("rand.txt");
        let mut file = std::fs::File::create(&temp_file)?;
        file.write_all(&random_data)?;

        let pw = "abcdefghijkl".to_string();
        let in_file = temp_file.to_str().unwrap().to_string();
        let mut out_path = std::env::temp_dir();
        out_path.push("encrypted.txt");
        let out_file = out_path.to_str().unwrap().to_string();
        let config = lorenz::Config::new(&lorenz::Mode::Encrypt, pw, &in_file, &out_file);
        lorenz::main_routine(&config)?;

        let mut possible_chars = ('a'..='z').collect::<Vec<char>>();
        possible_chars.append(&mut ('A'..='Z').collect());
        possible_chars.append(
            &mut "0123456789!@#$%^&*()-_=+`~,./<>?;':\"[]{}\\|"
                .chars()
                .collect(),
        );
        let mut combiner = possible_chars.iter().combinations_with_replacement(10);
        println!("possible chars: {}", possible_chars.len());

        let num_combos = 1951641934005400.;
        let start_time = Instant::now();
        let mut attempts = 0;

        loop {
            let guess_chars = combiner
                .next()
                .ok_or("end of combinations")?
                .iter()
                .cloned()
                .cloned()
                .collect::<Vec<char>>();
            let guess: String = guess_chars.into_iter().collect();
            let c =
                lorenz::Config::new(&lorenz::Mode::Decrypt, guess.clone(), &out_file, "./result");
            assert!(lorenz::main_routine(&c).is_err());

            attempts += 1;
            let elapsed = Instant::now()
                .duration_since(start_time.clone())
                .as_secs_f64();
            if elapsed == 0. {
                continue;
            };
            let attempts_per_sec = attempts as f64 / elapsed;
            let num_secs = num_combos / attempts_per_sec;
            let num_years = num_secs / (60. * 60. * 24. * 365.);
            if attempts % 100 == 0 {
                println!("guess: {}", guess);
                println!("at {:.3} attempts per second, it would take {:.2} years to test all 12-character passwords including lower-/uppercase letters, numbers, and symbols.", attempts_per_sec, num_years);
            }
        }
        Ok(())
    }
}
