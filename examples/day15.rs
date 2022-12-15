use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day15.txt")?;


    let mut sensors: Vec<(i32, i32)> = Vec::new();
    let mut beacons: Vec<(i32, i32)> = Vec::new();
    let mut min_x: i32 = i32::MAX;
    let mut max_x: i32 = i32::MIN;

    for l in input.lines() {
        let l = l.strip_prefix("Sensor at x=").unwrap();
        let parts: Vec<&str> = l.split(": closest beacon is at x=").collect();
        for (i, part) in parts.iter().enumerate() {
            let coords: Vec<i32> = part.split(", y=").map(|n| n.parse::<i32>().unwrap()).collect();
            let coords = (coords[0], coords[1]);
            if i == 0 {
                sensors.push(coords);
            } else {
                beacons.push(coords);
            }

            min_x = min_x.min(coords.0);
            max_x = max_x.max(coords.0);
        }
    }

    assert_eq!(sensors.len(), beacons.len());

    // part 1
    let y_to_check = 2000000;
    let mut row: Vec<u8> = vec![0; (max_x - min_x + 1000000) as usize];
    for i in 0..sensors.len() {
        let sensor = &sensors[i];
        if sensor.1 == y_to_check {
            row[(sensor.0 - min_x) as usize] = 1;
        }

        let beacon = &beacons[i];
        if beacon.1 == y_to_check {
            row[(beacon.0 - min_x) as usize] = 2;
        }

        let distance_from_beacon = manhattan_distance(sensor, beacon);
        let min_distance_from_row = (sensor.1 - y_to_check).abs();

        if min_distance_from_row > distance_from_beacon {
            // too far away to matter
            continue;
        }

        let diff = distance_from_beacon - min_distance_from_row;

        for x in (sensor.0 - diff)..=(sensor.0 + diff) {
            let idx = (x - min_x) as usize;
            if row[idx] == 0 {
                row[idx] = 3;
            }
        }
    }

    let part1: usize = row.iter().filter(|&&v| v == 3).count();
    println!("part 1: {}", part1);

    Ok(())
}

fn manhattan_distance(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
