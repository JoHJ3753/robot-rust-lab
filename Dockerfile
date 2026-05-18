# Rust 공식 이미지를 기반으로 사용합니다.
# slim 이미지는 용량이 작고 실습용으로 적합합니다.
FROM rust:1.95.0-slim

# 패키지 목록을 업데이트하고,
# 실습에 필요한 기본 도구를 설치합니다.
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    curl \
    git \
    vim \
    nano \
    tree \
    && rm -rf /var/lib/apt/lists/*

# 컨테이너 안에서 사용할 작업 폴더입니다.
WORKDIR /workspace

# Rust 버전과 Cargo 버전을 확인하는 기본 명령입니다.
CMD ["bash"]