use crate::detections::message::{LOGONSUMMARY_FLAG, METRICS_FLAG};
use crate::detections::{configs::CONFIG, detection::EvtxRecordInfo};
use comfy_table::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;

use super::metrics::EventMetrics;
use hashbrown::HashMap;

#[derive(Debug)]
pub struct Timeline {
    pub stats: EventMetrics,
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Timeline {
    pub fn new() -> Timeline {
        let totalcnt = 0;
        let filepath = String::default();
        let starttm = String::default();
        let endtm = String::default();
        let statslst = HashMap::new();
        let statsloginlst = HashMap::new();

        let statistic =
            EventMetrics::new(totalcnt, filepath, starttm, endtm, statslst, statsloginlst);
        Timeline { stats: statistic }
    }

    pub fn start(&mut self, records: &[EvtxRecordInfo]) {
        self.stats.evt_stats_start(records);
        self.stats.logon_stats_start(records);
    }

    pub fn tm_stats_dsp_msg(&mut self) {
        if !*METRICS_FLAG {
            return;
        }
        // 出力メッセージ作成
        let mut sammsges: Vec<String> = Vec::new();
        sammsges.push("---------------------------------------".to_string());
        sammsges.push(format!("Evtx File Path: {}", self.stats.filepath));
        sammsges.push(format!("Total Event Records: {}\n", self.stats.total));
        sammsges.push(format!("First Timestamp: {}", self.stats.start_time));
        sammsges.push(format!("Last Timestamp: {}\n", self.stats.end_time));
        
        let mut stats_tb = Table::new();
        stats_tb.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS)
        .set_style(TableComponent::VerticalLines, ' ');
        stats_tb.set_header(vec!["Count", "Percent", "ID", "Event"]);


        // 集計件数でソート
        let mut mapsorted: Vec<_> = self.stats.stats_list.iter().collect();
        mapsorted.sort_by(|x, y| y.1.cmp(x.1));

        // イベントID毎の出力メッセージ生成
        let stats_msges: Vec<Vec<String>> = self.tm_stats_set_msg(mapsorted);

        for msgprint in sammsges.iter() {
            println!("{}", msgprint);
        }
        stats_tb.add_rows(stats_msges);
        println!("{stats_tb}");
    }

    pub fn tm_logon_stats_dsp_msg(&mut self) {
        if !*LOGONSUMMARY_FLAG {
            return;
        }
        // 出力メッセージ作成
        let mut sammsges: Vec<String> = Vec::new();
        sammsges.push("---------------------------------------".to_string());
        sammsges.push(format!("Evtx File Path: {}", self.stats.filepath));
        sammsges.push(format!("Total Event Records: {}\n", self.stats.total));
        sammsges.push(format!("First Timestamp: {}", self.stats.start_time));
        sammsges.push(format!("Last Timestamp: {}\n", self.stats.end_time));
        sammsges.push("---------------------------------------".to_string());
        for msgprint in sammsges.iter() {
            println!("{}", msgprint);
        }

        self.tm_loginstats_tb_set_msg();
    }

    // イベントID毎の出力メッセージ生成
    fn tm_stats_set_msg(&self, mapsorted: Vec<(&std::string::String, &usize)>) -> Vec<Vec<String>> {
        let mut msges: Vec<Vec<String>> = Vec::new();

        for (event_id, event_cnt) in mapsorted.iter() {
            // 件数の割合を算出
            let rate: f32 = **event_cnt as f32 / self.stats.total as f32;

            // イベント情報取得(eventtitleなど)
            let conf = CONFIG
                .read()
                .unwrap()
                .event_timeline_config
                .get_event_id(*event_id)
                .is_some();
            // event_id_info.txtに登録あるものは情報設定
            // 出力メッセージ1行作成
            if conf {
                msges.push(vec!
                    [event_cnt.to_string(),
                    format!("{:.1}%", (rate * 1000.0).round() / 10.0),
                    event_id.to_string(),
                    CONFIG
                        .read()
                        .unwrap()
                        .event_timeline_config
                        .get_event_id(*event_id)
                        .unwrap()
                        .evttitle.to_string(),
                    ]
                );
            } else {
                msges.push(
                    vec![
                    event_cnt.to_string(),
                    format!("{:.1}%", (rate * 1000.0).round() / 10.0),
                    event_id.to_string(),
                    "Unknown".to_string(),
                ]);
            }
        }
        msges
    }

    /// ユーザ毎のログイン統計情報出力メッセージ生成
    fn tm_loginstats_tb_set_msg(&self) {
        println!("Logon Summary");
        if self.stats.stats_login_list.is_empty() {
            let mut loginmsges: Vec<String> = Vec::new();
            loginmsges.push("-----------------------------------------".to_string());
            loginmsges.push("|     No logon events were detected.    |".to_string());
            loginmsges.push("-----------------------------------------\n".to_string());
            for msgprint in loginmsges.iter() {
                println!("{}", msgprint);
            }
        } else {
            let mut logins_stats_tb = Table::new();
            logins_stats_tb.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS)
            .set_style(TableComponent::VerticalLines, ' ');
            logins_stats_tb.set_header(vec!["User", "Failed", "Successful"]);
            // 集計件数でソート
            let mut mapsorted: Vec<_> = self.stats.stats_login_list.iter().collect();
            mapsorted.sort_by(|x, y| x.0.cmp(y.0));

            for (key, values) in &mapsorted {
                let mut username: String = key.to_string();
                //key.to_string().retain(|c| c != '\"');
                //key.to_string().pop();
                username.pop();
                username.remove(0);
                logins_stats_tb.add_row(vec![
                    Cell::new(&username),
                    Cell::new(&values[1].to_string()),
                    Cell::new(&values[0].to_string()),
                ]);
            }
            println!("{logins_stats_tb}");
            println!();
        }
    }
}
