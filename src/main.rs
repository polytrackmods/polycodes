use polycodes::codes::*;

fn main() {
    let code = "PolyTrack14p9i0XLjMgKCNYfUUyV6tLGZZx5HaVlVoreJTniKyMdy9Id01UTzy0ACiwgIjyJP9x9Izwryz0xgdO8Iz2L3j0VnDJ30927Q8tcfGorB6ujpHlfLo8kBA2GzRx";

    let track_data = decode_track_code(code).unwrap().track_data;
    let track_info = decode_track_data(&track_data).unwrap();

    println!("{track_info:#?}");
}
