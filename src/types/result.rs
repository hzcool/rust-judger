use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::{Deserialize as Deserialize2, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum JudgeStatus {
    Accepted,
    //0
    WrongAnswer,
    //1
    TimeLimitExceeded,
    //2
    MemoryLimitExceeded,
    //3
    OutputLimitExceeded,
    //4
    RuntimeError,
    //5
    SystemError,
    //6
    CompileError,
    //-1
    BadRequst, // 80
}

#[derive(Serialize, Deserialize2, Default, Debug)]
pub struct JudgeResult {
    pub id: Option<i32>,
    pub cpu_time: u32,
    pub memory: u32,
    pub info: String,
    pub status: JudgeStatus,
}

#[derive(Serialize, Deserialize2, Default, Debug)]
pub struct SpjJudgeResult {
    pub status: JudgeStatus,
    pub info: String,
}

impl JudgeResult {
    pub fn from_system_err(id: Option<i32>, info: String) -> JudgeResult {
        JudgeResult {
            id,
            cpu_time: 0,
            memory: 0,
            info,
            status: JudgeStatus::SystemError,
        }
    }
}

impl JudgeResult {
    pub fn system_error_result(s: String) -> JudgeResult {
        JudgeResult {
            id: None,
            cpu_time: 0,
            memory: 0,
            info: s,
            status: JudgeStatus::SystemError,
        }
    }
}

impl JudgeStatus {
    pub fn from(x: i32) -> JudgeStatus {
        match x {
            0 => JudgeStatus::Accepted,
            1 => JudgeStatus::WrongAnswer,
            2 => JudgeStatus::TimeLimitExceeded,
            3 => JudgeStatus::MemoryLimitExceeded,
            4 => JudgeStatus::OutputLimitExceeded,
            5 => JudgeStatus::RuntimeError,
            -1 => JudgeStatus::CompileError,
            80 => JudgeStatus::BadRequst,
            _ => JudgeStatus::SystemError,
        }
    }
    pub fn get_i32(&self) -> i32 {
        match self {
            JudgeStatus::Accepted => 0,
            JudgeStatus::WrongAnswer => 1,
            JudgeStatus::TimeLimitExceeded => 2,
            JudgeStatus::MemoryLimitExceeded => 3,
            JudgeStatus::OutputLimitExceeded => 4,
            JudgeStatus::RuntimeError => 5,
            JudgeStatus::SystemError => 6,
            JudgeStatus::CompileError => -1,
            JudgeStatus::BadRequst => 80,
        }
    }
}

impl Default for JudgeStatus {
    fn default() -> Self {
        JudgeStatus::Accepted
    }
}

impl Serialize for JudgeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.get_i32())
    }
}

impl<'de> Deserialize<'de> for JudgeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(I32Visitor)
    }
}

struct I32Visitor;

impl<'de> Visitor<'de> for I32Visitor {
    type Value = JudgeStatus;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JudgeStatus::from(value as i32))
    }
    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JudgeStatus::from(value as i32))
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JudgeStatus::from(value as i32))
    }
    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JudgeStatus::from(value as i32))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JudgeStatus::from(value))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JudgeStatus::from(value as i32))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JudgeStatus::from(value as i32))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JudgeStatus::from(value as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let res = JudgeResult {
            id: Some(1),
            cpu_time: 1000,
            memory: 232334234,
            info: "Ok".to_string(),
            status: JudgeStatus::Accepted,
        };
        let v = serde_json::json!(res);
        println!("{}", v.to_string())
    }

    #[test]
    fn test_deserialize() {
        let s = "{\"cpu_time\":1000,\"id\":1,\"info\":\"Ok\",\"memory\":232334234,\"status\":0}";
        let res: JudgeResult = serde_json::from_str(s).unwrap();
        println!("{:?}", res);
    }
}
