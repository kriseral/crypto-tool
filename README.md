# CryptoTool - 文本/文件加密工具

基于 **Tauri 2 + Vue 3 + Rust** 构建的本地加密工具，体积小、性能好、无联网功能。

## 功能特性

- 支持 **文本加密/解密** 和 **文件加密/解密**
- 多种加密算法可选
- 暗色主题 GUI 界面
- 无联网，纯本地运行

## 支持的算法

| 类型 | 算法 |
|------|------|
| 编码 | Base64, Base64URL, Hex |
| 对称加密 | AES-256-GCM, DES, 3DES, XOR |
| 哈希 | MD5, SHA-256, SHA-512, Blake2b, Blake2s |
| 校验 | CRC32 |

## 截图

<img src="screenshot.png" width="600">

## 开发环境要求

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) (rustc 1.77+)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (C++ 桌面开发)

## 快速开始

```bash
# 克隆仓库
git clone https://github.com/kriseral/crypto-tool.git
cd crypto-tool

# 安装前端依赖
npm install

# 开发模式运行
npx tauri dev
```

## 构建发布版

```bash
# 构建前端 + 打包
npx tauri build
```

生成的可执行文件位于 `src-tauri/target/release/crypto-tool.exe`

## 项目结构

```
crypto-tool/
├── index.html                 # 入口页面
├── package.json               # 前端依赖
├── vite.config.js             # Vite 配置
├── src/
│   ├── main.js                # Vue 入口
│   ├── App.vue                # 主界面组件
│   └── style.css
└── src-tauri/
    ├── Cargo.toml             # Rust 依赖
    ├── tauri.conf.json        # Tauri 配置
    ├── build.rs
    ├── icons/
    └── src/
        ├── main.rs            # 程序入口
        ├── lib.rs             # Tauri 命令定义
        └── crypto/
            ├── base64_codec.rs  # Base64 / Base64URL / Hex
            ├── aes_codec.rs     # AES-256-GCM
            ├── des_codec.rs     # DES / 3DES
            ├── xor_codec.rs     # XOR
            ├── hash.rs          # MD5 / SHA / Blake2
            └── crc.rs           # CRC32
```

## 技术栈

- [Tauri 2](https://v2.tauri.app/) - 跨平台桌面应用框架
- [Vue 3](https://vuejs.org/) - 前端框架
- [Vite](https://vitejs.dev/) - 构建工具
- [Rust](https://www.rust-lang.org/) - 后端语言
- [RustCrypto](https://github.com/RustCrypto) - 加密算法库

## License

MIT

## 更新日志

### v1.1.1 (2026-08-25)
- 修复版本号和文件名大小写不统一的问题

### v1.1.0 (2026-08-25)
- 新增重置按钮，一键清空输入/输出/密钥
- 优化界面布局

### v1.0.0 (2026-08-25)
- 首次发布
- 支持文本加密/解密和文件加密/解密
- 支持 Base64、Base64URL、Hex 编码
- 支持 AES-256-GCM、DES、3DES、XOR 对称加密
- 支持 MD5、SHA-256、SHA-512、Blake2b、Blake2s 哈希
- 支持 CRC32 校验
- 暗色主题 GUI 界面
- 无联网，纯本地运行
