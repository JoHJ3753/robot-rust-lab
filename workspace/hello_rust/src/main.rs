const SAFETY_DISTANCE: f64 = 0.7;

fn is_obstacle_too_close(distance: f64) -> bool {
    distance < SAFETY_DISTANCE
}

fn main() {
    let lidar_distance: f64 = 0.45;

    let danger = is_obstacle_too_close(lidar_distance);

    println!("라이다 거리: {} m", lidar_distance);
    println!("장애물이 너무 가까운가? {}", danger);
}