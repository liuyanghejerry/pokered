# 迁移计划：Toggle Objects → Script Config

## 背景

当前存在两套并行的 NPC 隐藏/显示机制，导致架构冲突：

| 机制 | 存储位置 | 可编辑性 | 与脚本集成 |
|------|----------|----------|------------|
| `toggleable_objects.rs` | Rust 端硬编码常量 | ❌ 无法编辑 | ❌ 脚本无法感知 |
| `script_config.json` + `script.js` | JSON + JS 文件 | ✅ map-editor 可编辑 | ✅ 脚本 API 调用 |

**问题表现**：
- Pallet Town 和 Oak's Lab 出现"好几个 Oak"（应该默认隐藏的 NPC 都显示了）
- `map_data_loading.rs` 获取 toggle 数据后完全未使用
- 脚本作者无法通过 map-editor 查看或编辑 NPC 的初始可见性

## 目标

将 NPC 的 toggle 状态（默认隐藏/显示）迁移到 `script_config.json`，实现：

1. ✅ 所有 NPC 初始可见性在 JSON 中声明
2. ✅ map-editor 可视化编辑
3. ✅ 脚本可通过命名常量引用
4. ✅ 废弃 `toggleable_objects.rs` 的硬编码数据

---

## 新设计

### JSON Schema 扩展

`script_config.json` 的 `npcs` 数组扩展：

```json
{
  "mapScripts": ["palletTownDefault", "palletTownOakHeyWait", ...],
  "npcs": [
    { 
      "id": 1, 
      "talk": "talkOak",
      "toggleId": "PALLET_TOWN_OAK",
      "defaultHidden": true
    },
    { 
      "id": 2, 
      "talk": "talkGirl"
    },
    { 
      "id": 3, 
      "talk": "talkFisher"
    }
  ],
  "signs": [...],
  "coordEvents": [...]
}
```

**新增字段**：

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `toggleId` | string | 否 | 命名标识符，供脚本 `showObject`/`hideObject` 使用 |
| `defaultHidden` | boolean | 否 | 默认 `false`。为 `true` 时 NPC 初始不渲染 |

### 脚本 API

```javascript
// 方式1：使用命名 toggleId（推荐）
await game.showObject("PALLET_TOWN_OAK");
await game.hideObject("PALLET_TOWN_OAK");

// 方式2：使用数字 ID（向后兼容）
await game.showObject(0);
```

### Rust 端数据结构

```rust
// pokered-script/src/config.rs

#[derive(Debug, Clone, Deserialize)]
pub struct NpcBinding {
    pub id: u8,
    pub talk: String,
    #[serde(default)]
    pub toggle_id: Option<String>,
    #[serde(default)]
    pub default_hidden: bool,
}

impl MapScriptConfig {
    /// 获取默认隐藏的 NPC ID 列表
    pub fn hidden_npc_ids(&self) -> Vec<u8> {
        self.npcs
            .iter()
            .filter(|n| n.default_hidden)
            .map(|n| n.id)
            .collect()
    }
    
    /// 通过 toggleId 查找 NPC ID
    pub fn npc_id_by_toggle(&self, toggle_id: &str) -> Option<u8> {
        self.npcs
            .iter()
            .find(|n| n.toggle_id.as_deref() == Some(toggle_id))
            .map(|n| n.id)
    }
}
```

---

## 迁移步骤

### Phase 1: 数据结构扩展

- [ ] **1.1** 扩展 `NpcBinding` 结构体，添加 `toggle_id` 和 `default_hidden` 字段
- [ ] **1.2** 扩展 `MapScriptConfig`，添加 `hidden_npc_ids()` 方法
- [ ] **1.3** 更新单元测试验证 JSON 解析

**验证**：`cargo test -p pokered-script`

### Phase 2: 应用隐藏逻辑

- [ ] **2.1** 修改 `map_data_loading.rs` 的 `load_full_map_data`：
  - 从 `script_config.json` 读取 `defaultHidden`
  - 过滤掉 `default_hidden: true` 的 NPC，或标记其不可见
- [ ] **2.2** 确保 `MapData.npcs` 正确反映初始可见性
- [ ] **2.3** 更新相关测试

**验证**：进入 Pallet Town 时不再看到 Oak（直到脚本触发）

### Phase 3: 数据迁移

- [ ] **3.1** 编写迁移脚本：从 `toggleable_objects.rs` 提取数据写入 JSON
- [ ] **3.2** 更新所有已配置地图的 `script_config.json`：
  - `PalletTown`
  - `OaksLab`
  - `BluesHouse`
  - `ViridianCity`
  - ...（共约 60+ 地图有 toggle 配置）
- [ ] **3.3** 验证迁移后数据与原版一致

**验证**：对比迁移前后 Rust test 结果

### Phase 4: 脚本 API 更新

- [ ] **4.1** 扩展 `game_api.rs` 的 `showObject`/`hideObject`：
  - 支持字符串参数（toggleId）
  - 保持数字参数兼容
- [ ] **4.2** 更新 `script.js` 使用命名常量：
  ```javascript
  const TOGGLE = {
    PALLET_TOWN_OAK: "PALLET_TOWN_OAK",
    DAISY_SITTING: "DAISY_SITTING",
    ...
  };
  ```
- [ ] **4.3** 更新脚本文档

**验证**：运行 Pallet Town Oak 出场脚本

### Phase 5: 废弃旧代码

- [ ] **5.1** 标记 `toggleable_objects.rs` 为 deprecated
- [ ] **5.2** 移除 `map_loading.rs` 中对 `toggleable_objects_for_map` 的调用
- [ ] **5.3** 更新 `pokered-data/src/lib.rs` 移除 `pub mod toggleable_objects`

**验证**：`cargo build` 无警告

### Phase 6: map-editor 集成

- [ ] **6.1** 更新 map-editor 读取新字段
- [ ] **6.2** UI 显示 NPC 的隐藏状态（如半透明图标）
- [ ] **6.3** 支持编辑 `toggleId` 和 `defaultHidden`

---

## 受影响文件

### Rust Core

| 文件 | 改动类型 |
|------|----------|
| `pokered-script/src/config.rs` | 扩展结构体 |
| `pokered-core/src/overworld/map_data_loading.rs` | 应用隐藏逻辑 |
| `pokered-script/src/game_api.rs` | 扩展 API |
| `pokered-data/src/toggleable_objects.rs` | 废弃 |

### Data Files

| 文件 | 改动类型 |
|------|----------|
| `pokered-data/maps/*/script_config.json` | 扩展 JSON |
| `pokered-data/maps/*/script.js` | 使用命名常量 |

### Tools

| 文件 | 改动类型 |
|------|----------|
| `tools/generate_map_json.rs`（如有） | 更新生成逻辑 |
| `map-editor` | 支持新字段 |

---

## 原版对照

原版 `toggle_constants.asm` 中的 toggle ID 映射：

```
PALLET_TOWN:
  TOGGLE_PALLET_TOWN_OAK (0x00) → NPC id=1 (Oak)

OAKS_LAB:
  TOGGLE_OAKS_LAB_RIVAL (0x2A) → NPC id=1 (Blue)
  TOGGLE_STARTER_BALL_1 (0x2B) → NPC id=2 (PokeBall)
  TOGGLE_STARTER_BALL_2 (0x2C) → NPC id=3 (PokeBall)
  TOGGLE_STARTER_BALL_3 (0x2D) → NPC id=4 (PokeBall)
  TOGGLE_OAKS_LAB_OAK_1 (0x2E) → NPC id=5 (Oak at desk), defaultHidden=true
  TOGGLE_POKEDEX_1 (0x2F) → NPC id=6 (Pokedex)
  TOGGLE_POKEDEX_2 (0x30) → NPC id=7 (Pokedex)
  TOGGLE_OAKS_LAB_OAK_2 (0x31) → NPC id=8 (Oak at door), defaultHidden=true
```

---

## 回滚计划

如需回滚：
1. 恢复 `toggleable_objects.rs` 的使用
2. 还原 `map_data_loading.rs` 对 toggle 的处理
3. JSON 中新增字段向后兼容，无需移除

---

## 时间估计

| Phase | 预计时间 |
|-------|----------|
| Phase 1 | 1-2 小时 |
| Phase 2 | 2-3 小时 |
| Phase 3 | 3-4 小时（数据迁移较多） |
| Phase 4 | 2-3 小时 |
| Phase 5 | 1 小时 |
| Phase 6 | 4-6 小时（map-editor） |

**总计**：13-19 小时

---

## 状态

- [ ] 待开始
- [ ] 进行中
- [ ] 已完成

**创建日期**：2026-04-04
**最后更新**：2026-04-04