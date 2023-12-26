// music struct
struct Music {
    title: String,
    artist: String,
    album: String,
    duration: u32, // in seconds
    liked: bool,
}

// playlist struct
struct Playlist {
    name: String,
    songs: Vec<Music>,
    current: usize, // index of the current song
}

impl Playlist {
    // create a new playlist with a name
    fn new(name: String) -> Self {
        Self {
            name,
            songs: Vec::new(),
            current: 0,
        }
    }

    // add a song to the playlist
    fn add_song(&mut self, song: Music) {
        // check if the song already exists in the playlist
        if self.songs.iter().any(|s| s.title == song.title) {
            println!("This song already exists in the playlist.");
        } else {
            self.songs.push(song);
        }
    }

    // remove a song from the playlist by title
    fn remove_song(&mut self, title: &str) {
        // find the index of the song to remove
        if let Some(index) = self.songs.iter().position(|s| s.title == title) {
            self.songs.remove(index);
            println!("{} removed from the playlist.", title);
        } else {
            println!("This song does not exist in the playlist.");
        }
    }

    // toggle the like status of the current song
    fn toggle_like(&mut self) {
        // get the current song as a mutable reference
        if let Some(song) = self.songs.get_mut(self.current) {
            song.liked = !song.liked;
            println!(
                "You {} {} song.",
                if song.liked { "liked" } else { "unliked" },
                song.title
            );
        } else {
            println!("There is no song playing.");
        }
    }

    // print the current song
    fn now_playing(&self) -> String {
        // get the current song as an immutable reference
        if let Some(song) = self.songs.get(self.current) {
            format!(
                "Now playing: {} by {} from {} ({:02}:{:02})",
                song.title,
                song.artist,
                song.album,
                song.duration / 60,
                song.duration % 60
            )
        } else {
            format!("There is no song playing.")
        }
    }

    // play the next song in the playlist
    fn next(&mut self) {
        // increment the current index and wrap around if needed
        self.current += 1;
    }

    // play the previous song in the playlist
    fn prev(&mut self) {
        // decrement the current index and wrap around if needed
        self.current -= 1;
    }

    // stop the playlist
    fn stop(&mut self) {
        // reset the current index to zero
        self.current = 0;
        println!("Playlist stopped.");
    }
}

fn main() {
    // create some sample songs
    let song1 = Music {
        title: "Bohemian Rhapsody".to_string(),
        artist: "Queen".to_string(),
        album: "A Night at the Opera".to_string(),
        duration: 355,
        liked: false,
    };
    let song2 = Music {
        title: "Stairway to Heaven".to_string(),
        artist: "Led Zeppelin".to_string(),
        album: "Led Zeppelin IV".to_string(),
        duration: 482,
        liked: false,
    };
    let song3 = Music {
        title: "Hotel California".to_string(),
        artist: "Eagles".to_string(),
        album: "Hotel California".to_string(),
        duration: 390,
        liked: false,
    };

    // create a playlist
    let mut playlist = Playlist::new("Up is Down".to_string());
    println!("Playlist: {} has been created", playlist.name);

    // add songs to the playlist
    playlist.add_song(song1);
    playlist.add_song(song2);
    playlist.add_song(song3);

    // play the playlist
    let playing = playlist.now_playing();
    println!("{}", playing);
    playlist.next();
    let playing = playlist.now_playing();
    println!("{}", playing);
    playlist.toggle_like();
    playlist.prev();
    let playing = playlist.now_playing();
    println!("{}", playing);
    playlist.stop();
    playlist.remove_song("Bohemian Rhapsody");
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    // create a sample song for testing
    fn sample_song() -> Music {
        Music {
            title: "Bohemian Rhapsody".to_string(),
            artist: "Queen".to_string(),
            album: "A Night at the Opera".to_string(),
            duration: 355,
            liked: false,
        }
    }

    // create a sample playlist for testing
    fn sample_playlist() -> Playlist {
        // create some sample songs
        let song1 = Music {
            title: "Bohemian Rhapsody".to_string(),
            artist: "Queen".to_string(),
            album: "A Night at the Opera".to_string(),
            duration: 355,
            liked: false,
        };
        let song2 = Music {
            title: "Stairway to Heaven".to_string(),
            artist: "Led Zeppelin".to_string(),
            album: "Led Zeppelin IV".to_string(),
            duration: 482,
            liked: false,
        };
        let song3 = Music {
            title: "Hotel California".to_string(),
            artist: "Eagles".to_string(),
            album: "Hotel California".to_string(),
            duration: 390,
            liked: false,
        };

        // create a playlist with a name and some songs
        let mut playlist = Playlist::new("My Playlist".to_string());
        playlist.add_song(song1);
        playlist.add_song(song2);
        playlist.add_song(song3);

        playlist
    }

    // test the new method of the Playlist struct
    #[test]
    fn test_new_playlist() {
        // create a new playlist with a name
        let playlist = Playlist::new("My Playlist".to_string());
        // check the name of the playlist
        assert_eq!(playlist.name, "My Playlist");
        // check the songs vector is empty
        assert_eq!(playlist.songs.len(), 0);
        // check the current index is zero
        assert_eq!(playlist.current, 0);
    }

    // test the add_song method of the Playlist struct
    #[test]
    fn test_add_song() {
        // create a mutable playlist
        let mut playlist = Playlist::new("My Playlist".to_string());
        // create a song
        let song = sample_song();
        // add the song to the playlist
        playlist.add_song(song);
        // check the songs vector has one element
        assert_eq!(playlist.songs.len(), 1);
        // check the song in the playlist is the same as the added song
        assert_eq!(playlist.songs[0].title, "Bohemian Rhapsody");
        assert_eq!(playlist.songs[0].artist, "Queen");
        assert_eq!(playlist.songs[0].album, "A Night at the Opera");
        assert_eq!(playlist.songs[0].duration, 355);
        assert_eq!(playlist.songs[0].liked, false);
    }

    // test the remove_song method of the Playlist struct
    #[test]
    fn test_remove_song() {
        // create a mutable playlist
        let mut playlist = sample_playlist();
        // check the initial songs vector length
        assert_eq!(playlist.songs.len(), 3);
        // remove a song by title
        playlist.remove_song("Bohemian Rhapsody");
        // check the updated songs vector length
        assert_eq!(playlist.songs.len(), 2);
        // check the song is removed from the playlist
        assert!(!playlist
            .songs
            .iter()
            .any(|s| s.title == "Bohemian Rhapsody"));
    }

    // test the toggle_like method of the Playlist struct
    #[test]
    fn test_toggle_like_playlist() {
        // create a mutable playlist
        let mut playlist = sample_playlist();
        // check the initial liked status of the current song
        assert_eq!(playlist.songs[playlist.current].liked, false);
        // toggle the like status of the current song
        playlist.toggle_like();
        // check the updated liked status of the current song
        assert_eq!(playlist.songs[playlist.current].liked, true);
    }

    // test the now_playing method of the Playlist struct
    #[test]
    fn test_now_playing() {
        // create a playlist
        let playlist = sample_playlist();
        // get the current song
        let song = &playlist.songs[playlist.current];
        let output = playlist.now_playing();
        // check the output matches the expected format
        assert!(output.contains(&song.title));
    }

    // test the next method of the Playlist struct
    #[test]
    fn test_next() {
        // create a mutable playlist
        let mut playlist = sample_playlist();
        // check the initial current index
        assert_eq!(playlist.current, 0);
        // play the next song in the playlist
        playlist.next();
        // check the updated current index
        assert_eq!(playlist.current, 1);
    }

    // test the prev method of the Playlist struct
    #[test]
    fn test_prev() {
        // create a mutable playlist
        let mut playlist = sample_playlist();
        // set the current index to the last song
        playlist.current = playlist.songs.len() - 1;
        // check the initial current index
        assert_eq!(playlist.current, 2);
        // play the previous song in the playlist
        playlist.prev();
        // check the updated current index
        assert_eq!(playlist.current, 1);
    }

    // test the stop method of the Playlist struct
    #[test]
    fn test_stop() {
        // create a mutable playlist
        let mut playlist = sample_playlist();
        // set the current index to a non-zero value
        playlist.current = 1;
        // check the initial current index
        assert_eq!(playlist.current, 1);
        // stop the playlist
        playlist.stop();
        // check the updated current index
        assert_eq!(playlist.current, 0);
    }
}
