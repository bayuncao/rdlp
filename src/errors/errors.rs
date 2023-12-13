use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RdlpError {
    ErrNotReach,
    ErrHasNotConfiged,
    ErrConfEmpty,
    ErrConfUrlEmpty,
    ErrConfPathEmpty,
    ErrDisableRuleFailed,
    ErrAuthFailed,
    ErrRemoteCfgFailed,
    ErrProcessAfterClose,
    ErrNewEngineDebug,
    ErrMaxInputLimit,
    ErrPositionError,
    ErrRegexNeedString,
    ErrRegexCompileFailed,
    ErrDictNeedStringArray,
    ErrReEmpty,
    ErrMaskWorkerNotFound,
    ErrLoadMaskNameConflict,
    ErrPanic,
    ErrMaskNotSupport,
    ErrMaskFailed,
    ErrMaskTagNotSupport,
    ErrMaskNameConflict,
    ErrMaskRuleNotFound,
    ErrDataMarshal,
    ErrSendRequest,
    ErrMaskStructInput,
    ErrMaskStructOutput,
    ErrOnlyForLog,
    // Add more errors as needed
}

impl fmt::Display for RdlpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RdlpError::ErrNotReach => write!(f, "not reach"),
            RdlpError::ErrHasNotConfiged => write!(f, "has not configed"),
            RdlpError::ErrConfEmpty => write!(f, "conf empty"),
            RdlpError::ErrConfUrlEmpty => write!(f, "conf url empty"),
            RdlpError::ErrConfPathEmpty => write!(f, "conf path empty"),
            RdlpError::ErrDisableRuleFailed => write!(f, "disable rule failed"),
            RdlpError::ErrAuthFailed => write!(f, "auth failed"),
            RdlpError::ErrRemoteCfgFailed => write!(f, "remote cfg failed"),
            RdlpError::ErrProcessAfterClose => write!(f, "process after close"),
            RdlpError::ErrNewEngineDebug => write!(f, "new engine debug"),
            RdlpError::ErrMaxInputLimit => write!(f, "max input limit"),
            RdlpError::ErrPositionError => write!(f, "position error"),
            RdlpError::ErrRegexNeedString => write!(f, "regex need string"),
            RdlpError::ErrRegexCompileFailed => write!(f, "regex compile failed"),
            RdlpError::ErrDictNeedStringArray => write!(f, "dict need string array"),
            RdlpError::ErrReEmpty => write!(f, "re empty"),
            RdlpError::ErrMaskWorkerNotFound => write!(f, "mask worker not found"),
            RdlpError::ErrLoadMaskNameConflict => write!(f, "load mask name conflict"),
            RdlpError::ErrPanic => write!(f, "panic"),
            RdlpError::ErrMaskNotSupport => write!(f, "mask not support"),
            RdlpError::ErrMaskFailed => write!(f, "mask failed"),
            RdlpError::ErrMaskTagNotSupport => write!(f, "mask tag not support"),
            RdlpError::ErrMaskNameConflict => write!(f, "mask name conflict"),
            RdlpError::ErrMaskRuleNotFound => write!(f, "mask rule not found"),
            RdlpError::ErrDataMarshal => write!(f, "data marshal"),
            RdlpError::ErrSendRequest => write!(f, "send request"),
            RdlpError::ErrMaskStructInput => write!(f, "mask struct input"),
            RdlpError::ErrMaskStructOutput => write!(f, "mask struct output"),
            RdlpError::ErrOnlyForLog => write!(f, "only for log"),
        }
    }
} // impl fmt::Display for RdlpError
