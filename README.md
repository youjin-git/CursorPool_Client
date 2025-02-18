# Cursor Pool

Cursor Pool 是一个基于 Tauri + Vue3 + TypeScript 开发的桌面应用程序，用于管理和优化 Cursor 账户使用。

## 功能特性

- 账户管理
  - 账户切换
  - 机器码重置
  - 一键切换（账户+机器码）
  - 使用量统计

- 用户系统
  - 会员等级
  - 激活码兑换
  - 密码修改
  - 账户状态监控

- 使用统计
  - GPT-4 使用量
  - GPT-3.5 使用量
  - 账户使用记录
  - 历史操作记录

- 其他特性
  - 多语言支持
  - 深色/浅色主题
  - 自动更新检查
  - 操作历史记录

## 开发环境

### 推荐的 IDE 配置

- [VS Code](https://code.visualstudio.com/) 
- [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### 环境要求

- Node.js >= 16
- Rust >= 1.70
- 系统要求
  - Windows 10/11
  - macOS 10.15+
  - Linux (with WebKit2GTK installed)

## 开发指南

1. 安装依赖
```bash
npm install
```

2. 启动开发服务器
```bash
npm run tauri dev
```

3. 构建发布版本
```bash
npm run tauri build
```

## 项目结构

```
src/
├── api/          # API 接口定义
├── components/   # 通用组件
├── locales/      # 国际化文件
├── stores/       # 状态管理
├── types/        # TypeScript 类型定义
├── utils/        # 工具函数
└── views/        # 页面组件

src-tauri/
├── src/
│   ├── api/         # Rust API 实现
│   ├── auth/        # 认证相关
│   ├── cursor_reset/# Cursor 重置功能
│   └── utils/       # 工具函数
```

## 技术栈

- 前端
  - Vue 3
  - TypeScript
  - Naive UI
  - Vue Router
  - Pinia

- 后端
  - Rust
  - Tauri
  - SQLite

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

[MIT License](LICENSE)

## 版本历史

- v0.1.0
  - 初始版本发布
  - 基础功能实现
  - 多语言支持
