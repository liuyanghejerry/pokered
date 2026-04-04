# 地图脚本完善计划

本文档记录了为 pokered-rust 重制版完善地图脚本的分批推进计划。

## 当前状态

| 项目 | 数量 |
|------|------|
| 地图总数 | 248 |
| 有脚本的地图 | 248 (100%) |
| 脚本完成度 | 存根阶段 (TODO 占位符) |

所有地图已生成 `script.js` 和 `script_config.json` 存根文件，需要填充实际游戏逻辑。

---

## 第一批：核心开场流程（优先级最高）

这些脚本直接影响游戏能否正常开始，必须在其他内容之前完成。

| 地图 | 核心功能 | 依赖关系 |
|------|---------|---------|
| PalletTown | Oak 出场事件 | ✓ 已有脚本 |
| OaksLab | 初始精灵选择、获得图鉴、Rival 对战 | ✓ 已有脚本 |
| Route1 | 教学引导、返回 Viridian | 依赖 OaksLab |
| ViridianCity | Old Man 捕捉教学、Gym 门状态检测 | 依赖 Route1 |
| BluesHouse | 地图获取事件 | 依赖 Pokedex |
| RedsHouse1F/2F | 基础交互 | 独立 |

**产出**：玩家可以完成从起床 → 选精灵 → 获得图鉴 → 学会捕捉 → 开始冒险

---

## 第二批：第一道馆区域

完成 Pewter City 区域的所有交互。

| 地图 | 核心功能 |
|------|---------|
| Route2 | 训练师战斗 |
| ViridianForest | 训练师战斗、道具收集 |
| PewterCity | 挡路人强制引导去 Gym |
| PewterGym | Brock 战斗、Badge 获取 |
| Museum1F/2F | 可选参观 |

**产出**：玩家可以挑战第一个道馆

---

## 第三批：月见山与华蓝市

| 地图 | 核心功能 |
|------|---------|
| MtMoon1F/B1F/B2F | 队员战斗、化石选择 |
| Route3/4 | 训练师战斗 |
| CeruleanCity | Rival 战斗、Nugget Bridge |
| CeruleanGym | Misty 战斗 |
| BillsHouse | PC 系统介绍、船票 |
| Route24/25 | Nugget Bridge 挑战 |

**产出**：完成前两个道馆，获得 PC 存储系统和船票

---

## 第四批：枯叶市与圣安奴号

| 地图 | 核心功能 |
|------|---------|
| Route5/6/7/8 | 地下通道入口 |
| VermilionCity | 体力恢复、Old Rod |
| VermilionGym | Lt. Surge 战斗 |
| SSAnne (所有层) | 船上剧情、Cut HM 获取 |
| VermilionDock | 登船检测 |
| DiglettsCave | Cut 后解锁 |

**产出**：获得 Cut 技能，可以探索更多区域

---

## 第五批：紫苑镇与幽灵塔

| 地图 | 核心功能 |
|------|---------|
| Route9/10 | 训练师战斗 |
| RockTunnel | 黑暗洞穴探索 |
| LavenderTown | 基础交互 |
| PokemonTower1F-7F | Marowak 幽灵战斗、Fuji 获救 |
| MrFujisHouse | 获得 Poke Flute |

**产出**：获得 Poke Flute，可以唤醒卡比兽

---

## 第六批：彩虹市与游戏城

| 地图 | 核心功能 |
|------|---------|
| CeladonCity | 百货大楼、Game Corner |
| CeladonGym | Erika 战斗 |
| GameCorner | 老虎机、代币购买 |
| GameCornerPrizeRoom | 奖品兑换 |
| RocketHideout (全部) | Team Rocket 剧情 |
| CeladonMansion | Eevee 获取 |

**产出**：完成 Rocket 剧情，获得 Silph Scope

---

## 第七批：金黄市与西尔佛公司

| 地图 | 核心功能 |
|------|---------|
| SaffronCity | 城市解锁事件 |
| SilphCo1F-11F | 多层迷宫、传送板 |
| SaffronGym | Sabrina 战斗 |
| FightingDojo | 获得格斗精灵 |
| CopycatsHouse | 获得月之石 |

**产出**：获得 Master Ball，解锁 Saffron City

---

## 第八批：浅红市与狩猎地带

| 地图 | 核心功能 |
|------|---------|
| Route12-15 | 训练师战斗 |
| FuchsiaCity | 基础交互 |
| FuchsiaGym | Koga 战斗 |
| SafariZone (全部) | 狩猎机制、HM03 Surf |
| WardensHouse | 金假牙、Surf 获取 |
| SeafoamIslands | Articuno 捕捉 |

**产出**：获得 Surf 技能

---

## 第九批：红莲岛与精灵屋

| 地图 | 核心功能 |
|------|---------|
| Route19-21 | 水上战斗 |
| CinnabarIsland | 基础交互 |
| PokemonMansion (全部) | 钥匙获取 |
| CinnabarGym | Blaine 战斗 |
| CinnabarLab | 化石复活 |

**产出**：完成第七个道馆

---

## 第十批：冠军之路与四天王

| 地图 | 核心功能 |
|------|---------|
| Route22/23 | 资格检测 |
| VictoryRoad1F-3F | 最终迷宫 |
| IndigoPlateau | Elite Four 入口 |
| LoreleisRoom | Elite 1 |
| BrunosRoom | Elite 2 |
| AgathasRoom | Elite 3 |
| LancesRoom | Elite 4 |
| ChampionsRoom | Rival 最终战 |
| HallOfFame | 结局动画 |

**产出**：完整的通关流程

---

## 第十一批：隐藏内容与可选内容

| 类型 | 内容 |
|------|------|
| 隐藏道具 | 地上道具、隐藏物品 |
| 特殊 NPC | 屋内的 NPC 对话 |
| 交易中心 | In-game trade |
| 证书 | 通关后内容 |

---

## 工作量估计

| 批次 | 地图数 | 复杂度 | 预计工时 |
|------|--------|--------|----------|
| 1 | 6 | 高 | 2-3 天 |
| 2 | 5 | 中 | 1-2 天 |
| 3 | 10 | 高 | 2-3 天 |
| 4 | 15+ | 高 | 2-3 天 |
| 5 | 10 | 高 | 2 天 |
| 6 | 15+ | 高 | 2-3 天 |
| 7 | 15+ | 高 | 2-3 天 |
| 8 | 15+ | 中高 | 2 天 |
| 9 | 10 | 中 | 1-2 天 |
| 10 | 12 | 高 | 2 天 |
| 11 | 全部 | 低 | 持续 |

**总计**：约 **20-30 天** 完整工作量

---

## 执行建议

1. **按批次顺序执行** - 每批次完成后游戏可玩性会显著提升
2. **优先完成第一批** - 这是游戏能正常启动的基础
3. **每批次完成后测试** - 确保游戏流程通畅
4. **脚本可迭代** - 先实现核心功能，细节后续优化

---

## 技术参考

- 原作脚本：`scripts/MapName.asm`
- 原作文本：`text/MapName.asm`
- 原作头文件：`data/maps/headers/MapName.asm`
- Rust 地图数据：`crates/pokered-data/maps/MapName/`

---

## 更新日志

- 2026-04-04：创建计划文档，完成存根脚本生成