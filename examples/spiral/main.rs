use hexgridspiral::HGSTile;
fn main() {
    // Make a HexGridSpiral Tile at index 3
    let a = HGSTile::make(3);
    // Compute its CubeCoordinates
    let b = a.cc();
    println!(
        "The Index {} has the tile center at cube-coordinate {}",
        a, b
    );

    // Let's spawn twenty hex-tiles around the origin
    let step_size = 1.;
    for i in 0..21 {
        let tile = HGSTile::make(i).cc().to_pixel((0., 0.), step_size.into());
        // Let's say you want to add tiles to your levelmap in the game UI:
        add_tile_to_user_interface(tile, i);
    }
}

fn add_tile_to_user_interface(position: (f64, f64), i: u64) {
    // I don't want to add a huge dependency to this example, so imagine a GUI.
    println!(
        "The GUI would position tile {} at pixel-position ({},{})",
        i, position.0, position.1
    );
}
