// instruction.rs：指令

// 操作码
#[derive(Debug, PartialEq)]
pub enum Opcode {
    /// 合法操作码
    HLT,
    /// 加载操作码
    LOAD, 
    /// 加法操作码
    ADD,
    /// 减法操作码
    SUB,
    /// 乘法操作码
    MUL,
    /// 除法操作码
    DIV,
    /// 绝对跳转操作码
    JMP,
    /// 向前跳转操作码
    JMPF,
    /// 向后跳转操作码
    JMPB,
    /// 等于操作码
    EQ,
    /// 跳转等于操作码
    JEQ,
    /// 跳转不等操作码
    JNEQ,
    /// 非法操作码  
    IGL,
    // 加了这里后，记得添加 from 特征
}

/** 操作码的from特征：
 * 如果 VM 遇到非操作码数
 * 返回 IGL（非法的缩写）操作码*/
impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::ADD,
            3 => return Opcode::SUB,
            4 => return Opcode::MUL,
            5 => return Opcode::DIV,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            9 => return Opcode::EQ,
            10 => return Opcode::JEQ,
            11 => return Opcode::JNEQ,
            _ => return Opcode::IGL,
        }
    }
}

/// 指令
#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    /// 初始化：输入操作码以初始化指令
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode: opcode
        }
    }
}


// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }

}
