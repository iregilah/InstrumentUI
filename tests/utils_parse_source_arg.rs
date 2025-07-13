// tests/utils_parse_source_arg.rs
use rigol_cli::utils::parse_source_arg;

#[test]
fn parse_channel_variants() {
    let cases = [
        ("1", "CHANnel1"),
        ("2", "CHANnel2"),
        ("CHAN3", "CHANnel3"),
        ("chan4", "CHANnel4"),
        ("math", "MATH"),
        ("EXT", "EXT"),
        ("ext5", "EXT5"),
        ("D0", "D0"),
        ("d15", "D15"),
    ];
    for (input, expected) in cases {
        assert_eq!(parse_source_arg(input).unwrap(), expected);
    }
}

#[test]
fn invalid_inputs() {
    for bad in ["CHAN5", "d16", "xyz"] {
        assert!(parse_source_arg(bad).is_err());
    }
}
