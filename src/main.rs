extern crate rspotify;
extern crate lastfm;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::SpotifyClientCredentials;
use lastfm::{Lastfm, Track};

fn main() {
    dotenv::dotenv().ok();

    // Spotify setup
    let spotify_client_id = "your_spotify_client_id";
    let spotify_client_secret = "your_spotify_client_secret";

    let client_credential = SpotifyClientCredentials::default()
        .client_id(spotify_client_id)
        .client_secret(spotify_client_secret)
        .build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    // Last.fm setup
    let lastfm_api_key = "your_lastfm_api_key";
    let lastfm_api_secret = "your_lastfm_api_secret";
    let lastfm_username = "your_lastfm_username";
    let lastfm_password = "your_lastfm_password";

    let lastfm = Lastfm::new(lastfm_api_key, lastfm_api_secret)
        .username(lastfm_username)
        .password(lastfm_password)
        .auth();
}
