use super::*;

#[test]
fn base62_decode() {
    let test_values = [
        (
            "4p9i0XLjMgsD1nSz23q8JF3jMQ3dPQn907i8NTfGh7OmeEz13A9AAw30xwA",
            vec![
                120, 218, 139, 244, 181, 140, 12, 200, 14, 245, 41, 205, 246, 173, 242, 73, 113,
                143, 12, 116, 119, 15, 116, 246, 244, 46, 242, 205, 244, 141, 112, 119, 76, 79,
                204, 245, 13, 244, 0, 0, 223, 116, 12, 3,
            ],
        ),
        (
            "YM9YPkULukMzLdGYQGGQCIKrMiMXGAgamMQH",
            vec![
                24, 211, 99, 15, 73, 45, 46, 201, 204, 75, 103, 96, 144, 97, 64, 2, 162, 172, 140,
                200, 92, 6, 0, 106, 38, 3, 29,
            ],
        ),
    ];
    for (code, data) in test_values {
        let result = decode(code);
        assert_eq!(result, Some(data));
    }
}

#[test]
fn base62_encode() {
    let test_values = [
        (
            vec![
                120, 218, 139, 244, 181, 140, 12, 200, 14, 245, 41, 205, 246, 173, 242, 73, 113,
                143, 12, 116, 119, 15, 116, 246, 244, 46, 242, 205, 244, 141, 112, 119, 76, 79,
                204, 245, 13, 244, 0, 0, 223, 116, 12, 3,
            ],
            "4p9i0XLjMgsD1nSz23q8JF3jMQ3dPQn907i8NTfGh7OmeEz13A9AAw30xwA",
        ),
        (
            vec![
                24, 211, 99, 15, 73, 45, 46, 201, 204, 75, 103, 96, 144, 97, 64, 2, 162, 172, 140,
                200, 92, 6, 0, 106, 38, 3, 29,
            ],
            "YM9YPkULukMzLdGYQGGQCIKrMiMXGAgamMQH",
        ),
    ];
    for (data, code) in test_values {
        let result = encode(&data);
        assert_eq!(result, Some(code.to_string()));
    }
}
