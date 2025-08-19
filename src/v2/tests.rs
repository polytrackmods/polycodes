use super::*;

#[test]
fn track_decode() {
    let test_values = [
        (
            "v1nBwIreozarBQABAAAAAACAAAAAAACAAAEAAQAAAP7_fwAAAP__fwIAAAIAAAD-_38AAAADAIACAACAAAAAAQCAAyQAAQAAAAAAgAAAAP7_fwAqAAEAAAD9_38AAAACAIAA",
            Track {
                author: None,
                name: "Ireozar".to_string(),
                track_data: vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 1, 0, 1, 0, 0, 0, 254, 255,
                    127, 0, 0, 0, 255, 255, 127, 2, 0, 0, 2, 0, 0, 0, 254, 255, 127, 0, 0, 0, 3, 0,
                    128, 2, 0, 0, 128, 0, 0, 0, 1, 0, 128, 3, 36, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0,
                    0, 254, 255, 127, 0, 42, 0, 1, 0, 0, 0, 253, 255, 127, 0, 0, 0, 2, 0, 128, 0,
                ],
            },
        ),
        (
            "v1nBwTestingBQABAAAAAACAAAAAAACAACQAAQAAAAAAgAAAAP7_fwAqAAIAAAD9_38AAAABAIAA__9_AAAAAwCAAA",
            Track {
                author: None,
                name: "Testing".to_string(),
                track_data: vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 36, 0, 1, 0, 0, 0, 0, 0,
                    128, 0, 0, 0, 254, 255, 127, 0, 42, 0, 2, 0, 0, 0, 253, 255, 127, 0, 0, 0, 1,
                    0, 128, 0, 255, 255, 127, 0, 0, 0, 3, 0, 128, 0,
                ],
            },
        ),
    ];
    for (code, track) in test_values {
        let result = decode_track_code(code);
        assert_eq!(result, Some(track));
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
        TrackInfo {
            parts: vec![
                Part {
                    id: 0,
                    amount: 1,
                    blocks: vec![Block {
                        x: -1,
                        y: 0,
                        z: 0,
                        rotation: 0,
                    }],
                },
                Part {
                    id: 41,
                    amount: 2,
                    blocks: vec![
                        Block {
                            x: 0,
                            y: 0,
                            z: -1,
                            rotation: 0,
                        },
                        Block {
                            x: 2,
                            y: 0,
                            z: 0,
                            rotation: 1,
                        },
                    ],
                },
                Part {
                    id: 5,
                    amount: 1,
                    blocks: vec![Block {
                        x: -2,
                        y: 0,
                        z: 0,
                        rotation: 1,
                    }],
                },
                Part {
                    id: 43,
                    amount: 1,
                    blocks: vec![Block {
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
