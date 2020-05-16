use bracket_geometry::prelude::Radians;

pub fn sphere_vertex<A:Into<Radians>>(altitude: f32, lat: A, lon: A) -> (f32, f32, f32) {
    let rlat = lat.into();
    let rlon = lon.into();
    let sinlat = f32::sin(rlat.0);
    let coslat = f32::cos(rlat.0);
    let sinlon = f32::sin(rlon.0);
    let coslon = f32::cos(rlon.0);
    (
        altitude * coslat * coslon,
        altitude * coslat * sinlon,
        altitude * sinlat,
    )
}
