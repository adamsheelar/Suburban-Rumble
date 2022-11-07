use bevy::{
	prelude::*,
	window::PresentMode,
};

mod fight;
mod conversation;
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Credits,
    Conversation,
    Fight,
}
#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);
#[derive(Component, Deref, DerefMut)]
struct DespawnTimer(Timer);
pub struct ConvInputEvent(String);
pub struct ConvLossEvent();
pub struct ConvWinEvent();

pub struct CollideEvent(bool,String);


fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("Suburban Rumble"),
			width: WIN_W,
			height: WIN_H,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.insert_resource(ClearColor(Color::BLACK))
		.add_state(GameState::Conversation)	//start the game in the fight state
		.add_event::<ConvInputEvent>()
		.add_event::<ConvLossEvent>()
		.add_event::<ConvWinEvent>()
		.add_event::<CollideEvent>()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system_set(
			SystemSet::on_update(GameState::Credits)
				.label("credits")
				.with_system(show_popup)
				.with_system(remove_popup)
		)
		.add_system_set(
			SystemSet::on_enter(GameState::Credits)
				.with_system(setup_credits)
		)
		.add_system_set(
			SystemSet::on_exit(GameState::Credits)
				.with_system(clear_credits)	// remove the popups on screen when exiting the credit state
		)
		.add_system_set(
			SystemSet::on_update(GameState::Fight)
				.label("fight")
				.with_system(fight::move_player)
				.with_system(fight::attack)
				.with_system(fight::remove_popup)
				.with_system(fight::move_enemy)
				.with_system(fight::collision_handle)
		)
		.add_system_set(
			SystemSet::on_enter(GameState::Fight)
				.with_system(fight::setup_fight)
		)
		.add_system_set(
			SystemSet::on_exit(GameState::Fight)
				.with_system(fight::clear_fight)
		)
		.add_system_set(
			SystemSet::on_enter(GameState::Conversation)
				.with_system(conversation::setup_conversation)
		)
		.add_system_set(
			SystemSet::on_exit(GameState::Conversation)
				.with_system(conversation::clear_conversation)	// remove the popups on screen when exiting the credit state
		)
		.add_system_set(
			SystemSet::on_update(GameState::Conversation)
				.label("conversation")
				.with_system(conversation::text_input)
				.with_system(conversation::handle_player_response)
			    .with_system(conversation::process_input)
		)
		.add_system(change_gamestate)
		.add_system(conv_over)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(Camera2dBundle::default());
	commands.spawn_bundle(TextBundle::from_section(
		"Press \"1\" at any time to start over.",
		TextStyle {
			font: asset_server.load("fonts/SourceSansPro-Regular.ttf"),
			font_size: 25.0,
			color: Color::WHITE,
		}
	));
}

fn setup_credits(mut clear_color: ResMut<ClearColor>, mut commands: Commands, asset_server: Res<AssetServer>) {
	//commands.spawn_bundle(Camera2dBundle::default());
	//commands
	//	.spawn_bundle(SpriteBundle {
	//		texture: asset_server.load("hello_world_win.png"),
	//		..default()
	//	});
	clear_color.0 = Color::BLACK;

	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Makayla_Miles.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(0.,false)))
		.insert(DespawnTimer(Timer::from_seconds(3.,false)));
	
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("adamsheelar.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(3., false)))
		.insert(DespawnTimer(Timer::from_seconds(6.,false)));

	
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("colinferlan.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(6., false)))
		.insert(DespawnTimer(Timer::from_seconds(9.,false)));
	
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("BoazJoseph.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(9., false)))
		.insert(DespawnTimer(Timer::from_seconds(12.,false)));
	
	
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("AlexChlpka.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(12., false)))
		.insert(DespawnTimer(Timer::from_seconds(15.,false)));

	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Birizibe Gnassingbe.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(15., false)))
		.insert(DespawnTimer(Timer::from_seconds(18.,false)));

	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("emilykyle.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(18., false)))
		.insert(DespawnTimer(Timer::from_seconds(21.,false)));

	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("VibhuCreditsF.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(21., false)))
		.insert(DespawnTimer(Timer::from_seconds(24.,false)));		
	info!("GameState: Credits");
}

fn show_popup(
	time: Res<Time>,
	mut popup: Query<(&mut PopupTimer, &mut Transform)>
) {
	for (mut timer, mut transform) in popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			transform.translation.z = 2.;		
		}
	}
}

fn remove_popup(
	time: Res<Time>,
	mut rmpopup: Query<(&mut DespawnTimer, &mut Visibility)>
) {
	for (mut timer, mut vis_map) in rmpopup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			vis_map.is_visible = false;
		}
	}
}

fn clear_credits(
	mut popup: Query<&mut Visibility, With<PopupTimer>>
) {
	for mut vis_map in popup.iter_mut() {
		vis_map.is_visible = false;
	}
}

// Has an event listener for a conversation 'loss' that sends the player to the fight state
fn conv_over(
	mut game_state: ResMut<State<GameState>>,
	mut loss_reader: EventReader<ConvLossEvent>,
	mut win_reader: EventReader<ConvWinEvent>
) {
	for _ev in loss_reader.iter() {
		match game_state.set(GameState::Fight){
			Ok(_) => info!("GameState: Fight"),
			Err(_) => (),
		}
	}
	for _ev in win_reader.iter() {
		match game_state.set(GameState::Credits){
			Ok(_) => info!("GameState: Credits"),
			Err(_) => (),
		}
	}
}

// changes the current gamestate on keypress
fn change_gamestate(
	keys: Res<Input<KeyCode>>,
	mut game_state: ResMut<State<GameState>>,

) {
	if keys.pressed(KeyCode::Key1) {	// change GameState to Conversation
		match game_state.set(GameState::Conversation) {
			Ok(_) => info!("GameState: Conversation"),
			Err(_) => (),
		}
	}
}