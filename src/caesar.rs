use crate::models::NumberArgs;

use super::char_utils::char_mod;

pub fn init() -> NumberArgs {
    NumberArgs { number: 0 }
}

pub fn next(NumberArgs { number }: NumberArgs) -> Option<NumberArgs> {
    if number >= 25 {
        None
    } else {
        Some(NumberArgs { number: number + 1 })
    }
}

pub fn decrypt(strs: Vec<String>, NumberArgs { number }: NumberArgs) -> Vec<String> {
    strs.iter()
        .map(|str| {
            str.chars()
                .into_iter()
                .map(|c| char_mod(c, number.try_into().unwrap(), true))
                .collect()
        })
        .collect()
}

pub fn encrypt(strs: Vec<String>, NumberArgs { number }: NumberArgs) -> Vec<String> {
    let size = 26;
    decrypt(
        strs,
        NumberArgs {
            number: size - number,
        },
    )
}

#[cfg(test)]
mod tests {

    use crate::models;

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(char_mod('A', 5, true), 'F')
    }
    #[test]
    fn it_works_2() {
        assert_eq!(
            decrypt(
                vec!["MYNAMEISCAESAR".to_string()],
                models::NumberArgs { number: 10 }
            ),
            vec!["WIXKWOSCMKOCKB"]
        )
    }
    #[test]
    fn it_works_3() {
        assert_eq!(
            decrypt(vec!["YVIORM".to_string()], models::NumberArgs { number: 1 }),
            vec!["ZWJPSN"]
        )
    }

    #[test]
    fn it_works_4() {
        assert_eq!(
            decrypt(
                encrypt(
                    vec!["YVIORM".to_string()],
                    models::NumberArgs { number: 10 }
                ),
                models::NumberArgs { number: 10 }
            ),
            vec!["YVIORM"]
        )
    }
}
