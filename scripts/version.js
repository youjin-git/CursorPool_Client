import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { execSync } from 'child_process';

// 获取 __dirname
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 读取命令行参数
const args = process.argv.slice(2);
const versionArg = args[0];

// 验证命令参数
if (!versionArg) {
  console.error('请提供版本号，例如: node version.js v1.8.0');
  process.exit(1);
}

// 读取当前 package.json
const packagePath = path.join(__dirname, '../package.json');
const pkg = JSON.parse(fs.readFileSync(packagePath, 'utf8'));

// 读取当前 Cargo.toml
const cargoPath = path.join(__dirname, '../src-tauri/Cargo.toml');
const cargoContent = fs.readFileSync(cargoPath, 'utf8');

// 处理版本参数，确保格式正确
let version = versionArg;
if (version.startsWith('v')) {
  version = version.substring(1);
}

// 验证版本格式
if (!/^\d+\.\d+\.\d+$/.test(version)) {
  console.error('版本号格式不正确，应为 x.x.x');
  process.exit(1);
}

try {
  // 更新 package.json
  pkg.version = version;
  fs.writeFileSync(packagePath, JSON.stringify(pkg, null, 2) + '\n');
  console.log(`package.json 版本已更新到 ${version}`);

  // 更新 Cargo.toml
  const newCargoContent = cargoContent.replace(
    /^version = "(.+?)"$/m,
    `version = "${version}"`
  );
  fs.writeFileSync(cargoPath, newCargoContent);
  console.log(`Cargo.toml 版本已更新到 ${version}`);

  // 执行 Git 操作
  execSync('git add .', { stdio: 'inherit' });
  execSync(`git commit -m "chore(release): bump version to v${version}"`, { stdio: 'inherit' });
  execSync('git push origin dev', { stdio: 'inherit' });
  execSync(`git tag v${version}`, { stdio: 'inherit' });
  execSync(`git push origin v${version}`, { stdio: 'inherit' });

  console.log(`\n已完成版本 v${version} 的发布流程！`);
} catch (error) {
  console.error('发布操作失败:', error.message);
  process.exit(1);
} 