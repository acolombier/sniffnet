//! Module defining the `Sniffer` struct, which trace gui's component statuses and permits
//! to share data among the different threads.

use pcap::Device;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};

use crate::enums::language::Language;
use crate::enums::overlay::MyOverlay;
use crate::enums::report_type::ReportType;
use crate::enums::running_page::RunningPage;
use crate::enums::status::Status;
use crate::structs::filters::Filters;
use crate::structs::notifications::Notifications;
use crate::{InfoTraffic, RunTimeData, StyleType, TrafficChart};

/// Struct on which the gui is based
///
/// It contains gui statuses and network traffic statistics to be shared among the different threads
pub struct Sniffer {
    /// Capture number, incremented at every new run
    pub current_capture_id: Arc<Mutex<u16>>,
    /// Capture data updated by thread parsing packets
    pub info_traffic: Arc<Mutex<InfoTraffic>>,
    /// Status of the application (init or running) and the associated condition variable
    pub status_pair: Arc<(Mutex<Status>, Condvar)>,
    /// Traffic data displayed in GUI
    pub runtime_data: Rc<RefCell<RunTimeData>>,
    /// Network adapter to be analyzed
    pub device: Device,
    /// Active filters on the observed traffic
    pub filters: Filters,
    /// Signals if a pcap error occurred
    pub pcap_error: Option<String>,
    /// Application style (only values Day and Night are possible for this field)
    pub style: StyleType,
    /// Waiting string
    pub waiting: String,
    /// Chart displayed
    pub traffic_chart: TrafficChart,
    /// Report type to be displayed
    pub report_type: ReportType,
    /// Currently displayed overlay; None if no overlay is displayed
    pub overlay: Option<MyOverlay>,
    /// Contains the notifications configuration set by the user
    pub notifications: Notifications,
    /// Defines the current running page
    pub running_page: RunningPage,
    /// Language used in the GUI
    pub language: Language,
}
