import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// 获取 __dirname
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 读取命令行参数
const args = process.argv.slice(2);
const [major, minor] = args;

if (!major || !minor) {
  console.error('请提供主版本号和次版本号，例如: node version.js 1 2');
  process.exit(1);
}

// 读取当前 package.json
const packagePath = path.join(__dirname, '../package.json');
const pkg = JSON.parse(fs.readFileSync(packagePath, 'utf8'));

// 获取当前的 bugfix 版本号并加1
const currentVersion = pkg.version;
const [currentMajor, currentMinor, currentPatch] = currentVersion.split('.').map(Number);

// 构造第三个数字的逻辑
const newPatch = currentMajor === Number(major) && currentMinor === Number(minor) 
  ? currentPatch + 1  // 如果主版本号和次版本号没变，bugfix版本号加1
  : 0;                // 如果主版本号或次版本号变了，bugfix版本号重置为0

// 构建新版本号
const newVersion = `${major}.${minor}.${newPatch}`;

// 更新 package.json
pkg.version = newVersion;
fs.writeFileSync(packagePath, JSON.stringify(pkg, null, 2) + '\n');

console.log(`版本已更新到 ${newVersion}`); 