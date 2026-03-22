# Demo分支视觉完善计划

## 已完成 ✅
- [x] M1-M10: 全部游戏逻辑+渲染+音频+测试+WASM
- [x] Demo分支: ResourceManager接入官方gfx素材
- [x] Oak演讲画面修复: 正确素材+居中+文字框+分阶段渲染
- [x] CLI截图系统: screenshot/screenshot-all子命令
- [x] 全部9个画面渲染坐标对齐原版ASM(hlcoord)

## 里程碑 D1: 文字框使用真实瓦片边框（替换1px线条）
**目标**: `draw_text_box()` 当前用1px线条画边框，应该用Game Boy原版的8×8瓦片边框字符
**文件**: `pokered-rust/crates/pokered-app/src/main.rs` (draw_text_box函数)
**参考**: `pokered-renderer/src/textbox.rs` — TextBoxFrame已有边框瓦片定义
**验收**: 截图显示粗体瓦片边框（类似原版对话框外观）

## 里程碑 D2: 大地图渲染改进
**目标**: 大地图目前只是从(0,0)铺瓦片，没有摄像机/视口逻辑，玩家精灵位置不对
**任务**:
- 玩家精灵居中在屏幕中央（160x144像素的中心）
- 实现简单视口：以玩家为中心裁剪地图瓦片
- 显示地图名称（进入时短暂显示）
- NPC精灵在正确位置渲染
**文件**: `pokered-rust/crates/pokered-app/src/main.rs` (draw_overworld函数)
**参考**: `pokered-renderer/src/viewport.rs`, `pokered-core/src/overworld/`
**验收**: 截图显示玩家居中、周围有正确地图瓦片、NPC可见

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
