// iterm.rs 处理repl业务类型

/// 业务枚举类型
pub enum Items {
    /// .hello：打招呼业务
    HELLO,
    /// .quit：退出程序业务
    QUIT,
    /// .program：显示程序字节码业务
    PROGRAM,
    /// .registers：显示寄存器参数业务
    REGISTERS,
    /// .help: 帮助业务
    HELP,
    /// .history: 历史记录
    HISTORY,
    /// 其他：无效输入
    INVALID,
}

/// 业务转换：由命令转换为业务结构体
impl From<&str> for Items {
    fn from(value: &str) -> Self {
        match value {
            ".hello" => return Items::HELLO,
            ".quit" => return Items::QUIT,
            ".program" => return Items::PROGRAM,
            ".registers" => return Items::REGISTERS,
            ".help" => return Items::HELP,
            ".history" => return Items::HISTORY,
            _ => return Items::INVALID,
        }
    }
}

impl Items {
    /// 打印业务
    pub fn print_items() {
        println!("--------------- Iterms --------------");
        println!(".hello: 打招呼
.quit: 退出程序
.program: 显示程序字节码
.registers: 显示寄存器参数
.history: 显示有效历史命令
.help: 帮助");
    }
}