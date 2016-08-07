use portmidi::MidiEvent;

pub trait MidiDestination {
    fn process_events(&mut self, events: &[MidiEvent]) {}
}
