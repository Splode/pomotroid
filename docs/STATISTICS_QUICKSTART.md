# 统计分析系统 - 快速开始指南

## 安装依赖

由于所有功能都使用内置功能实现，不需要安装额外的依赖。

## 启动开发环境

```powershell
# 安装项目依赖（如果还没安装）
npm install

# 启动开发服务器
npm run dev
```

## 快速测试

### 1. 测试数据记录

1. 启动应用后，点击开始按钮启动一个番茄钟
2. 等待几秒后点击重置按钮
3. 会弹出中断原因对话框，选择一个原因或跳过
4. 数据已被记录

### 2. 查看统计

1. 点击底部导航栏的**统计图标**（📊）
2. 默认显示"今日"统计视图
3. 切换到"本周"、"本月"、"历史"查看不同维度的统计

### 3. 生成测试数据

为了更好地展示统计功能，建议创建一些测试数据：

**方法1：手动创建**
- 完成多个番茄钟（正常完成或中断）
- 在不同时段进行，以便热力图有数据

**方法2：使用浏览器控制台**
打开开发者工具（F12），在控制台输入：

```javascript
// 创建最近7天的测试数据
const store = require('@/utils/StatisticsStore').getStatisticsStore()
const today = new Date()

for (let day = 0; day < 7; day++) {
  const date = new Date(today)
  date.setDate(today.getDate() - day)
  
  // 每天随机创建3-8个番茄钟
  const count = Math.floor(Math.random() * 6) + 3
  
  for (let i = 0; i < count; i++) {
    const hour = Math.floor(Math.random() * 12) + 8 // 8-20点
    const session = {
      id: Math.random().toString(36).substr(2, 9),
      type: 'work',
      duration: 25,
      startTime: new Date(date.setHours(hour, 0, 0, 0)).toISOString(),
      endTime: new Date(date.setHours(hour, 25, 0, 0)).toISOString(),
      completed: Math.random() > 0.2, // 80%完成率
      interrupted: Math.random() < 0.2,
      interruptReason: Math.random() < 0.2 ? ['紧急事项', '会议', '电话'][Math.floor(Math.random() * 3)] : null,
      taskName: null,
      taskId: null
    }
    store.data.sessions.push(session)
  }
}

store.saveData()
console.log('测试数据已生成！刷新统计页面查看。')
```

## 功能验证清单

- [ ] 番茄钟启动时创建会话记录
- [ ] 番茄钟完成时正确标记
- [ ] 重置时弹出中断对话框
- [ ] 中断原因可选择或自定义
- [ ] 今日统计显示正确
- [ ] 周统计柱状图显示正确
- [ ] 月统计折线图和日历显示正确
- [ ] 历史总览显示累计数据
- [ ] 热力图显示工作时段分布
- [ ] 中断统计显示干扰因素
- [ ] 成就系统根据数据解锁
- [ ] 视图切换流畅
- [ ] 数据持久化到文件

## 数据文件位置

统计数据保存在：
- **Windows**: `%APPDATA%/pomotroid/pomodoro-sessions.json`
- **macOS**: `~/Library/Application Support/pomotroid/pomodoro-sessions.json`
- **Linux**: `~/.config/pomotroid/pomodoro-sessions.json`

## 调试技巧

### 查看当前会话

```javascript
// 在浏览器控制台
this.$store.getters.currentSession
```

### 查看所有统计数据

```javascript
// 日统计
this.$store.getters.dayStats

// 周统计
this.$store.getters.weekStats

// 月统计
this.$store.getters.monthStats

// 历史统计
this.$store.getters.historyStats
```

### 手动触发数据刷新

```javascript
this.$store.dispatch('refreshStats')
```

### 清空所有数据（谨慎使用）

```javascript
this.$store.dispatch('clearAllData')
```

## 常见问题

### Q: 统计页面没有数据？
A: 确保已经完成或中断过至少一个工作番茄钟。休息时段不会被统计。

### Q: 热力图显示空白？
A: 热力图需要最近4周内有完成的番茄钟数据。

### Q: 成就没有解锁？
A: 检查是否满足成就条件，刷新历史页面查看。

### Q: 数据丢失了？
A: 检查用户数据目录下的 `pomodoro-sessions.json` 文件是否存在。

### Q: 如何备份数据？
A: 复制 `pomodoro-sessions.json` 文件到安全位置。

### Q: 如何恢复数据？
A: 将备份的 `pomodoro-sessions.json` 文件复制回用户数据目录。

## 性能建议

- 数据量超过1000条记录时，建议定期导出归档
- 大量数据可能影响启动速度，可考虑实现数据分片
- 复杂计算（如月视图）会在第一次加载时较慢，后续会使用缓存

## 下一步

1. 使用应用几天，积累真实数据
2. 根据统计洞察调整工作习惯
3. 关注中断原因，减少干扰
4. 利用热力图找到最佳工作时段
5. 设定并追踪连续打卡目标

## 贡献建议

如果发现问题或有改进建议：
1. 记录问题详情（截图、错误日志）
2. 提出功能改进建议
3. 考虑优化用户体验的方案

---

**祝你使用愉快！保持专注，提升效率！🍅**
