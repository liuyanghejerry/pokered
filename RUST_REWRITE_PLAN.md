# Pokémon Red/Blue Rust 重写计划

## 1. 重写目标

将 Game Boy 汇编实现的 Pokémon Red/Blue 游戏逻辑，使用 Rust 重写为一个**跨平台的游戏引擎**。

### 核心目标
- ✅ **忠实还原**：完整重现原版游戏逻辑和机制
- ✅ **数据兼容**：能加载原版游戏数据（精灵数据、地图、音乐等）
- ✅ **可扩展**：模块化设计，便于 Mod 和功能扩展
- ✅ **跨平台**：支持 Windows、macOS、Linux、WebAssembly
- ✅ **高性能**：利用 Rust 的零成本抽象和内存安全

### 非目标
- ❌ 不是 Game Boy 模拟器（不模拟硬件）
- ❌ 不需要逐字节与原版 ROM 一致
- ❌ 不包含原版 ROM 的版权资源（图形/音乐需外部加载）

---

## 2. 技术栈选型

| 层次 | 推荐方案 | 备选方案 | 理由 |
|------|----------|----------|------|
| **语言** | Rust (2021 edition) | - | 内存安全、高性能、丰富生态 |
| **图形渲染** | `wgpu` + `winit` | `sdl2-rs`, `macroquad` | 跨平台GPU抽象,支持WebGPU/Vulkan/Metal/DX12 |
| **2D框架** | `pixels` (基于wgpu) | `minifb`, `softbuffer` | 像素级渲染，适合复古游戏 |
| **音频** | `rodio` + `cpal` | `kira`, `SDL2 audio` | 纯Rust音频播放 |
| **输入** | `gilrs` + `winit` | `SDL2 gamepad` | 手柄/键盘统一输入 |
| **序列化** | `serde` + `bincode` | `rmp-serde` | 存档/数据序列化 |
| **数据格式** | `ron` (Rusty Object Notation) | `toml`, `json` | 游戏数据定义 |
| **ECS (可选)** | `hecs` | `bevy_ecs`, `specs` | 轻量实体组件系统 |
| **日志** | `tracing` | `log` + `env_logger` | 结构化日志 |
| **错误处理** | `thiserror` + `anyhow` | - | 错误类型定义 |
| **测试** | `cargo test` + `proptest` | `quickcheck` | 单元测试 + 属性测试 |
| **WASM** | `wasm-bindgen` + `web-sys` | `trunk` | Web 平台支持 |

---

## 3. 项目架构

### 3.1 Cargo Workspace 结构

```
pokered-rust/
├── Cargo.toml                    # Workspace 根配置
├── crates/
│   ├── pokered-core/             # 🎯 核心游戏逻辑 (纯逻辑, 无IO)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── pokemon/          # 精灵系统
│   │   │   ├── battle/           # 战斗系统
│   │   │   ├── overworld/        # 大地图系统
│   │   │   ├── items/            # 物品系统
│   │   │   ├── trainer/          # 训练师系统
│   │   │   ├── types/            # 属性系统
│   │   │   ├── menu/             # 菜单状态机
│   │   │   ├── audio/            # 音频指令系统
│   │   │   ├── save/             # 存档系统
│   │   │   ├── link/             # 连接/对战
│   │   │   ├── math/             # 数学工具
│   │   │   ├── event/            # 事件/脚本系统
│   │   │   └── text/             # 文本处理
│   │   └── Cargo.toml
│   │
│   ├── pokered-data/             # 📊 游戏数据定义与加载
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── pokemon_data.rs   # 151只精灵数据
│   │   │   ├── move_data.rs      # 165个招式数据
│   │   │   ├── item_data.rs      # 83个物品数据（不含TM/HM，TM/HM单独管理）
│   │   │   ├── type_chart.rs     # 属性克制表
│   │   │   ├── trainer_data.rs   # 训练师队伍
│   │   │   ├── wild_data.rs      # 野生遭遇表
│   │   │   ├── map_data.rs       # 地图定义
│   │   │   ├── text_data.rs      # 文本内容
│   │   │   └── loader.rs         # 数据加载器
│   │   ├── data/                 # RON/TOML 数据文件
│   │   └── Cargo.toml
│   │
│   ├── pokered-renderer/         # 🎨 图形渲染层
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── tile_renderer.rs  # 瓦片地图渲染
│   │   │   ├── sprite.rs         # 精灵渲染
│   │   │   ├── battle_scene.rs   # 战斗场景
│   │   │   ├── text_renderer.rs  # 文本框渲染
│   │   │   ├── menu_renderer.rs  # 菜单渲染
│   │   │   ├── transition.rs     # 场景过渡效果
│   │   │   └── palette.rs        # 调色板管理
│   │   └── Cargo.toml
│   │
│   ├── pokered-audio/            # 🔊 音频引擎
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── engine.rs         # 音频引擎
│   │   │   ├── channel.rs        # 4声道模拟
│   │   │   ├── music.rs          # 音乐播放
│   │   │   └── sfx.rs            # 音效播放
│   │   └── Cargo.toml
│   │
│   └── pokered-app/              # 🚀 可执行程序入口
│       ├── src/
│       │   ├── main.rs           # 主入口
│       │   ├── input.rs          # 输入处理
│       │   ├── config.rs         # 配置管理
│       │   └── platform/         # 平台适配
│       │       ├── desktop.rs
│       │       └── wasm.rs
│       └── Cargo.toml
│
├── assets/                       # 游戏资源 (需用户自行提供)
│   ├── gfx/                      # 图形
│   ├── maps/                     # 地图
│   ├── audio/                    # 音频
│   └── data/                     # 数据
│
├── tools/                        # 开发工具
│   ├── rom_extractor/            # 从原版ROM提取数据
│   └── data_converter/           # 汇编数据→RON转换器
│
└── docs/                         # 文档
    ├── architecture.md
    ├── data_formats.md
    └── contributing.md
```

### 3.2 模块依赖关系

```
pokered-app
  ├── pokered-core        (游戏逻辑)
  ├── pokered-data        (游戏数据)
  ├── pokered-renderer    (图形)
  └── pokered-audio       (音频)

pokered-core
  └── pokered-data        (数据定义)

pokered-renderer
  └── pokered-core        (读取游戏状态)

pokered-audio
  └── pokered-core        (读取音频指令)
```

---

## 4. 核心数据结构设计 (Rust)

### 4.1 精灵系统

```rust
// pokered-data/src/pokemon_data.rs

/// 精灵种族 (151种 + NO_POKEMON)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Species {
    None = 0,
    Bulbasaur = 1,
    Ivysaur = 2,
    Venusaur = 3,
    // ... 151种
    Mew = 151,
}

/// 精灵属性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Type {
    Normal = 0x00,
    Fighting = 0x01,
    Flying = 0x02,
    Poison = 0x03,
    Ground = 0x04,
    Rock = 0x05,
    Bug = 0x07,
    Ghost = 0x08,
    Fire = 0x14,
    Water = 0x15,
    Grass = 0x16,
    Electric = 0x17,
    Psychic = 0x18,
    Ice = 0x19,
    Dragon = 0x1A,
}

/// 经验增长类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GrowthRate {
    MediumFast,
    SlightlyFast,
    SlightlySlow,
    MediumSlow,
    Fast,
    Slow,
}

/// 基础数据 (对应原版22字节结构)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseStats {
    pub dex_number: u8,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    pub special: u8,
    pub type1: Type,
    pub type2: Type,
    pub catch_rate: u8,
    pub base_exp: u8,
    pub front_sprite: String,  // 资源路径替代指针
    pub back_sprite: String,
    pub initial_moves: [MoveId; 4],
    pub growth_rate: GrowthRate,
    pub tm_compatibility: TmHmBitfield,  // 位域
}

/// 个体值 (DVs) - 2字节压缩
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DeterminantValues {
    raw: u16,  // 4位×4: ATK, DEF, SPD, SPC (HP由其他推算)
}

impl DeterminantValues {
    pub fn attack(&self) -> u8 { ((self.raw >> 12) & 0xF) as u8 }
    pub fn defense(&self) -> u8 { ((self.raw >> 8) & 0xF) as u8 }
    pub fn speed(&self) -> u8 { ((self.raw >> 4) & 0xF) as u8 }
    pub fn special(&self) -> u8 { (self.raw & 0xF) as u8 }
    pub fn hp(&self) -> u8 {
        // HP DV = 低位拼接其他四项的最低位
        ((self.attack() & 1) << 3)
            | ((self.defense() & 1) << 2)
            | ((self.speed() & 1) << 1)
            | (self.special() & 1)
    }
}

/// 队伍精灵 (对应原版44字节结构)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyPokemon {
    pub species: Species,
    pub current_hp: u16,
    pub level: u8,
    pub status: StatusCondition,
    pub type1: Type,
    pub type2: Type,
    pub catch_rate: u8,
    pub moves: [MoveId; 4],
    pub original_trainer_id: u16,
    pub experience: u32,          // 原版3字节, 扩展为u32
    pub hp_exp: u16,
    pub attack_exp: u16,
    pub defense_exp: u16,
    pub speed_exp: u16,
    pub special_exp: u16,
    pub dvs: DeterminantValues,
    pub pp: [u8; 4],
    // 计算值 (原版在偏移33-43)
    pub stats: Stats,
}

/// 能力值
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Stats {
    pub max_hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub special: u16,
}

/// 状态异常
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusCondition {
    None,
    Sleep(u8),      // 1-7 回合
    Poison,
    Burn,
    Freeze,
    Paralysis,
}
```

### 4.2 招式系统

```rust
// pokered-data/src/move_data.rs

/// 招式ID (165个招式)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum MoveId {
    None = 0,
    Pound = 1,
    KarateChop = 2,
    // ... 165个招式
    Struggle = 165,
}

/// 招式效果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveEffect {
    NoAdditionalEffect,
    PoisonSideEffect { chance: u8 },
    DrainHp,
    BurnSideEffect { chance: u8 },
    ParalyzeSideEffect { chance: u8 },
    // ... 86 种效果类型 (NUM_MOVE_EFFECTS = 86)
    Ohko,
    Charge,           // SolarBeam, Fly, Dig 等
    Trapping,         // Wrap, Fire Spin 等
    Recoil,           // 反伤
    Confusion,
    StatUp { stat: StatType, stages: i8 },
    StatDown { stat: StatType, stages: i8 },
    Heal,
    Transform,
    Substitute,
}

/// 招式定义 (对应原版6字节结构)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveData {
    pub id: MoveId,
    pub name: String,
    pub effect: MoveEffect,
    pub power: u8,
    pub move_type: Type,
    pub accuracy: u8,      // 0-100
    pub pp: u8,            // 0-40
}
```

### 4.3 属性克制系统

```rust
// pokered-core/src/types/mod.rs

/// 属性克制效果
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeEffectiveness {
    NoEffect,           // 0x (免疫)
    NotVeryEffective,   // 0.5x
    Normal,             // 1x
    SuperEffective,     // 2x
}

/// 属性克制表
pub struct TypeChart {
    matchups: Vec<TypeMatchup>,
}

#[derive(Debug, Clone)]
pub struct TypeMatchup {
    pub attacker: Type,
    pub defender: Type,
    pub effectiveness: TypeEffectiveness,
}

impl TypeChart {
    /// 计算招式对目标的属性倍率
    pub fn get_effectiveness(&self, attack_type: Type, def_type1: Type, def_type2: Type) -> f32 {
        let mut multiplier = 1.0;
        for matchup in &self.matchups {
            if matchup.attacker == attack_type {
                if matchup.defender == def_type1 || matchup.defender == def_type2 {
                    multiplier *= match matchup.effectiveness {
                        TypeEffectiveness::NoEffect => 0.0,
                        TypeEffectiveness::NotVeryEffective => 0.5,
                        TypeEffectiveness::Normal => 1.0,
                        TypeEffectiveness::SuperEffective => 2.0,
                    };
                }
            }
        }
        multiplier
    }
}
```

### 4.4 战斗系统

```rust
// pokered-core/src/battle/mod.rs

pub mod core;
pub mod damage;
pub mod ai;
pub mod move_effects;
pub mod wild_encounter;
pub mod experience;

/// 战斗状态
#[derive(Debug)]
pub struct BattleState {
    pub battle_type: BattleType,
    pub turn: u32,
    pub player: BattlerState,
    pub enemy: BattlerState,
    // 注意: 第一世代没有天气系统，天气从第二世代开始引入
    pub escape_attempts: u8,
    pub is_over: bool,
    pub result: Option<BattleResult>,
}

#[derive(Debug, Clone, Copy)]
pub enum BattleType {
    Wild,
    Trainer { class: TrainerClass, id: u8 },
    Link,
}

#[derive(Debug)]
pub struct BattlerState {
    pub active_pokemon: usize,   // 队伍索引
    pub party: Vec<PartyPokemon>,
    pub stat_stages: StatStages,  // -6 到 +6
    pub volatile_status: VolatileStatus,
    pub substitute_hp: u8,
    pub is_charging: bool,        // SolarBeam 等
    pub is_recharging: bool,      // Hyper Beam 等
    pub trapped_turns: u8,        // Wrap 等
    pub confusion_turns: u8,
    pub disabled_move: Option<(MoveId, u8)>,  // (招式, 剩余回合)
}

/// 战斗能力阶段 (-6 到 +6)
#[derive(Debug, Clone, Default)]
pub struct StatStages {
    pub attack: i8,
    pub defense: i8,
    pub speed: i8,
    pub special: i8,
    pub accuracy: i8,
    pub evasion: i8,
}

/// 伤害计算 (忠实还原第一世代公式)
pub fn calculate_damage(
    attacker: &PartyPokemon,
    defender: &PartyPokemon,
    move_data: &MoveData,
    attacker_stages: &StatStages,
    defender_stages: &StatStages,
    type_chart: &TypeChart,
    is_critical: bool,
    rng: &mut GameRng,
) -> u16 {
    // Level
    let level = if is_critical {
        attacker.level as u32 * 2
    } else {
        attacker.level as u32
    };

    // 攻击/防御选择 (物理 vs 特殊)
    let (attack, defense) = if is_physical(move_data.move_type) {
        (
            apply_stage(attacker.stats.attack, attacker_stages.attack),
            apply_stage(defender.stats.defense, defender_stages.defense),
        )
    } else {
        (
            apply_stage(attacker.stats.special, attacker_stages.special),
            apply_stage(defender.stats.special, defender_stages.special),
        )
    };

    // 基础伤害公式
    let base = ((2 * level / 5 + 2) * move_data.power as u32 * attack as u32
        / defense as u32 / 50 + 2) as u16;

    // STAB (同属性加成 1.5x)
    let stab = if move_data.move_type == attacker.type1
        || move_data.move_type == attacker.type2
    {
        base * 3 / 2
    } else {
        base
    };

    // 属性克制
    let effectiveness = type_chart.get_effectiveness(
        move_data.move_type, defender.type1, defender.type2
    );
    let typed = (stab as f32 * effectiveness) as u16;

    // 随机倍率 (217-255) / 255
    let random = rng.range(217, 255) as u32;
    let final_damage = (typed as u32 * random / 255) as u16;

    final_damage.max(1)  // 最少造成1点伤害
}
```

### 4.5 大地图系统

```rust
// pokered-core/src/overworld/mod.rs

pub mod map;
pub mod movement;
pub mod collision;
pub mod sprites;
pub mod warp;
pub mod encounter;

/// 大地图状态
#[derive(Debug)]
pub struct OverworldState {
    pub current_map: MapId,
    pub player: PlayerState,
    pub npcs: Vec<NpcState>,
    pub camera: CameraState,
    pub walk_counter: u8,
    pub encounter_cooldown: u8,
    pub repel_steps: u16,
}

/// 玩家状态
#[derive(Debug)]
pub struct PlayerState {
    pub x: u16,
    pub y: u16,
    pub facing: Direction,
    pub movement_state: MovementState,
    pub walk_bike_surf: TransportMode,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction { Down, Up, Left, Right }

#[derive(Debug, Clone, Copy)]
pub enum TransportMode { Walking, Biking, Surfing }

#[derive(Debug, Clone, Copy)]
pub enum MovementState { Idle, Walking, Jumping }

/// 地图数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    pub id: MapId,
    pub name: String,
    pub width: u8,
    pub height: u8,
    pub tileset: TilesetId,
    pub music: MusicId,
    pub blocks: Vec<u8>,       // 瓦片块数据
    pub warps: Vec<WarpPoint>,
    pub npcs: Vec<NpcDefinition>,
    pub signs: Vec<Sign>,
    pub wild_pokemon: Option<WildEncounterTable>,
    pub connections: MapConnections,
}
```

### 4.6 存档系统

```rust
// pokered-core/src/save/mod.rs

/// 存档数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub player_name: String,
    pub rival_name: String,
    pub player_id: u16,
    pub party: Vec<PartyPokemon>,
    pub current_box: u8,
    pub boxes: [PokemonBox; 12],
    pub bag: Inventory,
    pub pc_items: Inventory,
    pub money: u32,
    pub badges: u8,              // 8位位域
    pub pokedex_owned: [u8; 19], // 151位
    pub pokedex_seen: [u8; 19],
    pub play_time: PlayTime,
    pub event_flags: EventFlags,
    pub current_map: MapId,
    pub player_x: u16,
    pub player_y: u16,
    pub options: GameOptions,
    pub checksum: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonBox {
    pub pokemon: Vec<PartyPokemon>,  // 最多20只
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<(ItemId, u8)>,  // (物品ID, 数量)
}

impl SaveData {
    /// 计算校验和 (兼容原版算法)
    pub fn calculate_checksum(&self) -> u8 {
        let bytes = bincode::serialize(self).unwrap();
        let sum: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        !sum  // 取反
    }

    /// 保存到文件
    pub fn save_to_file(&self, path: &Path) -> Result<()> { /* ... */ }

    /// 从文件加载
    pub fn load_from_file(path: &Path) -> Result<Self> { /* ... */ }
}
```

---

## 5. 开发阶段规划

### 第一阶段：基础架构与数据层 (预计 4-6 周)

**目标：** 建立项目骨架，实现数据定义和加载

- [x] **M1.1** 初始化 Cargo Workspace 和项目结构 ✅
- [x] **M1.2** 定义所有枚举类型 (Species, MoveId, ItemId, Type 等) ✅
  - `species.rs`: Species(152变体) + GrowthRate(6)
  - `types.rs`: PokemonType(15类型，含hex repr) + Effectiveness
  - `moves.rs`: MoveId(166变体) + MoveEffect(87，含gap)
  - `items.rs`: ItemId(84) + HmId(5) + TmId(50) + TM/HM→Move映射
- [x] **M1.3** 实现 165 个招式数据 (`MoveData`) ✅ — 从 ASM 自动生成
- [x] **M1.4** 实现属性克制表 (`TypeChart`) ✅ — 82条克制表 + lookup函数
- [x] **M1.5** 实现 `BaseStats` 结构定义 ✅ — 结构体已定义，151只数据待填充
- [x] **M1.5b** 填充 151 只精灵的基础数据 (`BaseStats`) ✅ — 1393行，含TM/HM位域
- [x] **M1.6** 实现 83 个物品数据 (`ItemData`，不含 TM/HM，TM01-TM50 + HM01-HM05 单独管理) ✅
- [x] **M1.7** 实现训练师队伍数据 ✅ — 47训练师类型，391支队伍
- [x] **M1.8** 实现野生遭遇表 ✅ — 56个地图，含Red/Blue版本差异+钓鱼数据
- [x] **M1.9** 实现进化/招式学习表
- [x] **M1.10** 编写数据转换工具 (ASM → RON) ✅ — 数据已通过Python脚本从ASM直接生成为Rust源码，RON格式导出推迟到Mod支持阶段
- [x] **M1.11** 数据完整性测试 (所有151只精灵、属性克制正确性等) ✅ — 7个测试文件，81项测试全部通过

**验收标准：**
```bash
cargo test -p pokered-data  # 所有数据测试通过
```

### 第二阶段：核心战斗逻辑 ✅ 已完成

**目标：** 实现完整的战斗系统

- [x] **M2.1** 实现伤害计算公式 (STAB、属性克制、暴击、随机) ✅ — 忠实ASM实现：顺序属性克制应用(floor(damage*mult/10))、暴击、STAB、随机化、能力值缩放(>255时/4)、反射/光幕、自爆减防、0伤害判定为miss
- [x] **M2.2** 实现能力阶段系统 (-6 到 +6) ✅ — StatStages结构体+apply_stage+StatIndex，倍率表验证与ASM StatModifierRatios一致
- [x] **M2.3** 实现回合执行流程 (先攻判定、招式执行、效果结算) ✅ — 分7个模块实现：state.rs(战斗状态/宝可梦/战斗者)、turn_order.rs(先攻判定：先制+速度+硬直)、status_checks.rs(9步状态检查：睡眠→冰冻→束缚→畏缩→蓄力→禁止→混乱→封印→麻痹)、accuracy.rs(命中判定：命中/闪避阶段+Swift/X命中绕过)、residual.rs(残余伤害：烧伤/中毒/剧毒/寄生种子)、move_execution.rs(单招执行管线：状态→PP→暴击→伤害→命中→替身)、turn.rs(回合编排器：排序→双方行动→残余→倒下检查)
- [x] **M2.4** 实现所有 86 种招式效果 ✅ — 8个子模块实现全部86种效果，已集成至move_execution管线。102个效果专项测试（status_effects:11 + stat_effects:9 + damage_effects:14 + multi_hit:5 + multi_turn:11 + field_effects:15 + special_effects:17 + dispatcher覆盖），总计179测试通过
  - [x] 直接伤害
  - [x] 状态异常 (中毒、烧伤、冰冻、麻痹、睡眠)
  - [x] 能力变化
  - [x] 吸血/反伤
  - [x] 多段攻击
  - [x] 一击必杀
  - [x] 充能招式 (SolarBeam, Fly, Dig)
  - [x] 束缚招式 (Wrap, Fire Spin)
  - [x] 替身、变身、模仿等特殊效果
- [x] **M2.5** 实现野生精灵遭遇和捕获系统 ✅ — wild.rs: 遭遇触发(encounter_rate vs hRandomAdd)、10槽位累积概率表(从ASM probabilities.asm精确提取)、驱虫喷雾判定(严格小于才阻止)。capture.rs: 完整Gen1捕获算法(大师球→rand1接受阈值→状态减算→W值计算→catch_rate判定→rand2判定→摇晃次数)，支持5种球、状态加成。29个新测试(wild:14 + capture:15)，总计208测试通过
- [x] **M2.6** 实现训练师 AI ✅ — trainer_ai/mod.rs: MoveChoiceLayer枚举(Layer1/2/3)、move_choice_layers()按训练师职业返回修正层列表(匹配move_choices.asm)、TrainerAiConfig/AiRoutine(19种AI例程)、trainer_ai_config()按职业返回AI配置(匹配ai_pointers.asm)。move_choice.rs: 4槽评分缓冲区(初始10分)、禁用招式惩罚(0x50)、Layer1(已有状态时抑制纯状态招式+5)、Layer2(鼓励标志=1时对stat-up/bide招式-1)、Layer3(克制-1/被克制+1仅当有更好招式)、最小值过滤。ai_action.rs: AiAction枚举(DoNothing/UsePotion/UseFullHeal/UseX*/UseGuardSpec/SwitchPokemon)、execute_ai_action()主入口(ai_count耗尽检查+charge消耗)、19个per-class例程完整实现(含CooltrainerF的ret nc缺失bug)。59个新测试(mod:14 + move_choice:16 + ai_action:29)，总计267测试通过
- [x] **M2.7** 实现经验值计算和等级提升 ✅ — experience/模块(4个子模块+4个测试文件)。growth.rs: 6种成长曲线三次多项式(exp_for_level/level_from_exp/max_exp)，精确匹配data/growth_rates.asm系数。stats.rs: Gen1 DV字节解包(atk_def/spd_spc双字节→5项IV)、HP IV位拼接、calc_stat公式(base+iv)*2+ev_term)*level/100+bonus)、ceil(sqrt(stat_exp))/4的EV贡献。gain.rs: calc_exp_gain(base_exp*level/7)、traded/trainer各1.5x可叠加、add_stat_exp(累加敌方基础值，saturating)、gain_experience主流程(EXP ALL双遍历：半值fighters+半值全队)。level_up.rs: process_level_up(检测新等级→重算5项能力值→HP差值补正→遍历learnset学新招→满槽替换末位)。Pokemon结构体扩展: dv_bytes/stat_exp/total_exp/is_traded。33个新测试(growth:10 + stats:8 + gain:8 + level_up:6 + 1 extra)，总计300测试通过
- [x] **M2.8** 实现战后结算 (金钱、进化检查) ✅ — settlement/模块(4个子模块+3个测试文件)。money.rs: calc_prize_money(base_money×level)、calc_blackout_penalty(money/2)、calc_total_winnings(prize+payday封顶999,999)。evolution.rs: check_level_evolution/check_trade_evolution/check_item_evolution(从evos_moves_data查表)、apply_evolution(更新species/type1/type2、重算全能力值、HP差值补正)。settle.rs: settle_battle编排器(Win=奖金+PayDay+进化、Loss=金钱减半、Escaped/Captured=仅进化检查)。BattleOutcome枚举(Win/Loss/Draw/Escaped/Captured)、BattleSettlement结构体。trainer_data.rs新增get_base_money()含47职业基础奖金。28个新测试(money:10 + evolution:9 + settle:9)，总计328测试通过
- [x] **M2.9** 实现逃跑机制 ✅ — escape/模块(mod.rs+tests.rs)。try_run_from_battle: 训练师战→禁止逃跑、野生战→速度比较(player≥enemy必逃)+概率公式(player_speed×32÷((enemy_speed÷4)%256)+30×attempts vs random)、溢出/除数为0特殊情况处理。try_escape_move: Teleport/Whirlwind/Roar的SwitchAndTeleportEffect(训练师战必败、野生战按等级比较+概率判定rand[0,c)≥target_lvl÷4)。RunResult/TeleportResult/EscapeMove枚举。17个新测试，总计345测试通过
- [x] **M2.10** 战斗系统集成测试 ✅ — 6个集成测试文件(helpers.rs共享工具+5个场景文件)。wild_battle_flow(2测试: 野生战遭遇→战斗→KO→经验→结算、削弱→捕获)、trainer_battle_flow(3测试: 多回合KO→经验→奖金、战败→惩罚、PayDay奖金)、escape_flow(6测试: 失败→重试→成功、训练师战禁逃、高速必逃、瞬移成功/失败、尝试次数累积)、status_across_turns(5测试: 烧伤/中毒/剧毒逐回合伤害、睡眠回合递减、冰冻阻止行动)、edge_cases(7测试: 多宝可梦经验分配、EXP ALL、交换奖励、进化触发、濒死不进化、PP追踪、奖金上限)。共23个集成测试全部通过，总计368测试(345单元+23集成)零失败

**验收标准：**
```bash
cargo test -p pokered-core -- battle  # 战斗测试通过
# 可在终端运行简单的战斗模拟
```

### 第三阶段：精灵管理与物品系统 (预计 3-4 周)

**目标：** 实现精灵管理和物品使用

- [x] **M3.1** 实现队伍管理 (添加/移除/交换) ✅ 已完成 — Party struct, 24个单元测试, 393总测试
- [x] **M3.2** 实现精灵存储箱系统 (12个箱子, 每个20只) ✅ 已完成 — PcBox/PcStorage, 23个单元测试, 415总测试
- [x] **M3.3** 实现进化系统 (等级进化、道具进化、交换进化) ✅ 已完成 — 统一EvolutionTrigger API, 14个单元测试, 429总测试
- [x] **M3.4** 实现招式学习/遗忘 ✅ 已完成 — move_learning模块: try_learn_move/teach_tm/teach_hm/forget_move/process_level_up_moves, tm_hm_flags位域兼容检查, 36个单元测试, 465总测试
- [x] **M3.5** 实现能力值计算 (基础值 + 个体值 + 努力值) ✅ 已完成 — stats模块: recalculate_stats(HP差值补正)、create_pokemon/create_pokemon_with_moves(一站式创建含正确stats/types/moves/PP/exp)、stat_summary(IV提取)、StatSummary结构体, 14个单元测试, 479总测试
- [x] **M3.6** 实现物品使用效果 ✅ 已完成 — items模块: healing(药水10种)、status_cure(状态治疗6种)、pp_restore(PP恢复5种+PP Up)、vitamins(营养剂5种+神奇糖果)、battle_items(战斗道具8种); Pokemon结构体新增pp_ups字段; 74个单元测试, 623总测试
  - [x] 回复类 (药水、解药)
  - [x] 战斗道具 (X攻击等)
  - [x] PP恢复 (以太/万灵药/PP提升)
  - [x] 营养剂 (HP提升/蛋白质/钙质等) + 神奇糖果(升级+进化+招式学习)
  - [ ] 精灵球 (捕获率计算) — 已在battle/capture.rs实现
  - [ ] 进化石 — 已在pokemon/evolution.rs实现
  - [ ] TM/HM — 已在pokemon/move_learning.rs实现
  - [ ] 关键道具 (自行车、钓竿等) — 属于大地图阶段
- [x] **M3.7** 实现背包/PC物品管理 ✅
- [x] **M3.8** 实现图鉴系统 ✅

### 第四阶段：大地图与事件系统 (预计 8-12 周)

**目标：** 实现完整的大地图探索、文本引擎和事件脚本

- [x] **M4.1** 实现地图加载和瓦片渲染 ✅
- [x] **M4.2** 实现玩家移动和碰撞检测 ✅
- [x] **M4.3** 实现地图连接和传送点 (包括双向无缝地图加载: 东西南北四方向连接) ✅
- [x] **M4.4** 实现 NPC 移动和交互 ✅ — parse_npcs.py解析223个ASM文件生成918个NPC和201个标牌数据(含OPP_前缀训练师/野生精灵遭遇区分); npc_movement.rs: NpcRuntimeState状态机(Stationary/Wander/FacePlayer/FixedPath)、load_map_npcs、update_npc_movement(范围限制+碰撞)、位置查询; npc_interaction.rs: try_interact(Talk/TrainerBattle/ItemPickup/AlreadyDefeated)、check_trainer_line_of_sight(视线范围检测)、collect_item/mark_trainer_defeated、check_sign_interaction; 45个新测试(数据完整性+移动+交互+训练师视线+标牌)，总计853测试通过
- [x] **M4.5** 实现脚本/事件系统 ✅ — parse_event_flags.py解析507个事件标志(最大bit 0x9DA, 316字节存储); parse_trainer_headers.py解析322个训练师头(跨69张地图); event_flags.rs: EventFlags位数组(set/check/reset/raw/count/clear/save-load); script_engine.rs: ScriptAction(23种动作)、ScriptPhase状态机、MapScriptState(动作队列+信号机制)、CoordEvent坐标触发、execute_next_action; trainer_engine.rs: TrainerBattleState(7态状态机)、can_trainer_see_player(方向+距离视线检测)、check_all_trainers; map_scripts.rs: MapScriptContext(整合脚本+坐标事件+训练师遭遇)、MapScriptType分类(StorySequence/TrainerMap/SimpleTextOnly/NoScripts); 68个新测试，总计921测试通过
- [x] **M4.6** 实现野生遭遇触发 ✅ — wild_encounters.rs: TileEncounterType(Grass/Water/IndoorCave/None)、determine_encounter_type(草地/水面/洞穴判定)、should_check_encounter(传送/NPC脚本/冷却前置检查)、check_wild_encounter(整合地形+概率+驱虫+等级判定); wild_data.rs: wild_data_name_for_map(50+地图映射)、wild_data_for_map(MapId→MapWildData桥接); 特殊处理: 森林图鉴需草地砖块、洞穴室内地图全砖触发、Route19/20共享SeaRoutes数据; 53个新测试，总计975测试通过
- [x] **M4.7** 实现秘传技效果 (Cut, Surf, Fly, Strength, Flash) ✅ — hm_effects.rs: 5枚徽章位检查(BoulderBadge=0~SoulBadge=4)、CutResult(树砍伐+草清除，Overworld树$3D/草$52、Gym树$50，9组方块替换表)、FlyResult(ThunderBadge+室外地图限制，13个飞行目的地含Route4/Route10)、SurfResult(SoulBadge+水面朝向+SeafoamB4F双石特殊阻挡+自行车道限制)、StrengthResult(RainbowBadge+二次推动机制)、FlashResult(BoulderBadge+暗洞照明); try_push_boulder/try_push_boulder_with_direction(力量激活+灰尘状态+精灵检测+方向匹配+碰撞); tileset_data.rs新增: cut_tree_replacement、is_dungeon_tileset(11种)、is_water_tileset(9种)、is_outside_tileset; 56个新测试，总计1031测试通过
- [x] **M4.8** 实现特殊地形 (草地、水面、暗洞)
- [x] **M4.9** 实现门、电梯、传送门 ✅ — elevator_data.rs: ElevatorFloor(14层)/ElevatorId(3部电梯)/elevator_data(层→MapId映射)/elevator_for_map; fly_warp_data.rs: FlyWarpCoords/FLY_DESTINATIONS(13目的地)/DUNGEON_WARPS(12入口)/NEW_GAME_WARP/LinkWarpType/TELEPORT_SPIN_ORDER; tileset_data.rs: is_door_tile(13种tileset门砖判定); doors_elevators.rs: is_standing_on_door/get_elevator_for_map/select_elevator_floor/resolve_fly_destination/teleport_spin_direction/resolve_dungeon_warp/new_game_warp/resolve_link_warp/elevator_shake_params; 62个新测试，总计1046测试通过
- [x] **M4.10** 实现所有 248 张地图数据
- [x] **M4.11** 实现文本引擎/命令处理器 (控制码: `<LINE>`, `<PAGE>`, `<DONE>`, `<PLAYER>`, `<RIVAL>` 等，逐字渲染，文本框管理)
- [x] **M4.12** 实现精灵/OAM 系统 (精灵碰撞、OAM DMA、精灵叠加优先级) ✅ — pokered-data: oam_constants.rs(OAM常量/位标志)、sprite_facing.rs(SpriteFacingAndAnimationTable 32项方向/动画查找表、6种瓦片模式、2种OAM模板)、sprite_state_constants.rs(SpriteStateData1/2字段偏移、MovementStatus枚举、朝向/移动常量); pokered-core/overworld/sprites/: mod.rs(SpriteStateData1/2结构体、SpriteTable 16槽管理、ShadowOam 40项OAM缓冲、OamEntry)、oam.rs(prepare_oam_data: 16精灵→OAM转换循环+facing表查找+草地优先级+ledge/fishing保留末4项)、collision.rs(detect_collisions_for_sprite: O(n²)碰撞检测+7/9像素阈值+方向位+16位碰撞位表)、update.rs(update_sprites调度器: 玩家/NPC分别处理+移动状态机+动画帧更新); 47个新测试(核心21+数据26)，总计1087+测试通过
- [x] **M4.13** 实现老虎机迷你游戏 (engine/slots/ 完整子系统)

### 第五阶段：图形渲染层 (预计 4-6 周)

**目标：** 实现可视化的游戏界面

- [x] **M5.1** 实现基础窗口和像素缓冲
- [x] **M5.2** 实现瓦片地图渲染器
- [x] **M5.3** 实现精灵渲染 (NPC、玩家)
- [x] **M5.4** 实现文本框和对话系统
- [x] **M5.5** 实现菜单渲染 (开始菜单、队伍、背包等)
- [x] **M5.6** 实现战斗场景渲染
- [x] **M5.7** 实现战斗动画
- [x] **M5.8** 实现场景过渡效果 (淡入淡出)
- [x] **M5.9** 实现调色板系统 (原版4色 + 可选彩色模式)
- [x] **M5.10** 资源加载管线 (PNG → 纹理)

### 第六阶段：音频系统 (预计 3-4 周)

**目标：** 实现音乐和音效播放

- [x] **M6.1** 实现 Game Boy 4声道音频模拟
- [x] **M6.2** 实现音乐序列器
- [x] **M6.3** 加载和播放所有 ~30 首 BGM
- [x] **M6.4** 实现 100+ 音效
- [x] **M6.5** 实现音量控制和淡入淡出

### 第七阶段：菜单与UI (预计 3-4 周)

**目标：** 实现所有菜单界面

- [x] **M7.1** 标题画面和主菜单
- [x] **M7.2** 开始菜单 (图鉴、队伍、背包、训练卡等)
- [x] **M7.3** 战斗菜单 (招式选择、精灵切换、道具使用)
- [x] **M7.4** PC 系统 (存取精灵)
- [x] **M7.5** 商店系统
- [x] **M7.6** 命名画面
- [x] **M7.7** 存读档界面
- [x] **M7.8** 设置界面 (文字速度、战斗动画、战斗模式)

### 第八阶段：存档与连接 (预计 2-3 周)

**目标：** 实现存档和多人功能

- [ ] **M8.1** 实现存档系统 (兼容原版校验和算法)
- [ ] **M8.2** 实现原版存档导入
- [ ] **M8.3** 实现网络对战 (替代串口通信)
- [ ] **M8.4** 实现网络交换

### 第九阶段：集成测试与打磨 (预计 4-6 周)

**目标：** 完整游戏流程测试

- [ ] **M9.1** 完整游戏流程测试 (开场 → 冠军)
- [ ] **M9.2** 所有训练师战斗平衡性验证
- [ ] **M9.3** 所有事件/脚本触发测试
- [ ] **M9.4** 性能优化
- [ ] **M9.5** WebAssembly 构建
- [ ] **M9.6** 文档完善

---

## 6. 开发优先级

```
高优先级 (MVP)          中优先级              低优先级
─────────────────   ─────────────────   ─────────────────
数据层 (Phase 1)     大地图系统 (Phase 4)  音频系统 (Phase 6)
战斗系统 (Phase 2)   图形渲染 (Phase 5)    网络对战 (Phase 8)
精灵管理 (Phase 3)   菜单UI (Phase 7)      WebAssembly
存档系统 (Phase 8)   事件系统 (Phase 4)    Mod 支持
```

**建议 MVP (最小可玩产品)：** Phase 1-3 完成后即可实现命令行战斗模拟器

---

## 7. 测试策略

### 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_effectiveness() {
        let chart = TypeChart::default();
        // 水克火
        assert_eq!(chart.get_effectiveness(Type::Water, Type::Fire, Type::Fire), 2.0);
        // 一般对鬼无效
        assert_eq!(chart.get_effectiveness(Type::Normal, Type::Ghost, Type::Ghost), 0.0);
        // 双属性: 冰对飞行/龙 = 4x
        assert_eq!(chart.get_effectiveness(Type::Ice, Type::Flying, Type::Dragon), 4.0);
    }

    #[test]
    fn test_damage_calculation() {
        // 使用已知的原版战斗数据验证伤害公式
        let pikachu = create_test_pokemon(Species::Pikachu, 25);
        let geodude = create_test_pokemon(Species::Geodude, 20);
        let thundershock = get_move(MoveId::Thundershock);

        // 电对地面 = 0 伤害
        let damage = calculate_damage(&pikachu, &geodude, &thundershock, ...);
        assert_eq!(damage, 0);
    }

    #[test]
    fn test_experience_growth() {
        // 验证各经验增长类型在关键等级的经验值
        assert_eq!(exp_for_level(GrowthRate::MediumFast, 100), 1_000_000);
        assert_eq!(exp_for_level(GrowthRate::Slow, 100), 1_250_000);
    }

    #[test]
    fn test_all_151_pokemon_data() {
        let data = PokemonDatabase::load();
        assert_eq!(data.species_count(), 151);
        // 验证所有精灵都有有效数据
        for species in Species::iter() {
            let stats = data.get_base_stats(species);
            assert!(stats.hp > 0);
            assert!(stats.catch_rate > 0 || species == Species::Mewtwo);
        }
    }
}
```

### 属性测试 (Property-Based Testing)
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn damage_is_always_positive_for_effective_moves(
        level in 1u8..100,
        power in 1u8..255,
    ) {
        let damage = calculate_base_damage(level, power, 100, 100);
        prop_assert!(damage >= 1);
    }

    #[test]
    fn stat_stages_are_bounded(stage in -6i8..=6) {
        let modified = apply_stage(100, stage);
        prop_assert!(modified >= 25);  // -6 阶 = 25%
        prop_assert!(modified <= 400); // +6 阶 = 400%
    }
}
```

### 集成测试 (与原版比对)
```rust
#[test]
fn test_battle_replay_matches_original() {
    // 使用预录的原版战斗输入序列
    // 验证每一回合的结果与原版一致
    let replay = BattleReplay::load("tests/fixtures/battle_001.json");
    let mut battle = Battle::new(replay.initial_state());
    for action in replay.actions() {
        let result = battle.execute_turn(action);
        assert_eq!(result, replay.expected_result());
    }
}
```

---

## 8. 关键技术决策

### 8.1 随机数生成
原版使用硬件定时器 (`rDIV`) 作为随机源。Rust 重写应：
- 使用确定性 PRNG (`rand_xoshiro` 或自定义)
- 支持 seed 设置 (用于重放和测试)
- 提供与原版兼容的随机数分布

### 8.2 Bank 切换处理
原版代码高度依赖 bank 切换。Rust 重写中：
- **完全消除** bank 概念 → 直接函数调用
- 原版 `farcall`/`Bankswitch` → Rust 普通方法调用
- 数据访问无需 bank 切换 → 统一内存模型

### 8.3 定时与帧率
原版在 V-Blank 中断 (~59.7 FPS) 下运行：
- 使用 `std::time` 或渲染框架的帧率控制
- 支持可配置帧率 (1x, 2x, 4x 速度)
- 逻辑帧率与渲染帧率分离

### 8.4 文本编码
原版使用自定义字符映射 (`charmap.asm`)：
- Rust 使用标准 UTF-8
- 提供原版字符映射转换器 (用于导入原版存档)

### 8.5 数据存储格式
- **开发阶段**: RON (Rusty Object Notation) - 易于编辑
- **发布阶段**: bincode 或自定义二进制格式 - 高效加载
- **存档**: serde + bincode，可选兼容原版 SRAM 格式

### 8.6 游戏主循环架构
原版使用 V-Blank 中断驱动的帧循环 (`home/vblank.asm`)，在每次 V-Blank 时更新输入、动画、音频和游戏状态。Rust 重写需要设计显式的游戏主循环：
- **固定时间步长**：逻辑更新以 ~59.7 FPS 固定步长运行，与原版帧率一致
- **状态机驱动**：顶层使用游戏状态枚举 (TitleScreen, Overworld, Battle, Menu, Transition 等) 驱动不同的 update/render 路径
- **输入缓冲**：模拟原版的 `hJoyPressed`/`hJoyHeld` 行为，在帧开始时采样输入
- **VBlank 等效**：将原版 VBlank 中的 OAM DMA、VRAM 拷贝等操作转换为显式的 `end_of_frame()` 调用
- **延迟帧**：原版大量使用 `DelayFrames` 等待函数（如文本逐字显示、动画等待），需转换为异步状态或帧计数器

---

## 9. 可扩展性设计

Rust 重写不仅是忠实还原，还应为未来扩展预留空间：

### 9.1 Mod 系统
```rust
/// 游戏数据可通过外部文件覆盖
pub struct GameData {
    pub pokemon: Vec<BaseStats>,   // 可添加自定义精灵
    pub moves: Vec<MoveData>,      // 可添加自定义招式
    pub items: Vec<ItemData>,      // 可添加自定义物品
    pub maps: Vec<MapData>,        // 可添加自定义地图
    // 从 data/ 目录加载 RON 文件
}
```

### 9.2 配置化
```rust
/// 游戏行为可通过配置调整
pub struct GameConfig {
    pub generation: Generation,     // Gen1 经典 / Gen1 修正
    pub physical_special_split: bool, // 是否启用物特分离
    pub infinite_tm: bool,          // TM是否可重复使用
    pub run_indoor: bool,           // 室内是否可跑步
    pub exp_share_all: bool,        // 全队经验分享
    pub speed_multiplier: f32,      // 游戏速度
}
```

---

## 10. 时间估算总览

| 阶段 | 内容 | 预计时间 | 人力 |
|------|------|----------|------|
| Phase 1 | 基础架构与数据层 | 4-6 周 | 1人 |
| Phase 2 | 核心战斗逻辑 | 6-8 周 | 1-2人 |
| Phase 3 | 精灵管理与物品 | 3-4 周 | 1人 |
| Phase 4 | 大地图与事件 | 8-12 周 | 1-2人 |
| Phase 5 | 图形渲染 | 4-6 周 | 1人 |
| Phase 6 | 音频系统 | 3-4 周 | 1人 |
| Phase 7 | 菜单与UI | 3-4 周 | 1人 |
| Phase 8 | 存档与连接 | 2-3 周 | 1人 |
| Phase 9 | 集成测试 | 4-6 周 | 1-2人 |
| **总计** | | **37-53 周** | **1-2人** |

> 💡 **注意**: 单人全职开发约需 9-13 个月。Phase 1-3 完成后可产出 MVP (命令行战斗模拟器)，约需 3-4 个月。Phase 4 因新增文本引擎、精灵系统、老虎机等子系统，工期从原估 6-8 周上调至 8-12 周。

---

## 11. 风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 原版BUG复现困难 | 行为不一致 | 建立原版行为测试集，逐个验证 |
| 音频模拟复杂度高 | 音效不准确 | 可先使用预录制音频，后续实现合成 |
| 地图数据量大 (248张) | 转换耗时 | 编写自动化 ASM→RON 转换工具 |
| 事件脚本复杂 (224个地图脚本) | 实现遗漏 | 设计可扩展的脚本解释器，含事件标志位系统 |
| 文本引擎命令处理器 | 低估复杂度 | 原版文本引擎含控制码解析、逐字渲染、文本框状态机，需独立设计模块 |
| 精灵/OAM系统 | 渲染层耦合 | NPC状态机、碰撞检测、OAM优先级需与渲染层协同设计 |
| 版权问题 | 法律风险 | 代码开源,资源需用户自行提供 |

---

## 12. 快速启动命令

```bash
# 1. 创建项目
cargo init pokered-rust
cd pokered-rust

# 2. 设置 workspace
cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "crates/pokered-core",
    "crates/pokered-data",
    "crates/pokered-renderer",
    "crates/pokered-audio",
    "crates/pokered-app",
]

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
ron = "0.8"
bincode = "1"
thiserror = "1"
anyhow = "1"
tracing = "0.1"
rand = "0.8"
EOF

# 3. 创建各 crate
for crate in pokered-core pokered-data pokered-renderer pokered-audio pokered-app; do
    cargo init "crates/$crate" --lib
done
cargo init crates/pokered-app --bin

# 4. 开始编码
cargo build  # 验证项目结构
cargo test   # 运行测试
```

---

## 总结

本计划将 ~173,000 行 Game Boy Z80 汇编代码重构为模块化的 Rust 项目，分为 9 个阶段渐进式实现。基于对原版汇编的深度分析，关键数据：151 种精灵、165 个招式、86 种招式效果、83 个物品（不含 TM/HM）、248 张地图、224 个地图脚本。关键原则：

1. **数据驱动**：游戏数据与逻辑分离，便于维护和扩展
2. **忠实还原**：通过大量测试确保与原版行为一致
3. **渐进交付**：每个阶段都有可验证的交付物
4. **安全第一**：利用 Rust 类型系统在编译期消除大量错误
5. **可扩展**：为 Mod 和自定义功能预留接口
