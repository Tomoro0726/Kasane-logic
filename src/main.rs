use kasane_logic::{
    function::{
        ecef::{
            ECEF,
            ecef_to_point::ecef_to_point,
            point_to_ecef::{self},
        },
        line::line,
    },
    id::{DimensionRange, SpaceTimeId, coordinates::Point},
    set::SpaceTimeIdSet,
};
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let a = Point {
        latitude: 35.6809591,
        longitude: 139.7673068,
        altitude: 100.0,
    };

    let b = Point {
        latitude: 35.6291112,
        longitude: 139.7389313,
        altitude: 100.0,
    };

    let result = line(24, a, b);

    // ファイルを作成
    let file = File::create("voxels.txt")?;
    let mut writer = BufWriter::new(file);

    for ele in result {
        let line_str = format!("{},\n", ele);

        // コンソール出力

        // ファイル出力
        writer.write_all(line_str.as_bytes())?;
    }

    // バッファをフラッシュ
    writer.flush()?;

    Ok(())
}
