# 构建一个语言 VM 

## 0 概述

### 0.0 VM 类型

- VM 解释器（*Interpreters*）包含三种类型：
  - Tree-walking > 简单、代码转换灵活 > 速度慢
  - Stack-based > 比寄存器更易于编程、性能好 > 不能映射到真实硬件、更多指令
  - Register-based > 更接近实际硬件工作方式、性能更高 > 编程复杂、寄存器分配

### 0.1 项目目标

- 构造一个基于寄存器（*Register-based*）的虚拟机。

- 功能列表如下：

1. [y] 寄存器（vm -> register）
2. [y] 指令（instruction -> instruction）

---


## 1 构造寄存器

- 寄存器一共有 32bit ：
  - 8bit 表示操作码
  - 8bit 表示寄存器编号
  - 16bit 存储数字（max of uint16 = 65536）

> 注意：
> 
> 为什么使用数组而不是向量？因为我们知道编译时需要的数字。

## 2 基本操作码

- 指令（instructions）和解码操作码（decoding opcodes）

> 什么是操作码？
> 
> 介于 0 和某个上限之间的整数。因为我们使用 8bit 来表示操作码，所以我们可以有 255 个。

### 2.1 LOAD
- `LOAD`将数字500加载到寄存器0中：

```Assembly
LOAD $0 #500
```

### 2.2 ADD & SUB & MUL & DIV

- `ADD`将寄存器内两个数相加：

```Assembly
LOAD $0 #10
LOAD $1 #15
ADD $0 $1 $2
```

- `SUB`将寄存器内两个数相减：
```Assembly
LOAD $0 #15
LOAD $1 #10
SUB $0 $1 $2
```
- `MUL`类似

- `DIV`需要存储其余部分`8 / 5 = 1 remainder 3`：

```Assembly
LOAD $0 #8
LOAD $1 #5
DIV $0 $1 $2
```

### 2.3 跳转
- 常见跳转：
1. 绝对跳转
2. 相对向前
3. 相对向后

- `JMP`绝对跳转：

```Assembly
LOAD $0 #0
JMP $0
```

- `JMPF`、`JMPB`相对跳转

### 2.4 相等性检查

- `EQ`比较两个寄存器内的值是否相等

- `JEQ`采用一个寄存器作为参数，如果为 true，则将 equal_flag 跳转到存储在该寄存器中的值。如果为 false，则不会跳转到它。

## 3 REPL设计

- 查看`repl.rs`：

```
.help
# 给寄存器0赋值500
01 00 01 F4
# 给寄存器1赋值499
01 01 01 F3
# 寄存器0 * 寄存器1
04 00 01 02
```

## 4 Assembler设计

- 加入`nom`




