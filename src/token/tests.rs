use super::*;

#[test]
fn test_lookup_ident() {
    let tests = vec![
        ("fn", TokenType::Function),
        ("let", TokenType::Let),
        ("true", TokenType::True),
        ("false", TokenType::False),
        ("if", TokenType::If),
        ("else", TokenType::Else),
        ("return", TokenType::Return),
        ("my_ident", TokenType::Ident),
    ];

    for (input, expected) in tests {
        let result = lookup_ident(input);
        assert_eq!(result, expected);
    }
}