use crate::storage::load_tracked_shows;

pub fn list_tracked() {
  let track_list = load_tracked_shows();
  for track in track_list {
    println!("{:07x} {}", track.id, track.name);
  }
}
