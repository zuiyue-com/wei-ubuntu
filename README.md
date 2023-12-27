# wei-ubuntu

- wsl --unregister wei-ubuntu
- mkdir wei-ubuntu
- wsl --import wei-ubuntu wei-ubuntu Ubuntu.tar.gz --version 2

- apt install net-tools curl wget
- 部署 wei-docker-linux -> /usr/bin/wei-docker-linux
- 部署 frpc -> /usr/bin/frpc
- 部署 frps -> /usr/bin/frps

- wsl --export wei-ubuntu Ubuntu.tar
- wsl gzip Ubuntu.tar
- 创建 docker 目录
- 复制 wsl_update_x64.msi, Ubuntu.tar.gz 到 docker 目录
- 导出 docker.torrent
- 使用 qbittorrent 制作种子文件

# 安装 rust

- sed -i 's/archive.ubuntu.com/mirrors.ustc.edu.cn/g' /etc/apt/sources.list
- apt update -y
- apt install build-essential pkg-config --fix-missing -y
- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- source "$HOME/.cargo/env"
