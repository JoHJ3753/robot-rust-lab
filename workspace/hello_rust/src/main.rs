use tokio::time::{sleep, Duration};

async fn sensor_task() {
    for i in 1..=5 {
        println!("[Sensor Task] 센서 데이터 수집 {}", i);
        sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(sensor_task());

    println!("main은 sensor_task를 spawn했습니다.");

    handle.await.expect("sensor_task 실행 실패");

    println!("main 종료");
}