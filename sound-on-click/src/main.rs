use device_query::{DeviceEvents, DeviceState};
use playback_rs::{Player, Song};

fn main() {
    let device_state = DeviceState::new();
    let audio_player = Player::new(None).expect("Unable to create audio player");
    let audio = Song::from_file("audio.mp3", None).expect("Unable to access the audio");

    let _listen_for_mouse_click = device_state.on_mouse_down(|button| {
        println!("Mouse button down: {:?}", button);
        audio_player
            .play_song_next(&audio, Some(std::time::Duration::from_secs(0)))
            .expect("Unable to play song");
    });

    loop {}
}
