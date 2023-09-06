// vm.rs：一个模拟硬件CPU功能的虚拟机
use crate::instruction::Opcode;

/// VM：虚拟机结构体
pub struct VM {
    /// 寄存器：存放32个int32类型的值
    registers: [i32; 32],
    /// 程序计数器：指向下一条指令地址
    pc: usize,
    /// 程序字节码：存放待解码的程序
    program: Vec<u8>,
    /// 存储除法的余数
    remainder: u32,
    /// 存储比较结果的布尔值
    equal_flag: bool,
}

impl VM {
    /// 初始化：将所有寄存器初始化为0
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
        }
    }

    /// 循环执行函数：虚拟机执行操作码
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    /// 单执行函数：执行一次指令
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    /// 指令执行函数
    fn execute_instruction(&mut self) -> bool {
        // 如果pc超过了程序本身长度，则返回true终止执行
        if self.pc >= self.program.len() {
            return true;
        }
        // 第一次解码
        match self.decode_opcode() {
            /*
             * HLT 0：合法操作码
             * */
            Opcode::HLT => {
                self.next_8_bits();
                self.next_16_bits();
                println!("HLT encountered");
            },
            /* 
             * LOAD 1：加载操作码
             * 步骤：
             * 1. 解码前 8 位并查看 LOAD（已完成）
             * 2. 解码接下来的 8 位并使用它来获取寄存器编号
             * 3. 将接下来的 16 位分成两个 u8 解码
             * 4. 将其数据存储在寄存器内
             * */ 
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
                println!("LOAD ${register} #{number}");
            },
            /*
             * ADD 2：加法操作码
             * 步骤：
             * 1. 解码前 8 位查看 ADD（已完成）
             * 2. 解码接下来三个 8 位作为寄存器编号
             * 3. 将前两个寄存器内值加到第三个寄存器上
             */
            Opcode::ADD => {
                let r1 = self.next_8_bits() as usize;
                let r2 = self.next_8_bits() as usize;
                let r3 = self.next_8_bits() as usize;
                self.registers[r3] = self.registers[r1] + self.registers[r2];
                println!("ADD ${r1} ${r2} ${r3}");
            },
            /*
             * SUB 3：减法操作码
             * 步骤：
             * 1. 解码前 8 位查看 ADD（已完成）
             * 2. 解码接下来三个 8 位作为寄存器编号
             * 3. 第一个寄存器值减去第二个存到第三个寄存器上
             */ 
            Opcode::SUB => {
                let r1 = self.next_8_bits() as usize;
                let r2 = self.next_8_bits() as usize;
                let r3 = self.next_8_bits() as usize;
                self.registers[r3] = self.registers[r1] - self.registers[r2];
                println!("SUB ${r1} ${r2} ${r3}");
            },
            /*
             * MUL 4：乘法操作码
             * 步骤：
             * 1. 解码前 8 位查看 ADD（已完成）
             * 2. 解码接下来三个 8 位作为寄存器编号
             * 3. 第一个寄存器值与第二个相乘存到第三个寄存器上
             */ 
            Opcode::MUL => {
                let r1 = self.next_8_bits() as usize;
                let r2 = self.next_8_bits() as usize;
                let r3 = self.next_8_bits() as usize;
                self.registers[r3] = self.registers[r1] * self.registers[r2];
                println!("MUL ${r1} ${r2} ${r3}");
            },
            /*
             * DIV 5：除法操作码
             * 步骤：
             * 1. 解码前 8 位查看 ADD（已完成）
             * 2. 解码接下来三个 8 位作为寄存器编号
             * 3. 第一个寄存器值除第二个存到第三个寄存器上
             * 4. 将余数存到remainder属性里
             */ 
            Opcode::DIV => {
                let r1 = self.next_8_bits() as usize;
                let r2 = self.next_8_bits() as usize;
                let r3 = self.next_8_bits() as usize;
                self.registers[r3] = self.registers[r1] / self.registers[r2];
                self.remainder = (self.registers[r1] % self.registers[r2]) as u32;
                println!("DIV ${r1} ${r2} ${r3}");
            },
            /*
             * JMP 6：绝对跳转操作码
             * 步骤：
             * 1. 解码前 8 位查看 JMP（已完成）
             * 2. 解码接下来的 8 位作为寄存器编号
             * 3. 访问寄存器内的值作为 pc 值
             */
            Opcode::JMP => {
                let r = self.next_8_bits() as usize;
                self.pc = self.registers[r] as usize;
                println!("JMP ${r}");
            },
            /*
             * JMPF 7：向前跳转操作码
             * 步骤：
             * 1. 解码前 8 位查看 JMPF（已完成）
             * 2. 解码接下来的 8 位作为寄存器编号
             * 3. 访问寄存器内的值作为 pc 向前跳转的值
             */
            Opcode::JMPF => {
                let r = self.next_8_bits() as usize;
                self.pc += self.registers[r] as usize;
                println!("JMPF ${r}");
            },
            /*
             * JMPB 8：向后跳转操作码
             * 步骤：
             * 1. 解码前 8 位查看 JMPB（已完成）
             * 2. 解码接下来的 8 位作为寄存器编号
             * 3. 访问寄存器内的值作为 pc 向后跳转的值
             */
            Opcode::JMPB => {
                let r = self.next_8_bits() as usize;
                self.pc -= self.registers[r] as usize;
                println!("JMPB ${r}");
            },
            /*
             * EQ 9：等于操作码
             * 步骤：
             * 1. 解码前 8 位查看 JMPB（已完成）
             * 2. 解码接下来的 2*8 位作为寄存器编号
             * 3. 比较寄存器内存储的值
             */
            Opcode::EQ => {
                let r1 = self.next_8_bits() as usize;
                let r2 = self.next_8_bits() as usize;
                if self.registers[r1] == self.registers[r2] {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
                println!("EQ ${r1} ${r2}");
            },
            /*
             * JEQ 10：跳转等于操作码
             * 步骤：
             * 1. 解码前 8 位查看 JEQ（已完成）
             * 2. 解码接下来的 8 位作为寄存器编号
             * 3. 查看equal_flag，如果为真就跳转
             */
            Opcode::JEQ => {
                let r = self.next_8_bits() as usize;
                if self.equal_flag {
                    self.pc = self.registers[r] as usize;
                }
                println!("JEQ ${r}");
            },
            /*
             * JNEQ 11：跳转不等操作码
             * 步骤：
             * 1. 解码前 8 位查看 JEQ（已完成）
             * 2. 解码接下来的 8 位作为寄存器编号
             * 3. 查看equal_flag，如果为假就跳转
             */
            Opcode::JNEQ => {
                let r = self.next_8_bits() as usize;
                if !self.equal_flag {
                    self.pc = self.registers[r] as usize;
                }
                println!("JNEQ ${r}");
            },
            // 其他
            _ => {
                println!("IGL encountered");
                return true;
            }
        }
        false
    }

    // --- 工具函数 ---
    /// 解码：将程序字节码解析为操作码
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;   // 解码之后，pc移动到下一子节
        return opcode;
    }

    /// 打印：显示所有寄存器参数
    pub fn print_registers(&mut self) {
        println!("Listing registers and all contents:");
        println!("{:#?}", self.registers);
        println!("End of Register Listing");
    }

    /// 打印：显示整个程序向量的字节码
    pub fn print_program(&mut self) {
        println!("Listing instructions currently in VM's program vector:");
        for instruction in &self.program {
            println!("{}", instruction);
        }
        println!("End of Program Listing");
    }

    /// 输入单个字节，加入程序字节码
    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }


    // --- 辅助函数 ---
    /// 获取下一个 8bit 程序字节码
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }
    /// 获取下一个 16bit 程序字节码
    fn next_16_bits(&mut self) -> u16 {
        let result = 
            ((self.program[self.pc] as u16) << 8) 
            | (self.program[self.pc + 1] as u16);
        self.pc += 2;
        return result;
    }
}


/// vm 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    /// HLT单元测试: code 0
    fn test_hlt_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    /// IGL单元测试: code else
    fn test_igl_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    /// LOAD单元测试: code 1
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    /// ADD单元测试: code 2
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 0, 10, 1, 1, 0, 15, 2, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 25);
    }

    #[test]
    /// SUB单元测试: code 3
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 0, 15, 1, 1, 0, 10, 3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    /// MUL单元测试: code 4
    fn test_mul_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 0, 15, 1, 1, 0, 10, 4, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 150);
    }

    #[test]
    /// DIV单元测试: code 5
    fn test_div_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 0, 8, 1, 1, 0, 5, 5, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 3);
    }

    #[test]
    /// JMP单元测试：code 6
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    /// JMPF单元测试：code 7
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 1, 1, 0, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[1], 3);
    }

    #[test]
    /// JMPB单元测试：code 8
    fn test_jmpb_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 6;
        test_vm.registers[1] = 8;
        test_vm.program = vec![7, 0, 1, 2, 0, 8, 255, 0, 8, 1];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 8);
    }

    #[test]
    /// EQ单元测试：code 9
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    /// JEQ单元测试：code 10
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 4;
        test_vm.equal_flag = true;
        test_vm.program = vec![10, 0, 3, 2, 1, 1, 0, 9];
        test_vm.run();
        assert_eq!(test_vm.registers[1], 9);
    }

    #[test]
    /// JNEQ单元测试：code 11
    fn test_jneq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 4;
        test_vm.equal_flag = false;
        test_vm.program = vec![11, 0, 3, 2, 1, 1, 0, 9];
        test_vm.run();
        assert_eq!(test_vm.registers[1], 9);
    }

}