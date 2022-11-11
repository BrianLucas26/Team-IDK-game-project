use bevy::{
	prelude::*,
};
use rand::Rng;

use super::{CombatOptions, CombatStats, Enemy, CombatLog};

const COMBAT_BUTTON: Color = Color::rgb(0.15, 0.15, 0.235);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);


pub fn spawn_combat_buttons(
	commands: &mut Commands,
    asset_server: &Res<AssetServer>,
   	id: CombatOptions,
    left_val: Val,
    top_val: Val,
    text: &str,
	button_size: Size<Val>,
){
	let _button_entity = 
	commands
		.spawn_bundle(ButtonBundle {
            style: Style {
                size: button_size,
				position: UiRect { 
					left: left_val,
					top: top_val, 
					..default()
				},
				position_type: PositionType::Absolute,
				justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: COMBAT_BUTTON.into(),
            ..default()
        })
		.with_children(|parent| {
			parent.spawn_bundle(TextBundle::from_section(
				text,
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 40.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				},
			));
		})
		.insert(id)
		.id();
}

pub fn despawn_button(mut commands: Commands, button_query: Query<Entity, With<CombatOptions>>){
    for button in button_query.iter(){
        commands.entity(button).despawn_recursive();
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
				*color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
				*color = COMBAT_BUTTON.into();
            }
            Interaction::None => {
				*color = COMBAT_BUTTON.into();
            }
        }
    }
}
//Can probably put this with the other button system
//This checks which button was clicked
pub fn combat_button_system2(
    query: Query<(&Interaction, &CombatOptions), (Changed<Interaction>, With<Button>)>,
	mut enemy_query: Query<&mut CombatStats, With<Enemy>>,
	mut player_query: Query<&mut CombatStats, Without<Enemy>>,
    //mut state: ResMut<State<GameState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Clicked{
			let mut log = CombatLog{
				player_damage:0,
				enemy_damage:0,
			};
			let mut player_stats = player_query.single_mut();
			let mut enemy_stats = enemy_query.single_mut();
			let mut valid = false;
            match button{
                CombatOptions::Attack => {
					//Will put into separate functions later
					println!("Attack");
					log.player_damage = if player_stats.double {10} else {5} ;
					valid = true;
					player_stats.double = false;
                }
                CombatOptions::Charge => {
					//Will put into separate functions later
					println!("Charge");
					if player_stats.tp >= 20*player_stats.tp_cost_mult {
						player_stats.tp -= 20*player_stats.tp_cost_mult;
						log.player_damage = if player_stats.double {60} else {30} ;
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!")
					}
                }
				CombatOptions::Recover => {
					println!("Recover");
					player_stats.tp = std::cmp::min(player_stats.tp+20, player_stats.max_tp);
					valid = true;
					player_stats.double = false;
                }
				CombatOptions::Heal => {
					println!("Heal");
					if player_stats.tp >= 10 {
						player_stats.tp -= 10;
						player_stats.health = std::cmp::min(player_stats.max_health, player_stats.health+20);
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!")
					}
                }
				CombatOptions::Guard => {
					println!("Guard");
					if player_stats.tp >= 30 {
						player_stats.tp -= 30;
						player_stats.guard = true;
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!")
					}
                }
				CombatOptions::AntiMage => {
					println!("AntiMage");
					if player_stats.tp >= 5*player_stats.tp_cost_mult {
						player_stats.tp -= 5*player_stats.tp_cost_mult;
						enemy_stats.tp = std::cmp::max(0, enemy_stats.tp-10);
						log.player_damage = if player_stats.double {10} else {5};
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!")
					}
                }
				CombatOptions::Double => {
					println!("Double");
					if player_stats.tp >= 5 {
						player_stats.tp -= 5;
						player_stats.double = true;
						player_stats.tp_cost_mult = 2;
						valid = true;
					} else {
						println!("TP Low!")
					}
                }
				CombatOptions::Block=> {
					println!("Block");
					if player_stats.tp >= 10 {
						player_stats.tp -= 10;
						player_stats.block = true;
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!")
					}
                }
                CombatOptions::Unleash => {
					print!("Unleash");
					match player_stats.token{
						1 => {
							if player_stats.tp <= player_stats.max_tp-10 {
								player_stats.tp += 10;
							} else {
								player_stats.tp = player_stats.max_tp;
							}
							log.player_damage += 10;
							player_stats.double = false;
							player_stats.token = 0;
							player_stats.use_token = true;
							valid = true;
						}

						2 => {
							if player_stats.tp <= player_stats.max_tp-20 {
								player_stats.tp += 20;
							} else {
								player_stats.tp = player_stats.max_tp;
							}
							if player_stats.health <= player_stats.max_health-20 {
								player_stats.health += 20;
							} else {
								player_stats.health = player_stats.max_health;
							}
							player_stats.double = false;
							player_stats.token = 0;
							player_stats.use_token = true;
							valid = true;
						}

						3 => {
							log.player_damage += 30;
							player_stats.double = false;
							player_stats.token = 0;
							player_stats.use_token = true;
							valid = true;
						}

						4 => {
							if player_stats.tp <= player_stats.max_tp-40 {
								player_stats.tp += 30;
							} else {
								player_stats.tp = player_stats.max_tp;
							}
							if player_stats.health <= player_stats.max_health-40 {
								player_stats.health += 40;
							} else {
								player_stats.health = player_stats.max_health;
							}
							player_stats.double = false;
							player_stats.token = 0;
							player_stats.use_token = true;
							valid = true;
						}

						5 => {
							player_stats.health = player_stats.max_health;
							log.player_damage += 50;
							player_stats.double = false;
							player_stats.token = 0;
							player_stats.use_token = true;
							valid = true;
						}

						// this line is to avoid compile error
						_ => println!("No Token!")
					}
				},
            }

			// TODO: Implement Token manipulations
			let mut rng = rand::thread_rng();
			let mut random_num = rng.gen_range(1..9);
			let mut valid_ai_move = false;
			if valid{
				while !valid_ai_move{
					match random_num{
						1 =>{
							println!("Enemy Attacks");
							log.enemy_damage = if enemy_stats.double {10} else {5} ;
							valid_ai_move = true;
							player_stats.double = false;
						}
						2 =>{
							if enemy_stats.tp >= 20*enemy_stats.tp_cost_mult {
								println!("Enemy Charges");
								enemy_stats.tp -= 20*enemy_stats.tp_cost_mult;
								log.enemy_damage = if enemy_stats.double {60} else {30};
								valid_ai_move = true;
								enemy_stats.double = false;
							}else{
								valid_ai_move = false;
							}
						}
						3 =>{
							println!("Enemy Recovers");
							enemy_stats.tp = std::cmp::min(enemy_stats.tp+20, enemy_stats.max_tp);
							valid_ai_move = true;
							enemy_stats.double = false;
						}
						4 =>{
							if enemy_stats.tp >= 10 {
								println!("Enemy Heals");
								enemy_stats.tp -= 10;
								enemy_stats.health = std::cmp::min(enemy_stats.max_health, enemy_stats.health+20);
								valid_ai_move = true;
								enemy_stats.double = false;
							} else {
								valid_ai_move = false;
							}
						}
						5 =>{
							if enemy_stats.tp >= 30 {
								println!("Enemy Guards");
								enemy_stats.tp -= 30;
								enemy_stats.guard = true;
								valid_ai_move = true;
								enemy_stats.double = false;
							} else {
								valid_ai_move = false;
							}
						}
						6 =>{
							if enemy_stats.tp >= 5*enemy_stats.tp_cost_mult {
								println!("Enemy AntiMage");
								enemy_stats.tp -= 5*enemy_stats.tp_cost_mult;
								player_stats.tp = std::cmp::max(0, player_stats.tp-10);
								log.enemy_damage = if enemy_stats.double {10} else {5};
								valid_ai_move = true;
								enemy_stats.double = false;
							} else {
								valid_ai_move = false;
							}
						}
						7 =>{
							if enemy_stats.tp >= 5 {
								println!("Enemy Double");
								enemy_stats.tp -= 5;
								enemy_stats.double = true;
								enemy_stats.tp_cost_mult = 2;
								valid_ai_move = true;
							} else {
								valid_ai_move = false;
							}
						}
						8 =>{
							if enemy_stats.tp >= 10 {
								println!("Enemy Block");
								enemy_stats.tp -= 10;
								enemy_stats.block = true;
								valid_ai_move = true;
								enemy_stats.double = false;
							} else {
								valid_ai_move = false;
							}
						}
						_ =>{
							panic!("Shouldn't happen");
						}
					}
					if !valid_ai_move{
						random_num = rng.gen_range(1..9);
					}
				}	
			}
			
			if valid {
				if log.player_damage > log.enemy_damage {
					if enemy_stats.block { 
						enemy_stats.health -= log.player_damage/2;
					} else if enemy_stats.guard {
						player_stats.health -= log.player_damage*2;
						if !enemy_stats.use_token {
							if enemy_stats.token < enemy_stats.max_token {
								enemy_stats.token += 1;
							}
						}
					} else {
						enemy_stats.health -= log.player_damage - log.enemy_damage;
						if !player_stats.use_token {
							if player_stats.token < player_stats.max_token {
								player_stats.token += 1;
							}
						}
					}
				} else if log.enemy_damage > log.player_damage {
					if player_stats.block { 
						player_stats.health -= log.enemy_damage/2;
					} else if player_stats.guard {
						enemy_stats.health -= log.enemy_damage*2;
						if !player_stats.use_token {
							if player_stats.token < player_stats.max_token {
								player_stats.token += 1;
							}
						}
					} else {
						player_stats.health -= log.enemy_damage - log.player_damage;
						if !enemy_stats.use_token {
							if enemy_stats.token < enemy_stats.max_token {
								enemy_stats.token += 1;
							}
						}
					}
				}
				if player_stats.health <= 0 {
					println!("You Lose!")
				} else if enemy_stats.health <= 0 {
					println!("Victory!")
				}
				player_stats.block = false;
				player_stats.guard = false;
				enemy_stats.block = false;
				enemy_stats.guard = false;
				player_stats.use_token = false;
				enemy_stats.use_token = false;
				println!("Your health is {}", player_stats.health);
				println!("Enemy health is {}", enemy_stats.health);
			}
        }
    }
}