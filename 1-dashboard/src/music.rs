use bevy::{
    color::palettes::tailwind,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite::Anchor,
    sprite_render::{Material2d, Material2dPlugin},
};

struct Song {
    title: &'static str,
    artist: &'static str,
    duration: f32,
    album: &'static str,
    artwork: &'static str,
}

impl Song {
    const fn new(
        title: &'static str,
        artist: &'static str,
        duration: f32,
        album: &'static str,
        artwork: &'static str,
    ) -> Self {
        Song {
            title,
            artist,
            duration,
            album,
            artwork,
        }
    }
}

const SONG_LIST: [Song; 10] = [
    Song::new(
        "Across the Universe",
        "The Beatles",
        228.0,
        "Let It Be",
        "let-it-be.png",
    ),
    Song::new(
        "Tomorrow Never Knows",
        "The Beatles",
        179.0,
        "Revolver",
        "revolver.png",
    ),
    Song::new(
        "A Day In The Life",
        "The Beatles",
        179.0,
        "Sgt. Pepper's Lonely Hearts Club Band",
        "sgt-peppers-lonely-hearts-club-band.png",
    ),
    Song::new(
        "Something",
        "The Beatles",
        182.0,
        "Abbey Road",
        "abbey-road.png",
    ),
    Song::new(
        "Blackbird",
        "The Beatles",
        138.0,
        "White Album",
        "white-album.png",
    ),
    Song::new(
        "All My Loving",
        "The Beatles",
        127.0,
        "With The Beatles",
        "with-the-beatles.png",
    ),
    Song::new("Yesterday", "The Beatles", 125.0, "Help!", "help.png"),
    Song::new(
        "In My Life",
        "The Beatles",
        146.0,
        "Rubber Soul",
        "rubber-soul.png",
    ),
    Song::new(
        "I Am The Walrus",
        "The Beatles",
        275.0,
        "Magical Mystery Tour",
        "magical-mystery-tour.png",
    ),
    Song::new(
        "I Saw Her Standing There",
        "The Beatles",
        173.0,
        "Please Please Me",
        "please-please-me.png",
    ),
];

pub fn music_plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<ProgressMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (update, display))
        .insert_resource(MusicState {
            current_song: 1,
            progress: 0.0,
        });
}

#[derive(Asset, TypePath, AsBindGroup, ShaderType, Clone)]
#[uniform(0, ProgressMaterial)]
struct ProgressMaterial {
    progress: f32,
}

impl<'a> From<&'a ProgressMaterial> for ProgressMaterial {
    fn from(material: &'a ProgressMaterial) -> Self {
        material.clone()
    }
}

impl Material2d for ProgressMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/music_progress.wgsl".into()
    }
}

#[derive(Resource)]
struct MusicState {
    current_song: usize,
    progress: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ProgressMaterial>>,
) {
    commands.spawn((
        Transform::from_xyz(300.0, -50.0, 0.0),
        Visibility::Visible,
        children![
            (
                Sprite::from_image(
                    asset_server.load("radio/sgt-peppers-lonely-hearts-club-band.png")
                ),
                Transform::from_xyz(0.0, 100.0, 0.0).with_scale(Vec3::splat(0.5)),
                Artwork
            ),
            (
                Transform::from_xyz(-135.0, -50.0, 0.0),
                Visibility::Visible,
                SongInformation,
                children![
                    (
                        Text2d::new(""),
                        Transform::from_xyz(0.0, 0.0, 0.0),
                        Anchor::CENTER_LEFT,
                    ),
                    (
                        Text2d::new(""),
                        TextColor(tailwind::GRAY_500.into()),
                        Transform::from_xyz(0.0, -25.0, 0.0),
                        Anchor::CENTER_LEFT,
                    ),
                ]
            ),
            (
                Transform::from_xyz(0.0, -100.0, 0.0).with_scale(Vec3::new(270.0, 10.0, 0.0)),
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(ProgressMaterial { progress: 0.0 })),
                ProgressIndicator,
            ),
        ],
    ));
}

#[derive(Component)]
struct ProgressIndicator;

#[derive(Component)]
struct SongInformation;

#[derive(Component)]
struct Artwork;

fn update(mut current: ResMut<MusicState>, time: Res<Time>) {
    current.progress += time.delta_secs() * 5.0;
    if current.progress > SONG_LIST[current.current_song].duration {
        current.current_song = (current.current_song + 1) % SONG_LIST.len();
        current.progress = 0.0;
    }
}

#[allow(clippy::too_many_arguments)]
fn display(
    current: Res<MusicState>,
    material: Single<&MeshMaterial2d<ProgressMaterial>, With<ProgressIndicator>>,
    mut progress_materials: ResMut<Assets<ProgressMaterial>>,
    mut artwork: Single<&mut Sprite, With<Artwork>>,
    song_information: Single<&Children, With<SongInformation>>,
    mut text: Query<&mut Text2d>,
    asset_server: Res<AssetServer>,
    mut last_displayed_song: Local<usize>,
) {
    let song = &SONG_LIST[current.current_song];

    let progress = current.progress / song.duration;
    progress_materials.get_mut(*material).unwrap().progress = progress;

    if *last_displayed_song != current.current_song {
        text.get_mut(song_information[0]).unwrap().0 = song.title.to_string();
        text.get_mut(song_information[1]).unwrap().0 = format!("{} - {}", song.artist, song.album);

        artwork.image = asset_server.load(format!("radio/{}", song.artwork));

        *last_displayed_song = current.current_song;
    }
}
