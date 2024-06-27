use regex::Regex;
use crate::client::Flow;
use wayland_client::{EventQueue, QueueHandle};
use crate::ROUNDTRIP_EXPECT;

pub struct Scratchpad {
    to_tags: u32,
    focus_view: String,
    layout_command: Option<String>,
    replace: bool,
}

impl Scratchpad {
    pub fn new(to_tags: u32, focus_view: String, layout_command: Option<String>, replace: bool) -> Self {
        Self {
            to_tags,
            focus_view,
            layout_command,
            replace,
        }
    }
    fn set_tags(&mut self, flow: &mut Flow, queue_handle: &QueueHandle<Flow>) {
        flow.send_command(
            vec!["set-focused-tags".into(), self.to_tags.to_string()],
            queue_handle,
        );
    }
    fn toggle_tags(&mut self, flow: &mut Flow, queue_handle: &QueueHandle<Flow>) {
        flow.send_command(
            vec!["toggle-focused-tags".into(), self.to_tags.to_string()],
            queue_handle,
        );
    }
    fn focus_desired_view(&mut self, flow: &mut Flow, queue_handle: &QueueHandle<Flow>, event_queue: &mut EventQueue<Flow>) {
        let re = Regex::new(&regex::escape(&self.focus_view)).unwrap();
        while !re.is_match(&flow.focused_view.clone().unwrap()) {
            flow.send_command(
                vec!["focus-view".into(), "previous".into()],
                queue_handle,
            );
            event_queue.roundtrip(flow).expect(ROUNDTRIP_EXPECT);
        }
    }
    pub fn summon(&mut self, flow: &mut Flow, queue_handle: QueueHandle<Flow>, event_queue: &mut EventQueue<Flow>) {
        if self.replace {
            self.set_tags(flow, &queue_handle);
            self.focus_desired_view(flow, &queue_handle, event_queue)
        } else {
            self.toggle_tags(flow, &queue_handle);
            self.focus_desired_view(flow, &queue_handle, event_queue);
        }
    }
}
