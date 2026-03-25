# CF TempMail CLI

A command-line client for temporary email service.

临时邮箱命令行客户端。

## Installation

### Download from Release

```bash
# Linux x86_64
wget https://github.com/cf-tempmail/cf-tempmail-cli/releases/latest/download/cf-tempmail-x86_64-linux
chmod +x cf-tempmail-x86_64-linux
sudo mv cf-tempmail-x86_64-linux /usr/local/bin/cf-tempmail

# Debian/Ubuntu
wget https://github.com/cf-tempmail/cf-tempmail-cli/releases/latest/download/cf-tempmail_amd64.deb
sudo dpkg -i cf-tempmail_amd64.deb
```

### Build from Source

```bash
git clone https://github.com/cf-tempmail/cf-tempmail-cli
cd cf-tempmail-cli
cargo build --release
sudo mv target/release/cf-tempmail /usr/local/bin/
```

## Usage

### Configure Server

```bash
cf-tempmail config --baseurl https://your-mail-domain.com
```

### Create Temporary Email

```bash
# Random
cf-tempmail new

# With prefix
cf-tempmail new --prefix myname
```

### List Emails

```bash
cf-tempmail list
```

### Listen for New Emails

```bash
# Default 5s polling
cf-tempmail listen

# Custom interval
cf-tempmail listen --interval 10
```

### Delete Email

```bash
cf-tempmail delete
```

## Config File

Config file location:

- Linux: `~/.config/cf-tempmail/config.toml`
- macOS: `~/Library/Application Support/cf-tempmail/config.toml`
- Windows: `%APPDATA%\cf-tempmail\config.toml`

---

## 安装

### 从 Release 下载

```bash
# Linux x86_64
wget https://github.com/cf-tempmail/cf-tempmail-cli/releases/latest/download/cf-tempmail-x86_64-linux
chmod +x cf-tempmail-x86_64-linux
sudo mv cf-tempmail-x86_64-linux /usr/local/bin/cf-tempmail

# Debian/Ubuntu
wget https://github.com/cf-tempmail/cf-tempmail-cli/releases/latest/download/cf-tempmail_amd64.deb
sudo dpkg -i cf-tempmail_amd64.deb
```

### 从源码编译

```bash
git clone https://github.com/cf-tempmail/cf-tempmail-cli
cd cf-tempmail-cli
cargo build --release
sudo mv target/release/cf-tempmail /usr/local/bin/
```

## 使用

### 配置服务器地址

```bash
cf-tempmail config --baseurl https://your-mail-domain.com
```

### 创建临时邮箱

```bash
# 随机生成
cf-tempmail new

# 指定前缀
cf-tempmail new --prefix myname
```

### 查看邮件列表

```bash
cf-tempmail list
```

### 监听新邮件

```bash
# 默认 5 秒轮询
cf-tempmail listen

# 自定义轮询间隔
cf-tempmail listen --interval 10
```

### 删除邮箱

```bash
cf-tempmail delete
```

## 配置文件

配置文件存储位置：

- Linux: `~/.config/cf-tempmail/config.toml`
- macOS: `~/Library/Application Support/cf-tempmail/config.toml`
- Windows: `%APPDATA%\cf-tempmail\config.toml`

## License

MIT
