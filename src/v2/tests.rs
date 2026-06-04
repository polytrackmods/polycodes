use super::*;
use crate::{decode_track_data, encode_track_data, export_to_id};

#[test]
fn track_decode() {
    let test_values = [
        (
            "v1nBwIreozarBQABAAAAAACAAAAAAACAAAEAAQAAAP7_fwAAAP__fwIAAAIAAAD-_38AAAADAIACAACAAAAAAQCAAyQAAQAAAAAAgAAAAP7_fwAqAAEAAAD9_38AAAACAIAA",
            (
                "Ireozar".to_string(),
                (),
                vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 1, 0, 1, 0, 0, 0, 254, 255,
                    127, 0, 0, 0, 255, 255, 127, 2, 0, 0, 2, 0, 0, 0, 254, 255, 127, 0, 0, 0, 3, 0,
                    128, 2, 0, 0, 128, 0, 0, 0, 1, 0, 128, 3, 36, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0,
                    0, 254, 255, 127, 0, 42, 0, 1, 0, 0, 0, 253, 255, 127, 0, 0, 0, 2, 0, 128, 0,
                ],
            ),
        ),
        (
            "v1nBwTestingBQABAAAAAACAAAAAAACAACQAAQAAAAAAgAAAAP7_fwAqAAIAAAD9_38AAAABAIAA__9_AAAAAwCAAA",
            (
                "Testing".to_string(),
                (),
                vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 36, 0, 1, 0, 0, 0, 0, 0,
                    128, 0, 0, 0, 254, 255, 127, 0, 42, 0, 2, 0, 0, 0, 253, 255, 127, 0, 0, 0, 1,
                    0, 128, 0, 255, 255, 127, 0, 0, 0, 3, 0, 128, 0,
                ],
            ),
        ),
    ];
    for (code, track) in test_values {
        let result = V2Track::decode_track_code(code);
        assert_eq!(result, Some(track));
    }
}

#[test]
fn track_encode() {
    let test_values = [
        (
            (
                "Ireozar".to_string(),
                (),
                vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 1, 0, 1, 0, 0, 0, 254, 255,
                    127, 0, 0, 0, 255, 255, 127, 2, 0, 0, 2, 0, 0, 0, 254, 255, 127, 0, 0, 0, 3, 0,
                    128, 2, 0, 0, 128, 0, 0, 0, 1, 0, 128, 3, 36, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0,
                    0, 254, 255, 127, 0, 42, 0, 1, 0, 0, 0, 253, 255, 127, 0, 0, 0, 2, 0, 128, 0,
                ],
            ),
            "v1nBwIreozarBQABAAAAAACAAAAAAACAAAEAAQAAAP7_fwAAAP__fwIAAAIAAAD-_38AAAADAIACAACAAAAAAQCAAyQAAQAAAAAAgAAAAP7_fwAqAAEAAAD9_38AAAACAIAA",
        ),
        (
            (
                "Testing".to_string(),
                (),
                vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 36, 0, 1, 0, 0, 0, 0, 0,
                    128, 0, 0, 0, 254, 255, 127, 0, 42, 0, 2, 0, 0, 0, 253, 255, 127, 0, 0, 0, 1,
                    0, 128, 0, 255, 255, 127, 0, 0, 0, 3, 0, 128, 0,
                ],
            ),
            "v1nBwTestingBQABAAAAAACAAAAAAACAACQAAQAAAAAAgAAAAP7_fwAqAAIAAAD9_38AAAABAIAA__9_AAAAAwCAAA",
        ),
    ];
    for ((name, _, track_data), code) in test_values {
        let result = V2Track::encode_track_code(name, (), &track_data);
        assert_eq!(result, Some(code.to_string()));
    }
}

#[test]
fn data_decode() {
    let test_values = [(
        vec![
            0, 0, 1, 0, 0, 0, 255, 255, 127, 0, 0, 0, 0, 0, 128, 0, 41, 0, 2, 0, 0, 0, 0, 0, 128,
            0, 0, 0, 255, 255, 127, 0, 2, 0, 128, 0, 0, 0, 0, 0, 128, 1, 5, 0, 1, 0, 0, 0, 254,
            255, 127, 0, 0, 0, 0, 0, 128, 1, 43, 0, 1, 0, 0, 0, 255, 255, 127, 3, 0, 0, 1, 0, 128,
            1,
        ],
        V2Track {
            parts: vec![
                V2Part {
                    id: 0,
                    amount: 1,
                    blocks: vec![V2Block {
                        x: -1,
                        y: 0,
                        z: 0,
                        rotation: 0,
                    }],
                },
                V2Part {
                    id: 41,
                    amount: 2,
                    blocks: vec![
                        V2Block {
                            x: 0,
                            y: 0,
                            z: -1,
                            rotation: 0,
                        },
                        V2Block {
                            x: 2,
                            y: 0,
                            z: 0,
                            rotation: 1,
                        },
                    ],
                },
                V2Part {
                    id: 5,
                    amount: 1,
                    blocks: vec![V2Block {
                        x: -2,
                        y: 0,
                        z: 0,
                        rotation: 1,
                    }],
                },
                V2Part {
                    id: 43,
                    amount: 1,
                    blocks: vec![V2Block {
                        x: -1,
                        y: 3,
                        z: 1,
                        rotation: 1,
                    }],
                },
            ],
        },
    )];
    for (data, track_data) in test_values {
        let result = decode_track_data(&data);
        assert_eq!(result, Some(track_data));
    }
}

#[test]
fn data_encode() {
    let test_values = [(
        V2Track {
            parts: vec![
                V2Part {
                    id: 0,
                    amount: 1,
                    blocks: vec![V2Block {
                        x: -1,
                        y: 0,
                        z: 0,
                        rotation: 0,
                    }],
                },
                V2Part {
                    id: 41,
                    amount: 2,
                    blocks: vec![
                        V2Block {
                            x: 0,
                            y: 0,
                            z: -1,
                            rotation: 0,
                        },
                        V2Block {
                            x: 2,
                            y: 0,
                            z: 0,
                            rotation: 1,
                        },
                    ],
                },
                V2Part {
                    id: 5,
                    amount: 1,
                    blocks: vec![V2Block {
                        x: -2,
                        y: 0,
                        z: 0,
                        rotation: 1,
                    }],
                },
                V2Part {
                    id: 43,
                    amount: 1,
                    blocks: vec![V2Block {
                        x: -1,
                        y: 3,
                        z: 1,
                        rotation: 1,
                    }],
                },
            ],
        },
        vec![
            0, 0, 1, 0, 0, 0, 255, 255, 127, 0, 0, 0, 0, 0, 128, 0, 41, 0, 2, 0, 0, 0, 0, 0, 128,
            0, 0, 0, 255, 255, 127, 0, 2, 0, 128, 0, 0, 0, 0, 0, 128, 1, 5, 0, 1, 0, 0, 0, 254,
            255, 127, 0, 0, 0, 0, 0, 128, 1, 43, 0, 1, 0, 0, 0, 255, 255, 127, 3, 0, 0, 1, 0, 128,
            1,
        ],
    )];
    for (track_data, data) in test_values {
        let result = encode_track_data(&track_data);
        assert_eq!(result, data);
    }
}
