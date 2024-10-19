# 使用官方Rust镜像作为基础镜像
FROM rust:1.80.1

# 设置工作目录
WORKDIR /app

# 复制项目文件到容器中
COPY . .

# 安装必要的依赖
# RUN apt-get update && apt-get install -y lld clang
ENV APP_ENVIRONMENT=live
# 构建项目
RUN cargo build --release

# 设置启动命令
CMD ["./target/release/mail-server"]