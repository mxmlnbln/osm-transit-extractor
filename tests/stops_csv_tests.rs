use tempdir::TempDir;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[test]
pub fn osm_fixture_stoppoints() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops = osm_transit_extractor::get_stop_points_from_osm(&mut parsed_pbf);
    assert_eq!(stops[0].id, "node:260743996");
    assert_eq!(stops.len(), 77);
}

#[test]
pub fn osm_fixture_stoppoints_categorization() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let mut stop_points = osm_transit_extractor::get_stop_points_from_osm(&mut parsed_pbf);
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    osm_transit_extractor::update_stop_points_type(&mut stop_points, &routes);
    let stop_points_unknown: Vec<&osm_transit_extractor::StopPoint> = stop_points
        .iter()
        .filter(|s| s.stop_point_type == osm_transit_extractor::StopPointType::Unknown)
        .collect();
    let stop_points_platform: Vec<&osm_transit_extractor::StopPoint> = stop_points
        .iter()
        .filter(|s| s.stop_point_type == osm_transit_extractor::StopPointType::Platform)
        .collect();
    assert_eq!(stop_points_unknown.len(), 52);
    assert_eq!(stop_points_platform.len(), 12);
}

#[test]
pub fn osm_fixture_stopareas() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stop_areas = osm_transit_extractor::get_stop_areas_from_osm(&mut parsed_pbf);
    assert_eq!(stop_areas.len(), 1);
}

#[test]
pub fn osm_fixture_routes_count() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    assert_eq!(routes.len(), 3);
    for r in routes {
        if r.id == "relation:1257168" {
            assert_eq!(r.ordered_route_points.len(), 34);
            assert_eq!(r.ordered_route_points[0].stop_point_id, "node:3270784465");
            assert_eq!(r.ordered_route_points[30].stop_point_id, "node:1577028157");
        }
    }
}

#[test]
pub fn osm_fixture_lines_count() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].routes_id.len(), 2);
}

#[test]
pub fn osm_fixture_routes_tags() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    for r in routes {
        match r.id.as_ref() {
            "relation:1257168" => {
                assert_eq!(r.colour, format!(""));
                assert_eq!(r.operator, format!("RATP"));
                assert_eq!(r.network, format!("RATP"));
                assert_eq!(r.mode, format!("bus"));
                assert_eq!(r.code, format!("57"));
                assert_eq!(r.origin, format!("Arcueil - Laplace"));
            }
            "relation:1257174" => {
                assert_eq!(r.destination, format!("Arcueil - Laplace"));
            }
            _ => {}
        }
    }
}

#[test]
pub fn osm_fixture_lines_tags() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    assert_eq!(lines[0].colour, format!("#9C983A"));
    assert_eq!(lines[0].operator, format!("RATP"));
    assert_eq!(lines[0].network, format!("RATP"));
    assert_eq!(lines[0].mode, format!("bus"));
    assert_eq!(lines[0].code, format!("57"));
}

#[test]
pub fn osm_fixture_stoppoints_csv() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops = osm_transit_extractor::get_stop_points_from_osm(&mut parsed_pbf);
    let tmp_dir = TempDir::new("osm_transit_extractor").expect("create temp dir");
    osm_transit_extractor::write_stop_points_to_csv(&stops, &tmp_dir, false);
    let file_path = tmp_dir
        .path()
        .join("osm-transit-extractor_stop_points.csv");
    assert!(file_path.is_file());
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    assert_eq!(78, reader.lines().count());    

    tmp_dir.close().expect("delete temp dir");
}

#[test]
pub fn osm_fixture_stopareas_stoppoints_csv() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stop_areas = osm_transit_extractor::get_stop_areas_from_osm(&mut parsed_pbf);
    let tmp_dir = TempDir::new("osm_transit_extractor").expect("create temp dir");
    osm_transit_extractor::write_stop_areas_stop_point_to_csv(&stop_areas, &tmp_dir);
    let file_path = tmp_dir
        .path()
        .join("osm-transit-extractor_stop_areas_stop_point.csv");
    assert!(file_path.is_file());
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    assert_eq!(3, reader.lines().count());    
    tmp_dir.close().expect("delete temp dir");
}

#[test]
pub fn osm_fixture_routes_csv() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    let tmp_dir = TempDir::new("osm_transit_extractor").expect("create temp dir");
    osm_transit_extractor::write_routes_to_csv(routes, &tmp_dir, true);
    let file_path = tmp_dir
        .path()
        .join("osm-transit-extractor_routes.csv");
    assert!(file_path.is_file());
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    assert_eq!(4, reader.lines().count());    
    tmp_dir.close().expect("delete temp dir");
}

#[test]
pub fn osm_fixture_lines_csv() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    let tmp_dir = TempDir::new("osm_transit_extractor").expect("create temp dir");
    osm_transit_extractor::write_lines_to_csv(lines, &tmp_dir, false);
    let file_path = tmp_dir
        .path()
        .join("osm-transit-extractor_lines.csv");
    assert!(file_path.is_file());
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    assert_eq!(2, reader.lines().count());    

    tmp_dir.close().expect("delete temp dir");
}
