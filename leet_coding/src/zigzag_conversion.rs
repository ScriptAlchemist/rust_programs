#[allow(dead_code)]
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows <= 0 {
        panic!("Number of rows must be positive");
    }
    let num_rows = num_rows as usize;
    if num_rows == 1 {
        return s;
    }

    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let cycle_len = 2 * num_rows - 2;
    let mut result = String::new();

    for i in 0..num_rows {
        let mut j = 0;
        while j + i < n {
            result.push(chars[i + j]);
            if i != 0 && i != num_rows - 1 && j + cycle_len - i < n {
                result.push(chars[j + cycle_len - i]);
            }
            j += cycle_len;
        }
    }
    result
}

#[cfg(test)]
mod v1_tests {
    use super::*;
    /*
        P   A   H   N
        A P L S I I G
        Y   I   R
    */
    #[test]
    fn test_1() {
        assert_eq!(convert(String::from("PAYPALISHIRING"), 3), "PAHNAPLSIIGYIR");
    }
    /*
        P     I    N
        A   L S  I G
        Y A   H R
        P     I
    */
    #[test]
    fn test_2() {
        assert_eq!(convert(String::from("PAYPALISHIRING"), 4), "PINALSIGYAHRPI");
    }
    /*
        IICP
        STAS
    */
    #[test]
    fn test_3() {
        assert_eq!(convert(String::from("ISITCAPS"), 2), "IICPSTAS");
    }
    /*
        A
    */
    #[test]
    fn test_4() {
        assert_eq!(convert(String::from("A"), 1), "A");
    }
}

#[allow(dead_code)]
pub fn convert_v2(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let cycle_len = 2 * num_rows - 2;
    let mut modulus_calc: i32;
    let mut str_output = String::from("");

    for i in 0..num_rows {
        for (idx, c) in s.chars().enumerate() {
            modulus_calc = idx as i32 % cycle_len as i32;
            if modulus_calc == i || modulus_calc == cycle_len - i {
                str_output.push(c);
            }
        }
    }
    str_output
}

#[cfg(test)]
mod v2_tests {
    use super::*;
    /*
        P   A   H   N
        A P L S I I G
        Y   I   R
    */
    #[test]
    fn test_1() {
        assert_eq!(
            convert_v2(String::from("PAYPALISHIRING"), 3),
            "PAHNAPLSIIGYIR"
        );
    }
    /*
        P     I    N
        A   L S  I G
        Y A   H R
        P     I
    */
    #[test]
    fn test_2() {
        assert_eq!(
            convert_v2(String::from("PAYPALISHIRING"), 4),
            "PINALSIGYAHRPI"
        );
    }
    /*
        IICP
        STAS
    */
    #[test]
    fn test_3() {
        assert_eq!(convert_v2(String::from("ISITCAPS"), 2), "IICPSTAS");
    }
    /*
        A
    */
    #[test]
    fn test_4() {
        assert_eq!(convert_v2(String::from("A"), 1), "A");
    }
}

#[allow(dead_code)]
pub fn convert_v3(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let num_rows = num_rows as usize;
    let cycle: usize = 2 * num_rows - 2;

    (0..num_rows)
        .flat_map(|row| {
            s.chars().enumerate().skip(row).filter_map(move |(i, c)| {
                if i % cycle == row || i % cycle == cycle - row {
                    Some(c)
                } else {
                    None
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod v3_tests {
    use super::*;
    /*
        P   A   H   N
        A P L S I I G
        Y   I   R
    */
    #[test]
    fn test_1() {
        assert_eq!(
            convert_v3(String::from("PAYPALISHIRING"), 3),
            "PAHNAPLSIIGYIR"
        );
    }
    /*
        P     I    N
        A   L S  I G
        Y A   H R
        P     I
    */
    #[test]
    fn test_2() {
        assert_eq!(
            convert_v3(String::from("PAYPALISHIRING"), 4),
            "PINALSIGYAHRPI"
        );
    }
    /*
        IICP
        STAS
    */
    #[test]
    fn test_3() {
        assert_eq!(convert_v3(String::from("ISITCAPS"), 2), "IICPSTAS");
    }
    /*
        A
    */
    #[test]
    fn test_4() {
        assert_eq!(convert_v3(String::from("A"), 1), "A");
    }
}

pub fn convert_v4(s: String, num_rows: i32) -> String {
    let mut zigzags: Vec<_> = (0..num_rows)
        .chain((1..num_rows - 1 ).rev())
        .cycle()
        .zip(s.chars())
        .collect();
    zigzags.sort_by_key(|&(row, _),| row);
    zigzags.into_iter()
        .map(|(_, c)| c)
        .collect()
}
