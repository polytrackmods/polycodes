struct PartInfo<'a> {
    checksum: &'a str,
    category: PartCategory,
    // part: PartId,
    models: &'a [&'a [&'a str]],
    env_colors: (),
    tiles: &'a [&'a [&'a [i32]]],
    // special_data: SpecialPartData,
    unknown4: [f64; 4],
}

enum PartCategory {
    Special = 0,
    Road = 1,
    RoadTurns = 2,
    RoadWide = 3,
    Plane = 4,
    Block = 5,
    WallTrack = 6,
    Pillar = 7,
    Sign = 8,
}
