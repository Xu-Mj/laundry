# cleanwave

## 运行说明

```bash
# 进入项目目录
cd cleanwave

# 安装yarn
npm install -g yarn

# 安装依赖
yarn --registry=https://registry.npmmirror.com
yarn install

# 启动项目
cargo tauri dev

# 打包
cargo tauri build
