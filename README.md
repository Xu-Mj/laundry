# CleanWave

CleanWave 是一个面向干洗店的管理系统客户端，基于 Vue3、Vite 和 Tauri 构建，支持跨平台桌面应用开发。项目集成了前后端分离、现代前端工程化等特性，适用于干洗店业务的高效管理。

> **注意：目前仅客户端代码开源，服务端尚未开源，因此即使安装客户端也无法正式使用。**
>
> 如需自行部署服务端或获取更多信息，请联系我们。

## 主要特性

- ⚡ 基于 Vue3 + Vite，开发体验极佳
- 🖥️ 使用 Tauri 实现跨平台桌面应用（支持 Windows、macOS、Linux）
- 💡 支持 JavaScript
- 🔒 内置权限管理与用户认证
- 🎨 丰富的 UI 组件与主题配置
- 🚀 一键打包与自动更新支持

## 技术栈

- [Vue 3](https://vuejs.org/)
- [Vite](https://vitejs.dev/)
- [Tauri](https://tauri.app/)
- [JavaScript](https://developer.mozilla.org/docs/Web/JavaScript)
- [Yarn](https://yarnpkg.com/)

## 安装与运行

```bash
# 克隆项目
git clone <your-repo-url>
cd cleanwave

# 安装 yarn（如未安装）
npm install -g yarn

# 安装 Tauri CLI（如未安装）
cargo install tauri-cli

# 安装 Rust（如未安装）
# 推荐使用 rustup 安装，详见 https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装依赖
yarn install --registry=https://registry.npmmirror.com

# 启动开发环境
cargo tauri dev
```

## 打包发布

```bash
# 构建并打包桌面应用
cargo tauri build
```

打包产物位于 `src-tauri/target/release/bundle` 目录下。

## 目录结构

```text
cleanwave/
├── src/                # 前端源码（Vue3）
├── src-tauri/          # Tauri 配置与后端代码
├── public/             # 公共资源
├── package.json        # 前端依赖配置
├── tauri.conf.json     # Tauri 配置文件
└── ...
```

## 常见问题

- **依赖安装缓慢或失败？**  
  推荐使用中国镜像源：`yarn install --registry=https://registry.npmmirror.com`
- **Tauri 构建失败？**  
  请确保已正确安装 Rust 环境和相关依赖，详见 [Tauri 官方文档](https://tauri.app/v1/guides/getting-started/prerequisites/)

## 参考链接

- [Tauri 官方文档](https://tauri.app/)
- [Vue3 官方文档](https://vuejs.org/)
- [Vite 官方文档](https://vitejs.dev/)

---
如有问题欢迎提 Issue 或 PR 贡献！
