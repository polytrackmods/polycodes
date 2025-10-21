use super::*;

#[test]
fn track_decode() {
    let test_values = [
        (
            r#"{"version":0,"name":"Ireozar","track":"{\"version\":0,\"parts\":{\"0\":[1,0,2,3],\"1\":[-2,0,0,2],\"4\":[2,0,0,3,1,0,-1,3],\"5\":[0,0,0,0]}}"}"#,
            Track {
                name: "Ireozar".to_string(),
                version: 0,
                track: TrackInfo {
                    version: 0,
                    parts: vec![
                        Part {
                            id: 0,
                            blocks: vec![Block {
                                x: 1,
                                y: 0,
                                z: 2,
                                rot: 3,
                            }],
                        },
                        Part {
                            id: 1,
                            blocks: vec![Block {
                                x: -2,
                                y: 0,
                                z: 0,
                                rot: 2,
                            }],
                        },
                        Part {
                            id: 4,
                            blocks: vec![
                                Block {
                                    x: 2,
                                    y: 0,
                                    z: 0,
                                    rot: 3,
                                },
                                Block {
                                    x: 1,
                                    y: 0,
                                    z: -1,
                                    rot: 3,
                                },
                            ],
                        },
                        Part {
                            id: 5,
                            blocks: vec![Block {
                                x: 0,
                                y: 0,
                                z: 0,
                                rot: 0,
                            }],
                        },
                    ],
                },
            },
        ),
        (
            r#"{"version":0,"name":"Testing","track":"{\"version\":0,\"parts\":{\"4\":[2,0,0,3,1,0,-1,0],\"5\":[0,0,0,0],\"23\":[-2,0,0,3,1,0,2,3]}}"}"#,
            Track {
                name: "Testing".to_string(),
                version: 0,
                track: TrackInfo {
                    version: 0,
                    parts: vec![
                        Part {
                            id: 4,
                            blocks: vec![
                                Block {
                                    x: 2,
                                    y: 0,
                                    z: 0,
                                    rot: 3,
                                },
                                Block {
                                    x: 1,
                                    y: 0,
                                    z: -1,
                                    rot: 0,
                                },
                            ],
                        },
                        Part {
                            id: 5,
                            blocks: vec![Block {
                                x: 0,
                                y: 0,
                                z: 0,
                                rot: 0,
                            }],
                        },
                        Part {
                            id: 23,
                            blocks: vec![
                                Block {
                                    x: -2,
                                    y: 0,
                                    z: 0,
                                    rot: 3,
                                },
                                Block {
                                    x: 1,
                                    y: 0,
                                    z: 2,
                                    rot: 3,
                                },
                            ],
                        },
                    ],
                },
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
                name: "Ireozar".to_string(),
                version: 0,
                track: TrackInfo {
                    version: 0,
                    parts: vec![
                        Part {
                            id: 0,
                            blocks: vec![Block {
                                x: 1,
                                y: 0,
                                z: 2,
                                rot: 3,
                            }],
                        },
                        Part {
                            id: 1,
                            blocks: vec![Block {
                                x: -2,
                                y: 0,
                                z: 0,
                                rot: 2,
                            }],
                        },
                        Part {
                            id: 4,
                            blocks: vec![
                                Block {
                                    x: 2,
                                    y: 0,
                                    z: 0,
                                    rot: 3,
                                },
                                Block {
                                    x: 1,
                                    y: 0,
                                    z: -1,
                                    rot: 3,
                                },
                            ],
                        },
                        Part {
                            id: 5,
                            blocks: vec![Block {
                                x: 0,
                                y: 0,
                                z: 0,
                                rot: 0,
                            }],
                        },
                    ],
                },
            },
            r#"{"version":0,"name":"Ireozar","track":"{\"version\":0,\"parts\":{\"0\":[1,0,2,3],\"1\":[-2,0,0,2],\"4\":[2,0,0,3,1,0,-1,3],\"5\":[0,0,0,0]}}"}"#,
        ),
        (
            Track {
                name: "Testing".to_string(),
                version: 0,
                track: TrackInfo {
                    version: 0,
                    parts: vec![
                        Part {
                            id: 4,
                            blocks: vec![
                                Block {
                                    x: 2,
                                    y: 0,
                                    z: 0,
                                    rot: 3,
                                },
                                Block {
                                    x: 1,
                                    y: 0,
                                    z: -1,
                                    rot: 0,
                                },
                            ],
                        },
                        Part {
                            id: 5,
                            blocks: vec![Block {
                                x: 0,
                                y: 0,
                                z: 0,
                                rot: 0,
                            }],
                        },
                        Part {
                            id: 23,
                            blocks: vec![
                                Block {
                                    x: -2,
                                    y: 0,
                                    z: 0,
                                    rot: 3,
                                },
                                Block {
                                    x: 1,
                                    y: 0,
                                    z: 2,
                                    rot: 3,
                                },
                            ],
                        },
                    ],
                },
            },
            r#"{"version":0,"name":"Testing","track":"{\"version\":0,\"parts\":{\"4\":[2,0,0,3,1,0,-1,0],\"5\":[0,0,0,0],\"23\":[-2,0,0,3,1,0,2,3]}}"}"#,
        ),
    ];
    for (track, code) in test_values {
        let result = encode_track_code(track);
        assert_eq!(result, code);
    }
}
