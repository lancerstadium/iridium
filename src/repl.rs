// repl.rs：交互式命令操作
use std::num::ParseIntError;
use std::io::{self, Write};

use crate::vm::VM;
use crate::item::Items;

/// REPL：交互式界面，读取 - 执行 - 打印输出 - 循环
pub struct REPL{
    /// 交互式界面内的虚拟机
    vm: VM,
    /// 交互式界面内存储的字符串，用于接收用户输入
    command_buffer: Vec<String>,
}


impl REPL {
    /// 初始化交互式界面
    pub fn new() -> REPL{
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    /// REPL交互式运行
    pub fn run(&mut self) {
        println!("Welcome to iridium-v1.0.0 !");
        // 主循环
        loop {
            let mut temp_buffer = String::new();
            print!(">>> ");
            io::stdout().flush().expect("Unable tp read flush stdout");
            let _ = io::stdin().read_line(&mut temp_buffer);
            self.exec_item(temp_buffer.trim());
        }
    }
    
    /// 执行交互式界面业务
    fn exec_item(&mut self, temp_buffer : &str) {
        let item = Items::from(temp_buffer);
        match item {
            /*
             * .hello：打招呼业务
             */
            Items::HELLO => {
                println!("Hello, world!");
                self.add_command_buffer(temp_buffer);
            },
            /*
             * .quit：退出程序业务
             */
            Items::QUIT => {
                println!("Farewell! Have a great day!");
                std::process::exit(-1);
            },
            /*
             * .program：显示程序字节码业务
             */
            Items::PROGRAM => {
                self.vm.print_program();
                self.add_command_buffer(temp_buffer);
            },
            /*
             * .registers：显示寄存器参数业务
             */
            Items::REGISTERS => {
                self.vm.print_registers();
                self.add_command_buffer(temp_buffer);
            },
            /*
             * .help：帮助业务
             */
            Items::HELP => {
                Items::print_items();
                self.add_command_buffer(temp_buffer);
            },
            /*
             * .history：历史记录业务
             */
            Items::HISTORY => {
                self.print_history_command();
                self.add_command_buffer(temp_buffer);
            },
            /*
             * 其他：无效输入
             */
            _ => {
                let results = self.parse_hex(temp_buffer);
                match results {
                    Ok(bytes) => {
                        for byte in bytes {
                            self.vm.add_byte(byte);
                        }
                        self.add_command_buffer(temp_buffer);
                    },
                    Err(_e) => {
                        println!("Invalid input: {:?}", temp_buffer);
                    }
                };
                self.vm.run_once();
            }
        }
    }

    // --- 工具函数 ---
    /// 解析器：解析16进制指令的输入
    fn parse_hex(&mut self, temp_buffer : &str) -> Result<Vec<u8>, ParseIntError> {
        let split = temp_buffer.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }

    // --- 辅助函数 ---
    /// 显示历史记录
    fn print_history_command(&mut self) {
        for command in &self.command_buffer {
            println!("{command}");
        }
    }

    /// 将命令加入command_buffer容器
    fn add_command_buffer(&mut self, temp_buffer : &str) {
        self.command_buffer.push(temp_buffer.to_string());
    }

}

