pub use general_audio::*;

#[cfg(any(
    not(any(feature = "general_audio_native", feature = "general_audio_web")),
    all(feature = "general_audio_native", feature = "general_audio_web"),
))]
mod implementation {
    use super::*;
    pub struct StaticAudioPlayer(());
    pub struct StaticHandle(());
    pub struct StaticSound(());

    const WARNING: &str = "using null implementation of StaticAudioPlayer";

    pub mod backend {
        #[cfg(all(feature = "general_audio_native", feature = "general_audio_web"))]
        pub use general_audio_native::*;
        #[cfg(all(feature = "general_audio_native", feature = "general_audio_web"))]
        pub use general_audio_web::*;
    }

    impl StaticHandle {
        pub fn set_volume(&self, _volume: f32) {}
        pub fn volume(&self) -> f32 {
            0.0
        }
        pub fn pause(&self) {}
        pub fn play(&self) {}
        pub fn background(self) {}
    }

    impl AudioHandle for StaticHandle {
        fn set_volume(&self, _volume: f32) {}
        fn volume(&self) -> f32 {
            0.0
        }
        fn pause(&self) {}
        fn play(&self) {}
        fn background(self) {}
    }

    impl StaticAudioPlayer {
        pub fn new<A>(_: A) -> Self {
            Self(())
        }
        pub fn play(&self, _sound: &StaticSound) -> StaticHandle {
            StaticHandle(())
        }
        pub fn play_loop(&self, _sound: &StaticSound) -> StaticHandle {
            StaticHandle(())
        }
        pub fn load_sound(&self, _bytes: &'static [u8]) -> StaticSound {
            StaticSound(())
        }
    }

    impl AudioPlayer for StaticAudioPlayer {
        type Sound = StaticSound;
        type Handle = StaticHandle;
        fn play(&self, _sound: &Self::Sound) -> Self::Handle {
            StaticHandle(())
        }
        fn play_loop(&self, _sound: &Self::Sound) -> Self::Handle {
            StaticHandle(())
        }
        fn load_sound(&self, _bytes: &'static [u8]) -> Self::Sound {
            StaticSound(())
        }
    }
}

#[cfg(all(feature = "general_audio_native", not(feature = "general_audio_web")))]
mod implementation {
    use super::*;
    use backend::{NativeAudioPlayer, NativeHandle, NativeSound};
    pub use general_audio_native as backend;
    pub struct StaticAudioPlayer(NativeAudioPlayer);
    pub struct StaticHandle(NativeHandle);
    pub struct StaticSound(NativeSound);

    impl StaticHandle {
        pub fn set_volume(&self, volume: f32) {
            self.0.set_volume(volume)
        }
        pub fn volume(&self) -> f32 {
            self.0.volume()
        }
        pub fn pause(&self) {
            self.0.pause()
        }
        pub fn play(&self) {
            self.0.play()
        }
        pub fn background(self) {
            self.0.background()
        }
    }

    impl AudioHandle for StaticHandle {
        fn set_volume(&self, volume: f32) {
            self.0.set_volume(volume)
        }
        fn volume(&self) -> f32 {
            self.0.volume()
        }
        fn pause(&self) {
            self.0.pause()
        }
        fn play(&self) {
            self.0.play()
        }
        fn background(self) {
            self.0.background()
        }
    }

    impl StaticAudioPlayer {
        pub fn new(inner: NativeAudioPlayer) -> Self {
            Self(inner)
        }
        pub fn play(&self, sound: &StaticSound) -> StaticHandle {
            StaticHandle(self.0.play(&sound.0))
        }
        pub fn play_loop(&self, sound: &StaticSound) -> StaticHandle {
            StaticHandle(self.0.play_loop(&sound.0))
        }
        pub fn load_sound(&self, bytes: &'static [u8]) -> StaticSound {
            StaticSound(self.0.load_sound(bytes))
        }
    }

    impl AudioPlayer for StaticAudioPlayer {
        type Sound = StaticSound;
        type Handle = StaticHandle;
        fn play(&self, sound: &Self::Sound) -> Self::Handle {
            StaticHandle(self.0.play(&sound.0))
        }
        fn play_loop(&self, sound: &Self::Sound) -> Self::Handle {
            StaticHandle(self.0.play_loop(&sound.0))
        }
        fn load_sound(&self, bytes: &'static [u8]) -> Self::Sound {
            StaticSound(self.0.load_sound(bytes))
        }
    }
}

#[cfg(all(feature = "general_audio_web", not(feature = "general_audio_native")))]
mod implementation {
    use super::*;
    use backend::{WebAudioPlayer, WebHandle, WebSound};
    pub use general_audio_web as backend;
    pub struct StaticAudioPlayer(WebAudioPlayer);
    pub struct StaticHandle(WebHandle);
    pub struct StaticSound(WebSound);

    impl StaticHandle {
        pub fn set_volume(&self, volume: f32) {
            self.0.set_volume(volume)
        }
        pub fn volume(&self) -> f32 {
            self.0.volume()
        }
        pub fn pause(&self) {
            self.0.pause()
        }
        pub fn play(&self) {
            self.0.play()
        }
        pub fn background(self) {
            self.0.background()
        }
    }

    impl AudioHandle for StaticHandle {
        fn set_volume(&self, volume: f32) {
            self.0.set_volume(volume)
        }
        fn volume(&self) -> f32 {
            self.0.volume()
        }
        fn pause(&self) {
            self.0.pause()
        }
        fn play(&self) {
            self.0.play()
        }
        fn background(self) {
            self.0.background()
        }
    }

    impl StaticAudioPlayer {
        pub fn new(inner: WebAudioPlayer) -> Self {
            Self(inner)
        }
        pub fn play(&self, sound: &StaticSound) -> StaticHandle {
            StaticHandle(AudioPlayer::play(&self.0, &sound.0))
        }
        pub fn play_loop(&self, sound: &StaticSound) -> StaticHandle {
            StaticHandle(self.0.play_loop(&sound.0))
        }
        pub fn load_sound(&self, bytes: &'static [u8]) -> StaticSound {
            StaticSound(self.0.load_sound(bytes))
        }
    }

    impl AudioPlayer for StaticAudioPlayer {
        type Sound = StaticSound;
        type Handle = StaticHandle;
        fn play(&self, sound: &Self::Sound) -> Self::Handle {
            StaticHandle(AudioPlayer::play(&self.0, &sound.0))
        }
        fn play_loop(&self, sound: &Self::Sound) -> Self::Handle {
            StaticHandle(self.0.play_loop(&sound.0))
        }
        fn load_sound(&self, bytes: &'static [u8]) -> Self::Sound {
            StaticSound(self.0.load_sound(bytes))
        }
    }
}

pub use implementation::*;
