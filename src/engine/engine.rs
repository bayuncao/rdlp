use crate::conf::conf::Item;
use crate::conf::conf::SINGLETON_CONF;
use crate::result::Result;
use crate::logger::logger::Logger;

use std::sync::Arc;
use tokio::task;
use crate::Conf;
use crate::conf::conf::Filter;
use crate::conf::conf::Detect;
use crate::conf::conf::Verify;
pub struct Process {
    log: Logger,
    result: Result,
}




pub struct Worker {
    conf: Arc<Conf>,
    rule: Item, // rule item in conf
    rule_type: i32, // VALUE if there is no KReg and KDict

}

impl Worker {
    pub fn new() -> Self {
        let conf = match SINGLETON_CONF.get_conf() {
            Ok(conf) => conf,
            Err(e) => {
                println!("Failed to get configuration: {}", e);
                return Err("Failed to get configuration");       
                 }
        };

          let rule = conf.rules.item[0].clone();
        let rule_type = 

        Ok(Self {
            conf: Arc::new(conf),
            rule,
            rule_type,
        })
    }

    pub async fn async_detect(
        &self,
        data: Vec<String>,
        rules: Vec<Item>,
    ) -> Result<(), &'static str> {
        let tasks: Vec<_> = data
            .into_iter()
            .map(|text| {
                let rules = Arc::new(rules.clone());
                task::spawn(async move {
                    for rule in rules.iter() {
                        // Apply the rule to the text sample
                        // This is a placeholder, replace it with your actual detection and masking logic
                        let _ = rule;
                        let _ = text;
                    }
                })
            })
            .collect();
    
        for t in tasks {
            t.await.unwrap();
        }
    
        Ok(())
    }
    
}
