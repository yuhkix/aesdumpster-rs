#[cfg(windows)]
use windows::Win32::System::Console::{
    GetStdHandle, SetConsoleTextAttribute, FOREGROUND_BLUE, FOREGROUND_GREEN, FOREGROUND_INTENSITY,
    FOREGROUND_RED, STD_OUTPUT_HANDLE,
};

#[cfg(unix)]
use crossterm::{
    execute,
    style::{Color, ResetColor, SetForegroundColor},
};

#[derive(Clone)]
pub struct Key(pub String);

pub struct Keys {
    pub key_vector: Vec<Key>,
}

impl Keys {
    pub fn new() -> Self {
        Self {
            key_vector: Vec::new(),
        }
    }
}

pub struct KeyDumpster {
    pub keys: Keys,
    pub key_entropies: Vec<f64>,
    pub most_likely_indices: Vec<usize>,

    key_patterns: Vec<&'static str>,
    false_positives: Vec<&'static str>,
    key_dword_offsets: Vec<Vec<usize>>,
}

impl KeyDumpster {
    pub fn new() -> Self {
        Self {
            keys: Keys::new(),
            key_entropies: Vec::new(),
            most_likely_indices: Vec::new(),
            key_patterns: vec![
                "C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ?",
                "C7 ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ?",
                "C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? 48 ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ?",
                "C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? ? C7 ? ? ? ? ? C3",
            ],
            false_positives: vec![
                "FFD9FFD9FFD9FFD9FFD9FFD9FFD9FFD9FFD9FFD9FFD9FFD9FFD9FFD9FFD9FFD9",
                "67E6096A85AE67BB72F36E3C3AF54FA57F520E518C68059BABD9831F19CDE05B",
                "D89E05C107D57C3617DD703039590EF7310BC0FF11155868A78FF964A44FFABE",
                "9A99593F9A99593F0AD7633F52B8BE3FE17A543FCDCC4C3D4260E53BAE47A13F",
                "6F168073B9B21449D742241700068ADABC306FA9AA3831164DEE8DE34E0EFBB0",
                "0AD7633FCDCC4C3DCDCCCC3D52B8BE3F9A99593F9A99593FC9767E3FE17A543F",
                "168073C7B21449C7430C00064310BC304314AA3843184DEE431C4E0E83C4205B",
                "E6096AC7AE67BBC7430C3AF543107F5243148C684318ABD9431C19CD436C2000",
                "9E05C1C7D57C36C7430C39594310310B431411154318A78F431CA44F436C1C00",
                "9E05C1C7D57C36C7DD7030C7590EF7C70BC0FFC7155868C78FF964C7A44FFABE",
                "168073C7B21449C7422417C7068ADAC7306FA9C7383116C7EE8DE3C74E0EFBB0",
                "0AD7633FCDCC4C3D00C742143DC742183FC7421C3FC742203FC742247E3FC742",
                "0000803F0AD7A33E0AD7633F52B8BE3FE17A543FCDCC4C3D4260E53B54AE47A1",
                "0AD7A33E0AD7633F52B8BE3FE17A543FCDCC4C3D4260E53BAE47A13F58583934",
                "0AD7A33E0AD7633F52B8BE3FE17A543FCDCC4C3D4260E53BAE47A13F38583934",
                "0000803F0AD7A33E0AD7633F52B8BE3FE17A543FCDCC4C3D4260E53B34AE47A1",
                "0000803F0000803F0AD7A33E0AD7633F52B8BE3FE17A543FCDCC4C3D2C4260E5",
                "0AD7633F52B8BE3FE17A543FCDCC4C3D4260E53BAE47A13F5839343C4CC9767E",
                "0AD7633F52B8BE3FE17A543FCDCC4C3D4260E53BAE47A13F5839343C4CC9767E",
                "07D57C3617DD703039590EF7310BC0FF11155868A78FF964A44FFABE6C1C0000",
                "85AE67BB72F36E3C3AF54FA57F520E518C68059BABD9831F19CDE05B6C200000",
                "E6096AC7AE67BBC7F36E3CC7F54FA5C7520E51C768059BC7D9831FC719CDE05B",
                "0AD7A33E0AD7633F52B8BE3FE17A543FCDCC4C3D4260E53BAE47A13F3C583934",
                "E4D6E74FE4D667500044AC47926595380080DC43000A9B46000080BF000080BF",
                "D04C8F7D71ECC047D8A60970FBA31C9E9EC1250BBBF6459AC480947212E1DB8C",
            ],
            key_dword_offsets: vec![
                vec![3, 10, 17, 24, 35, 42, 49, 56],
                vec![2, 9, 16, 23, 30, 37, 44, 51],
                vec![3, 10, 21, 28, 35, 42, 49, 56],
                vec![51, 45, 38, 31, 24, 17, 10, 3],
            ],
        }
    }

    pub fn find_aes_keys(&mut self, buffer: &[u8]) -> bool {
        let mut found_any = false;
        for (i, pattern) in self.key_patterns.iter().enumerate() {
            let matches = find_signature(buffer, pattern);
            let offsets = &self.key_dword_offsets[i];
            for base in matches {
                if let Some(hex) = self.concatenate_aes_type(buffer, base, offsets) {
                    self.keys.key_vector.push(Key(hex));
                }
            }
        }

        self.key_entropies = self.key_entropy_generator();
        if self.key_entropies.is_empty() || self.keys.key_vector.is_empty() {
            return false;
        }
        let (max_val, indices) = find_max_elements(&self.key_entropies);
        if max_val.is_finite() && !indices.is_empty() {
            self.most_likely_indices = indices;
            found_any = true;
        }
        found_any
    }

    fn concatenate_aes_type(
        &self,
        buf: &[u8],
        base: usize,
        offsets: &Vec<usize>,
    ) -> Option<String> {
        let mut out = String::new();
        for off in offsets.iter() {
            let idx = base.checked_add(*off)?;
            if idx + 4 > buf.len() {
                return None;
            }
            out.push_str(&hex_str(&buf[idx..idx + 4]));
        }
        Some(out.to_uppercase())
    }

    fn key_entropy_generator(&self) -> Vec<f64> {
        self.keys
            .key_vector
            .iter()
            .map(|k| calc_entropy(&k.0))
            .collect()
    }

    pub fn print_key_information(&self) {
        #[cfg(windows)]
        {
            let hconsole = unsafe { GetStdHandle(STD_OUTPUT_HANDLE).expect("GetStdHandle failed") };
            for (i, key) in self.keys.key_vector.iter().enumerate() {
                let ent = self.key_entropies.get(i).cloned().unwrap_or(0.0);
                let color = if ent >= 3.7 {
                    FOREGROUND_GREEN | FOREGROUND_INTENSITY
                } else if ent >= 3.5 {
                    FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_INTENSITY
                } else if ent >= 3.4 {
                    FOREGROUND_RED | FOREGROUND_GREEN
                } else if ent >= 3.3 {
                    FOREGROUND_RED | FOREGROUND_INTENSITY
                } else {
                    FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE
                };

                let is_most_likely = self.most_likely_indices.contains(&i);
                if ent >= 3.3 && !self.false_positives.iter().any(|fp| fp == &key.0) {
                    let final_color = if is_most_likely {
                        FOREGROUND_GREEN | FOREGROUND_INTENSITY
                    } else {
                        color
                    };
                    unsafe {
                        SetConsoleTextAttribute(hconsole, final_color).unwrap();
                    }
                    println!("Key: 0x{} | Key Entropy: {:.2}\n", key.0, ent);
                }
            }
            unsafe {
                SetConsoleTextAttribute(
                    hconsole,
                    FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE,
                )
                .unwrap();
            }
        }
        #[cfg(unix)]
        {
            let mut stdout = std::io::stdout();
            for (i, key) in self.keys.key_vector.iter().enumerate() {
                let ent = self.key_entropies.get(i).cloned().unwrap_or(0.0);

                let color = if ent >= 3.7 {
                    Color::Green
                } else if ent >= 3.5 {
                    Color::Yellow
                } else if ent >= 3.4 {
                    Color::DarkYellow
                } else if ent >= 3.3 {
                    Color::Red
                } else {
                    Color::White
                };

                let is_most_likely = self.most_likely_indices.contains(&i);
                if ent >= 3.3 && !self.false_positives.iter().any(|fp| fp == &key.0) {
                    let final_color = if is_most_likely { Color::Green } else { color };
                    execute!(stdout, SetForegroundColor(final_color)).unwrap();
                    println!("Key: 0x{} | Key Entropy: {:.2}\n", key.0, ent);
                }
            }
            // Reset color at the end
            execute!(stdout, ResetColor).unwrap();
        }
    }
}

fn hex_str(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn calc_entropy(s: &str) -> f64 {
    use std::collections::HashMap;
    let mut freq: HashMap<char, usize> = HashMap::new();
    for ch in s.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    let len = s.len() as f64;
    let mut info = 0.0;
    for (_ch, count) in freq {
        let p = (count as f64) / len;
        info += p * (p.ln() / 2f64.ln());
    }
    -info
}

fn find_max_elements(v: &[f64]) -> (f64, Vec<usize>) {
    let mut indices = Vec::new();
    let mut current_max = f64::NEG_INFINITY;
    for (i, &val) in v.iter().enumerate() {
        if val > current_max {
            current_max = val;
            indices.clear();
        }
        if val == current_max {
            indices.push(i);
        }
    }
    (current_max, indices)
}

fn parse_signature(pattern: &str) -> Vec<Option<u8>> {
    pattern
        .split_whitespace()
        .map(|tok| {
            if tok == "?" || tok == "??" || tok.contains('?') {
                None
            } else {
                u8::from_str_radix(tok, 16).ok()
            }
        })
        .collect()
}

fn find_signature(buf: &[u8], pattern: &str) -> Vec<usize> {
    let sig = parse_signature(pattern);
    if sig.is_empty() {
        return Vec::new();
    }
    let sig_len = sig.len();
    let mut out = Vec::new();
    if buf.len() < sig_len {
        return out;
    }
    for i in 0..=(buf.len() - sig_len) {
        let mut matched = true;
        for (j, maybe) in sig.iter().enumerate() {
            if let Some(b) = maybe {
                if buf[i + j] != *b {
                    matched = false;
                    break;
                }
            }
        }
        if matched {
            out.push(i);
        }
    }
    out
}
