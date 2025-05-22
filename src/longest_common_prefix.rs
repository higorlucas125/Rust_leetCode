

pub fn logest_prefix(strs: Vec<String>) -> String {
    let mut s: String  = if strs.len() == 1 && strs[0].len() == 1 {
        strs[0].to_string()
    } else {
        "".to_string()
    };
    let mut prefix = strs[0].clone();

    for s in strs.iter() {
        let mut i = 0;
        let max = prefix.len().min(s.len());
        while i < max && prefix.as_bytes()[i] == s.as_bytes()[i] {
            i += 1;
        }
        prefix = prefix[0..i].to_string();

        if prefix.is_empty() {
            break;
        }
    }

    prefix
} 

pub fn longest_common_now(strss: Vec<String>) -> String {
    if strss.is_empty() {
        return String::new();
    }

    // Pegamos a primeira strsing como referência
    let first = &strss[0];
    let mut prefix = String::new();

    for (i, ch) in first.chars().enumerate() {
        // Verifica o caractere na posição i em todas as strsings
        for s in strss.iter().skip(1) {
            // Se a strsing for muito curta ou caractere diferente, retorna o prefixo atual
            if s.len() <= i || s.chars().nth(i) != Some(ch) {
                return prefix;
            }
        }
        // Se passou, adiciona o caractere ao prefixo
        prefix.push(ch);
    }

    prefix
}

