# Demo分支视觉完善计划

## 已完成 ✅
- [x] M1-M10: 全部游戏逻辑+渲染+音频+测试+WASM
- [x] Demo分支: ResourceManager接入官方gfx素材
- [x] Oak演讲画面修复: 正确素材+居中+文字框+分阶段渲染
- [x] CLI截图系统: screenshot/screenshot-all子命令
- [x] 全部9个画面渲染坐标对齐原版ASM(hlcoord)

## 里程碑 D1: 文字框使用真实瓦片边框（替换1px线条） ✅ 已完成
**已完成**: draw_text_box改用8×8瓦片渲染(2px圆角边框)+box_tiles模块(8种边框位图)+draw_glyph/fill_tile通用函数
**提交**: a8da46ab

## 里程碑 D2: 大地图渲染改进 ✅ 已完成
**已完成**: draw_overworld重写 — blit_single_tile()瓦片渲染+demo_overworld_tile()32×32示例地图+视口以玩家为中心(20×18可见)+玩家精灵正确提取16×16帧(Direction)+地图名称弹窗居中显示
**提交**: 待提交

## 里程碑 D3: 战斗画面动态数据
**目标**: 战斗画面目前用硬编码的宝可梦名字/HP，应从BattleScreen状态读取
**任务**:
- 从state.enemy_pokemon/player_pokemon读取名字、等级、HP
- HP条按实际HP比例绘制（不同颜色：绿/黄/红）
- 显示正确的宝可梦精灵（根据species加载对应front/back sprite）
**文件**: `pokered-rust/crates/pokered-app/src/main.rs` (draw_battle函数)
**参考**: `pokered-core/src/battle/mod.rs` (BattleScreen结构)
**验收**: 截图显示实际宝可梦数据、HP条颜色正确

## 里程碑 D4: WASM构建同步更新
**目标**: pokered-web的main.rs可能还是旧版渲染代码，需要同步更新
**文件**: `pokered-rust/crates/pokered-web/src/main.rs`
**验收**: `cargo build -p pokered-web --target wasm32-unknown-unknown` 通过

## 里程碑 D5: 音频播放接入
**目标**: 连接pokered-audio到主循环，播放BGM和音效
**任务**:
- 标题画面播放Title BGM
- 大地图播放对应地图BGM
- 战斗播放战斗BGM
- 按键音效
**参考**: `pokered-audio/src/` (已有完整音频引擎)
**验收**: 运行游戏能听到音乐和音效
