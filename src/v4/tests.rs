use super::*;

#[test]
fn full_convert() {
    let test_values = [
        (
            "v3KAJJXZvpXYyB4pdPLGcDADCDD0JqFWkOCdfRGcQhYjE5RkOfsfCD8i7ncmjDbfgfaq0SGVwTZ6q5KHO3E2vYXekBaKZqGc7GIuEBd",
            "43996cf8e0dd55ed9216a7b8d416fb544f231b76485044d3a3d01bd263936863",
        ),
        (
            "v3KAUV2c0lmbnB4p9YlBGZAEoB4kg4ffff1DhPjqATeMxADsAkP7g5fvff6ZCkKbgRTAznJwyDiPDMwGYReHwqAo5wIAAbO1h6A",
            "57f1dd2abac3475e6013ef2cc3a93a706dd761594aac7a59b363b67b6a389ca7",
        ),
    ];
    for (code, id) in test_values {
        let result = export_to_id(code);
        assert_eq!(result, Some(id.to_string()));
    }
}

#[test]
fn track_decode() {
    let test_values = [
        (
            "v3KAJJXZvpXYyB4pdPLGcDADCDD0JqFWkOCdfRGcQhYjE5RkOfsfCD8i7ncmjDbfgfaq0SGVwTZ6q5KHO3E2vYXekBaKZqGc7GIuEBd",
            Track {
                author: None,
                name: "Ireozar".to_string(),
                track_data: vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 0, 0, 1, 0, 0, 0, 255, 255,
                    127, 0, 0, 0, 0, 0, 128, 1, 36, 0, 2, 0, 0, 0, 0, 0, 128, 0, 0, 0, 255, 255,
                    127, 1, 0, 0, 128, 2, 0, 0, 4, 0, 128, 1, 7, 0, 1, 0, 0, 0, 253, 255, 127, 2,
                    0, 0, 1, 0, 128, 1, 52, 0, 1, 0, 0, 0, 2, 0, 128, 2, 0, 0, 1, 0, 128, 1, 0, 0,
                    6, 0, 1, 0, 0, 0, 252, 255, 127, 2, 0, 0, 255, 255, 127, 1,
                ],
            },
        ),
        (
            "v3KAUV2c0lmbnB4p9YlBGZAEoB4kg4ffff1DhPjqATeMxADsAkP7g5fvff6ZCkKbgRTAznJwyDiPDMwGYReHwqAo5wIAAbO1h6A",
            Track {
                author: None,
                name: "Testing".to_string(),
                track_data: vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 0, 0, 1, 0, 0, 0, 255, 255,
                    127, 0, 0, 0, 0, 0, 128, 1, 36, 0, 1, 0, 0, 0, 0, 0, 128, 2, 0, 0, 4, 0, 128,
                    1, 7, 0, 1, 0, 0, 0, 253, 255, 127, 2, 0, 0, 1, 0, 128, 1, 52, 0, 1, 0, 0, 0,
                    2, 0, 128, 2, 0, 0, 1, 0, 128, 1, 0, 0, 6, 0, 1, 0, 0, 0, 252, 255, 127, 2, 0,
                    0, 255, 255, 127, 1,
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
fn track_encode() {
    let test_values = [
        (
            Track {
                author: None,
                name: "Ireozar".to_string(),
                track_data: vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 0, 0, 1, 0, 0, 0, 255, 255,
                    127, 0, 0, 0, 0, 0, 128, 1, 36, 0, 2, 0, 0, 0, 0, 0, 128, 0, 0, 0, 255, 255,
                    127, 1, 0, 0, 128, 2, 0, 0, 4, 0, 128, 1, 7, 0, 1, 0, 0, 0, 253, 255, 127, 2,
                    0, 0, 1, 0, 128, 1, 52, 0, 1, 0, 0, 0, 2, 0, 128, 2, 0, 0, 1, 0, 128, 1, 0, 0,
                    6, 0, 1, 0, 0, 0, 252, 255, 127, 2, 0, 0, 255, 255, 127, 1,
                ],
            },
            "v3KAJJXZvpXYyB4pdPLGcDADCDD0JqFWkOCdfRGcQhYjE5RkOfsfCD8i7ncmjDbfgfaq0SGVwTZ6q5KHO3E2vYXekBaKZqGc7GIuEBd",
        ),
        (
            Track {
                author: None,
                name: "Testing".to_string(),
                track_data: vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 0, 0, 1, 0, 0, 0, 255, 255,
                    127, 0, 0, 0, 0, 0, 128, 1, 36, 0, 1, 0, 0, 0, 0, 0, 128, 2, 0, 0, 4, 0, 128,
                    1, 7, 0, 1, 0, 0, 0, 253, 255, 127, 2, 0, 0, 1, 0, 128, 1, 52, 0, 1, 0, 0, 0,
                    2, 0, 128, 2, 0, 0, 1, 0, 128, 1, 0, 0, 6, 0, 1, 0, 0, 0, 252, 255, 127, 2, 0,
                    0, 255, 255, 127, 1,
                ],
            },
            "v3KAUV2c0lmbnB4p9YlBGZAEoB4kg4ffff1DhPjqATeMxADsAkP7g5fvff6ZCkKbgRTAznJwyDiPDMwGYReHwqAo5wIAAbO1h6A",
        ),
    ];
    for (track, code) in test_values {
        let result = encode_track_code(&track);
        assert_eq!(result, Some(code.to_string()));
    }
}

#[test]
fn data_decode() {
    let test_values = [(
        vec![
            5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 0, 0, 1, 0, 0, 0, 255, 255, 127, 0,
            0, 0, 0, 0, 128, 1, 36, 0, 2, 0, 0, 0, 0, 0, 128, 0, 0, 0, 255, 255, 127, 1, 0, 0, 128,
            2, 0, 0, 4, 0, 128, 1, 7, 0, 1, 0, 0, 0, 253, 255, 127, 2, 0, 0, 1, 0, 128, 1, 52, 0,
            1, 0, 0, 0, 2, 0, 128, 2, 0, 0, 1, 0, 128, 1, 0, 0, 6, 0, 1, 0, 0, 0, 252, 255, 127, 2,
            0, 0, 255, 255, 127, 1,
        ],
        TrackInfo {
            parts: vec![
                Part {
                    id: 5,
                    amount: 1,
                    blocks: vec![Block {
                        x: 0,
                        y: 0,
                        z: 0,
                        rotation: 0,
                        cp_order: None,
                    }],
                },
                Part {
                    id: 0,
                    amount: 1,
                    blocks: vec![Block {
                        x: -1,
                        y: 0,
                        z: 0,
                        rotation: 1,
                        cp_order: None,
                    }],
                },
                Part {
                    id: 36,
                    amount: 2,
                    blocks: vec![
                        Block {
                            x: 0,
                            y: 0,
                            z: -1,
                            rotation: 1,
                            cp_order: None,
                        },
                        Block {
                            x: 0,
                            y: 2,
                            z: 4,
                            rotation: 1,
                            cp_order: None,
                        },
                    ],
                },
                Part {
                    id: 7,
                    amount: 1,
                    blocks: vec![Block {
                        x: -3,
                        y: 2,
                        z: 1,
                        rotation: 1,
                        cp_order: None,
                    }],
                },
                Part {
                    id: 52,
                    amount: 1,
                    blocks: vec![Block {
                        x: 2,
                        y: 2,
                        z: 1,
                        rotation: 1,
                        cp_order: Some(0),
                    }],
                },
                Part {
                    id: 6,
                    amount: 1,
                    blocks: vec![Block {
                        x: -4,
                        y: 2,
                        z: -1,
                        rotation: 1,
                        cp_order: None,
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
        TrackInfo {
            parts: vec![
                Part {
                    id: 5,
                    amount: 1,
                    blocks: vec![Block {
                        x: 0,
                        y: 0,
                        z: 0,
                        rotation: 0,
                        cp_order: None,
                    }],
                },
                Part {
                    id: 0,
                    amount: 1,
                    blocks: vec![Block {
                        x: -1,
                        y: 0,
                        z: 0,
                        rotation: 1,
                        cp_order: None,
                    }],
                },
                Part {
                    id: 36,
                    amount: 2,
                    blocks: vec![
                        Block {
                            x: 0,
                            y: 0,
                            z: -1,
                            rotation: 1,
                            cp_order: None,
                        },
                        Block {
                            x: 0,
                            y: 2,
                            z: 4,
                            rotation: 1,
                            cp_order: None,
                        },
                    ],
                },
                Part {
                    id: 7,
                    amount: 1,
                    blocks: vec![Block {
                        x: -3,
                        y: 2,
                        z: 1,
                        rotation: 1,
                        cp_order: None,
                    }],
                },
                Part {
                    id: 52,
                    amount: 1,
                    blocks: vec![Block {
                        x: 2,
                        y: 2,
                        z: 1,
                        rotation: 1,
                        cp_order: Some(0),
                    }],
                },
                Part {
                    id: 6,
                    amount: 1,
                    blocks: vec![Block {
                        x: -4,
                        y: 2,
                        z: -1,
                        rotation: 1,
                        cp_order: None,
                    }],
                },
            ],
        },
        vec![
            5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 0, 0, 1, 0, 0, 0, 255, 255, 127, 0,
            0, 0, 0, 0, 128, 1, 36, 0, 2, 0, 0, 0, 0, 0, 128, 0, 0, 0, 255, 255, 127, 1, 0, 0, 128,
            2, 0, 0, 4, 0, 128, 1, 7, 0, 1, 0, 0, 0, 253, 255, 127, 2, 0, 0, 1, 0, 128, 1, 52, 0,
            1, 0, 0, 0, 2, 0, 128, 2, 0, 0, 1, 0, 128, 1, 0, 0, 6, 0, 1, 0, 0, 0, 252, 255, 127, 2,
            0, 0, 255, 255, 127, 1,
        ],
    )];
    for (track_data, data) in test_values {
        let result = encode_track_data(&track_data);
        assert_eq!(result, Some(data));
    }
}
