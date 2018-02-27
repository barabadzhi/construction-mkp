use std::fmt;
use std::time::Duration;

use colored::*;

#[derive(Default, Debug)]
pub struct Statistics {
    pub total_profit: u32,
    pub picked_items: Vec<String>,
    pub utilization: Vec<String>,
    pub duration: Duration,
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut run_time = String::new();

        if self.duration.as_secs() > 0 {
            run_time += &format!("{} {}", self.duration.as_secs().to_string().green(), 's');
        }

        if self.duration.subsec_nanos() > 0 {
            if run_time.is_empty() {
                run_time += &format!(
                    "{} {}",
                    self.duration.subsec_nanos().to_string().green(),
                    "ns"
                )
            } else {
                run_time += &format!(
                    " {} {}",
                    self.duration.subsec_nanos().to_string().green(),
                    "ns"
                )
            }
        }

        write!(
            f,
            r#"
    -> Total profit: {}
    -> Items ({}): {}
    -> Utilization: {}    
    -> Duration: {}
"#,
            self.total_profit.to_string().green(),
            self.picked_items.len(),
            &self.picked_items.join(", ").yellow(),
            self.utilization.join(" ").blue(),
            run_time
        )
    }
}
