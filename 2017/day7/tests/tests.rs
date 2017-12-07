extern crate day7;
use day7::*;


#[test]
fn test_sample_input1() {
    assert_eq!(Some(Entry {
        name: "jgtvhkv".to_string(),
        weight: 1885,
        children: vec!["ykqcpiv".to_string(), "gvupyd".to_string(), "vuyxvq".to_string()],
    }), str_to_entry("jgtvhkv (1885) -> ykqcpiv, gvupyd, vuyxvq"));
}

#[test]
fn test_sample_input2() {
    assert_eq!(Some(Entry {
        name: "siyms".to_string(),
        weight: 98,
        children: vec![],
    }), str_to_entry("siyms (98)"));
}

#[test]
fn test_sample_input3() {
    let expected = vec![
        Entry {
            name: "jgtvhkv".to_string(),
            weight: 1885,
            children: vec!["ykqcpiv".to_string(), "gvupyd".to_string(), "vuyxvq".to_string()],
        },
        Entry {
            name: "siyms".to_string(),
            weight: 98,
            children: vec![],
        }
    ];

    let input = "jgtvhkv (1885) -> ykqcpiv, gvupyd, vuyxvq\nsiyms (98)\n";

    assert_eq!(expected, input_to_vec(input));
}