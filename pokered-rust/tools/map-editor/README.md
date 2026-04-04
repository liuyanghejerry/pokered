# Map Editor 使用说明

pokered-rust 地图编辑器，用于查看和编辑全部 248 张地图的数据。编辑器直接读写 `crates/pokered-data/maps/` 目录下的 JSON 文件，修改即时保存到磁盘。

## 启动

```bash
cd pokered-rust/tools/map-editor

# 安装依赖（首次使用）
npm install

# 启动开发服务器
npm run dev
```

启动后访问终端输出的地址（默认 `http://localhost:5173`）。编辑器会自动加载全部地图，侧边栏显示进度。

### 构建检查

```bash
# 仅类型检查
npx vue-tsc --noEmit

# 类型检查 + 生产构建
npm run build
```

## 界面布局

```
┌──────────────────┬──────────────────────────────────────┐
│                  │                                      │
│    侧边栏        │          工具栏                       │
│                  │  [View] [Edit Collision] [-] 2x [+]  │
│  地图选择         │──────────────────────────────────────│
│  搜索过滤         │                                      │
│  显示选项         │          地图画布                      │
│  保存/导航        │       （可缩放、可点击）                │
│  { } Script 按钮 │     显示连接箭头和目标地图名            │
│  Passable Tiles  │                                      │
│  实体详情         │──────────────────────────────────────│
│  Map Header编辑   │  脚本编辑器（可折叠面板）               │
│  (BGM/连接编辑)   │  [函数列表] | [CodeMirror 代码编辑区]  │
│  地图信息         │                                      │
│  图例            │                                      │
│                  │                                      │
└──────────────────┴──────────────────────────────────────┘
```

## 基本操作

### 选择地图

- **下拉菜单**：侧边栏顶部的下拉框，按地图 ID 排序列出全部地图
- **搜索过滤**：在搜索框输入地图名称关键字（如 `Pallet`、`Route`、`Gym`），下拉列表实时过滤
- **前后翻页**：点击 ◀ ▶ 按钮或按 `←` `→` 方向键切换上/下一张地图

### 缩放

工具栏中的 `-` / `+` 按钮调整缩放倍数（1x ~ 4x），默认 2x。

### 显示图层

侧边栏复选框控制画布上显示哪些图层：

| 选项 | 说明 |
|------|------|
| Show Tiles | 渲染 tileset 贴图（关闭时按 blockID 着色） |
| Show Collision | 叠加碰撞信息（绿色=可通行，红色=不可通行） |
| Show Warps | 显示传送点（蓝色方块） |
| Show Signs | 显示路牌/告示牌（黄色方块，标记 S） |
| Show NPCs | 显示 NPC（紫色=普通，红色=训练师，绿色=道具） |
| Show Coord Events | 显示坐标触发事件（橙色方块，标记 C） |
| Show Connections | 显示地图连接信息 |
| Show Grid | 显示 block 网格线 |

### 鼠标交互

- **悬停**：画布上移动鼠标显示 tooltip，包含当前 tile 坐标、block 坐标、blockID、tileID、碰撞状态，以及该位置的实体信息（Warp/Sign/NPC/Coord Event）
- **点击 Warp**：如果该 Warp 有目标地图，自动跳转到目标地图；否则选中该 Warp 显示详情
- **点击 Sign / NPC / Coord Event**：选中实体，侧边栏显示详细信息
- **点击空地**：取消选中

### 键盘快捷键

| 按键 | 功能 |
|------|------|
| `←` / `→` | 切换上/下一张地图 |
| `V` | 切换到 View 模式 |
| `E` | 切换到 Edit Collision 模式 |
| `Ctrl+S` / `Cmd+S` | 保存当前脚本（脚本编辑器打开时） |
| `Esc` | 取消选中当前实体 |

## 查看地图信息

### Map Info 面板

侧边栏底部 **Map Info** 区域显示当前地图的结构化数据：

- **基本属性**：名称、ID、尺寸（block 数）、tileset、音乐
- **Connections**：上下左右连接的相邻地图（可点击跳转）
- **Warps 列表**：所有传送点坐标及目标（可点击选中/高亮）
- **Signs 列表**：所有路牌坐标及 textId、绑定的脚本函数（可点击跳转到脚本编辑器）
- **Map Scripts**：地图级脚本函数列表（可点击跳转到脚本编辑器）
- **Coord Events**：坐标触发事件及其触发函数（可点击跳转到脚本编辑器）
- **NPCs 列表**：所有 NPC，含精灵名、坐标、训练师/道具信息、脚本绑定（可点击跳转到脚本编辑器）
- **Wild Pokemon**：野生宝可梦遭遇表（分 Red/Blue 版本，显示草地/水面遭遇率和前 5 种）

### Entity Detail 面板

点击地图上的实体后，侧边栏中间显示详细信息：

- **NPC**：精灵名（颜色标识类型）、坐标、移动方式/朝向、视野范围、训练师职业和编号、道具 ID、脚本绑定（可点击函数名打开脚本编辑器并跳转到定义）
- **Sign**：坐标、textId、脚本绑定（可点击函数名打开脚本编辑器并跳转到定义）
- **Warp**：坐标、目标地图名、目标 warp ID，以及"Go to"跳转按钮
- **Coord Event**：坐标、触发函数名（可点击打开脚本编辑器并跳转到定义）

### Passable Tiles 面板

显示当前地图 tileset 对应的所有可通行 tile ID。这些数据按 tileset 分组（非按地图），来源于 `collision.rs`。

## 编辑功能

### 编辑 BGM 音乐

侧边栏 **Map Header Editor** 区域提供 BGM 音乐下拉选择器：

- 显示当前地图的音乐名称
- 可选择全部 45 种音乐（从 `music.rs` 中定义的 `MusicId` 枚举）
- 选择后自动标记为"未保存"，点击 Save 按钮保存到 `map.json`

### 编辑地图连接关系

Map Header Editor 区域同时提供地图连接的可视化编辑功能：

- **四个方向**：North / South / West / East
- **每个连接显示**：目标地图名、偏移量（offset）
- **操作按钮**：
  - `Go` — 跳转到目标地图
  - `Edit` — 编辑连接（选择目标地图、设置偏移量）
  - `✕` — 删除连接
- **画布可视化**：开启 Show Connections 后，地图边缘显示绿色箭头指示连接方向和目标地图名

### 编辑脚本绑定

选中 NPC 或 Sign 后，Entity Detail 面板中有 **Script Function** 输入框。修改后按回车确认，编辑器会同步更新 `script_config.json` 中的绑定关系。

### 地图间导航

编辑器维护一个导航历史栈：

- 点击 Warp / Connection / "Go to" 按钮跳转到目标地图
- 侧边栏出现 **← Back** 按钮，可返回上一张地图

### 保存

点击侧边栏 **Save** 按钮（有未保存修改时可用）。保存操作会：

1. 将 `map.json` 写回 `crates/pokered-data/maps/{MapName}/map.json`（自动剥离运行时字段 `talk`）
2. 将 `script_config.json` 写回同目录

保存成功后状态栏显示 `Saved {MapName}`，未保存标记消失。

## 脚本编辑器

编辑器内嵌了基于 **CodeMirror 6** 的代码编辑器，可以直接查看和编辑每张地图的 `script.js` 事件脚本。

### 打开脚本编辑器

有三种方式打开脚本编辑器：

1. **侧边栏按钮**：点击 `{ } Script` 按钮，面板在地图画布下方展开
2. **点击函数名**：在 Map Info 面板或 Entity Detail 面板中，点击任何高亮的函数名（如 NPC 的 talk 函数、Sign 的 talk 函数、Map Script、Coord Event trigger），编辑器自动打开并跳转到该函数的定义行
3. **再次点击 `✕ Script` 按钮**关闭面板

### 编辑器界面

```
┌─────────────────────────────────────────────────────┐
│ ═══ 可拖拽调整高度的把手 ═══                          │
├──────────────────────────────────────────────────────┤
│ { } PalletTown/script.js  [Modified]    [Save] [✕] │
├──────────────┬──────────────────────────────────────┤
│ Functions(5) │                                      │
│              │   // CodeMirror 代码编辑区            │
│ ▸ enterMap   │   export async function enterMap() { │
│ ▸ talkOak    │     if (!game.getFlag("...")) {      │
│ ▸ signPallet │       ...                            │
│              │     }                                │
│              │   }                                  │
│              │                                      │
└──────────────┴──────────────────────────────────────┘
```

- **顶部栏**：显示当前地图的脚本文件名，修改状态标记（Modified），Save 按钮和关闭按钮
- **左侧函数列表**：自动解析 `script.js` 中的所有函数定义，显示函数名、行号、是否为 `export`。点击函数名跳转到对应行
- **右侧编辑区**：CodeMirror 6 编辑器，支持 JavaScript 语法高亮、行号、代码折叠、括号匹配、搜索替换、撤销/重做
- **可拖拽调整高度**：顶部有拖拽把手，可上下拖动调整脚本编辑器的面板高度

### 编辑器功能

| 功能 | 说明 |
|------|------|
| JavaScript 语法高亮 | 基于 `@codemirror/lang-javascript`，支持 ES module 语法 |
| 暗色主题 | One Dark 主题，与编辑器整体风格一致 |
| 行号 | 左侧显示行号 |
| 代码折叠 | 点击行号旁的折叠图标可折叠/展开代码块 |
| 括号匹配 | 自动高亮匹配的括号对 |
| 搜索替换 | Ctrl+F 搜索，Ctrl+H 替换 |
| 撤销/重做 | Ctrl+Z / Ctrl+Shift+Z |
| 快捷保存 | Ctrl+S / Cmd+S 保存脚本到磁盘 |
| 函数跳转 | 从侧边栏函数列表或实体面板点击函数名跳转 |

### 保存脚本

- 点击顶部栏的 **Save** 按钮，或按 `Ctrl+S` / `Cmd+S`
- 脚本内容直接写回 `crates/pokered-data/maps/{MapName}/script.js`
- 保存后 "Modified" 标记消失，状态栏显示 `Saved script for {MapName}`

### 切换地图时

当在侧边栏切换到另一张地图时，脚本编辑器自动加载新地图的 `script.js`。如果新地图没有 `script.js`，编辑器显示为空白。

## 数据目录结构

```
crates/pokered-data/maps/
├── PalletTown/
│   ├── map.json              # 地图主数据（header, warps, npcs, signs, text, wild）
│   ├── map.blk               # block 数据（二进制，每字节一个 blockID）
│   ├── script_config.json    # 脚本配置（mapScripts, npc/sign 绑定, coordEvents）
│   └── script.js             # 事件脚本（可通过内嵌脚本编辑器查看和编辑）
├── OaksLab/
│   ├── map.json
│   ├── map.blk
│   └── ...
└── ... (248 张地图)
```

## API 端点

编辑器通过 Vite 开发服务器内置的 API 访问数据：

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/maps` | 所有地图目录名列表 |
| GET | `/api/maps/:name/map.json` | 地图 JSON 数据 |
| GET | `/api/maps/:name/map.blk` | block 数据（JSON 字节数组） |
| GET | `/api/maps/:name/script_config.json` | 脚本配置 |
| GET | `/api/maps/:name/script.js` | 脚本文件（纯文本） |
| PUT | `/api/maps/:name/map.json` | 保存地图 JSON |
| PUT | `/api/maps/:name/script_config.json` | 保存脚本配置 |
| PUT | `/api/maps/:name/script.js` | 保存脚本文件 |
| GET | `/api/blocksets` | 所有 tileset 的 blockset 数据 |
| GET | `/api/passable-tiles` | 各 tileset 的可通行 tile 列表 |
| GET | `/gfx/tilesets/*.png` | tileset 贴图 |

## 图例

| 颜色 | 含义 |
|------|------|
| 🟢 绿色半透明 | 可通行 tile |
| 🔴 红色半透明 | 不可通行 tile |
| 🔵 蓝色 | Warp（传送点） |
| 🟡 黄色 | Sign（路牌） |
| 🔴 红色实心 | NPC — 训练师（标记 T） |
| 🟢 绿色实心 | NPC — 道具（标记 I） |
| 🟣 紫色 | NPC — 普通（标记 N） |
| 🟠 橙色 | Coord Event（标记 C） |
| 🟢 绿色箭头 | 地图连接（显示目标地图名和偏移量） |
| ⬜ 白/黄闪烁边框 | 当前选中的实体 |
