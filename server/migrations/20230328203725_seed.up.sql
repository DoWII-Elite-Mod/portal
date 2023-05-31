CREATE TABLE player (
	relic_id int UNIQUE NOT NULL,
	steam_id int UNIQUE NOT NULL,
	forum_id int UNIQUE,
	discord_id int UNIQUE,
	name text NOT NULL,
	PRIMARY KEY(relic_id)
);

CREATE TABLE map (
	id SERIAL UNIQUE NOT NULL,
	file_name text NOT NULL,
	screen_name text NOT NULL,
	player_count int NOT NULL,
	PRIMARY KEY(id)
);

CREATE TABLE hero (
	id int UNIQUE NOT NULL,
	hero_slot int NOT NULL,
	race_slot int NOT NULL,
	short_name text NOT NULL,
	name text NOT NULL,
	race_name text NOT NULL,
	PRIMARY KEY(id)
);

CREATE TABLE league (
	id SERIAL UNIQUE NOT NULL,
	started_at timestamptz NOT NULL,
	ended_at timestamptz NOT NULL
);

CREATE TABLE match (
	relic_id int UNIQUE NOT NULL,
	map_id int NOT NULL,
	ticks int NOT NULL,
	league_id int,
	reported_at timestamptz NOT NULL DEFAULT NOW(),
	mod_version int NOT NULL,
	PRIMARY KEY(relic_id),
	CONSTRAINT fk_map FOREIGN KEY(map_id) REFERENCES map(id),
	CONSTRAINT fk_league FOREIGN KEY(league_id) REFERENCES league(id)
);

CREATE TABLE player_match (
	player_id int NOT NULL,
	match_id int NOT NULL,
	winner boolean NOT NULL,
	hero_id int NOT NULL,
	team int NOT NULL,
	PRIMARY KEY(player_id, match_id),
	CONSTRAINT fk_player FOREIGN KEY(player_id) REFERENCES player(relic_id),
	CONSTRAINT fk_match FOREIGN KEY(match_id) REFERENCES match(relic_id),
	CONSTRAINT fk_hero FOREIGN KEY(hero_id) REFERENCES hero(id)
);

CREATE TABLE action (
	relic_id int NOT NULL,
	match_relic_id int NOT NULL,
	tick int NOT NULL,
	data integer[],
	CONSTRAINT fk_action_player FOREIGN KEY(relic_id) REFERENCES player(relic_id),
	CONSTRAINT fk_action_match FOREIGN KEY(match_relic_id) REFERENCES match(relic_id)
);

INSERT INTO hero (id,hero_slot,race_slot,short_name,name,race_name) VALUES
	 (1,2,3,'FC','Force Commander','Space Marines'),
	 (2,1,3,'AP','Apothecary','Space Marines'),
	 (3,3,3,'TM','Techmarine','Space Marines'),
	 (4,3,4,'WB','Warboss','Orks'),
	 (5,1,4,'KN','Kommando Nob','Orks'),
	 (6,2,4,'MB','Mekboy','Orks'),
	 (7,2,1,'WL','Warlock','Eldar'),
	 (8,3,1,'WS','Warp Spider Exarch','Eldar'),
	 (9,1,1,'FS','Farseer','Eldar'),
	 (10,3,5,'HT','Hive Tyrant','Tyranids'),
	 (11,2,5,'RA','Ravener Alpha','Tyranids'),
	 (12,1,5,'LA','Lictor Alpha','Tyranids'),
	 (13,1,0,'CL','Chaos Lord','Chaos Space Marines'),
	 (14,2,0,'PC','Plague Champion','Chaos Space Marines'),
	 (15,3,0,'CS','Chaos Sorcerer','Chaos Space Marines'),
	 (16,3,2,'IN','Inquisitor','Imperial Guard'),
	 (17,2,2,'LC','Lord Commissar','Imperial Guard'),
	 (18,1,2,'LG','Lord General','Imperial Guard'),
	 (19,0,3,'BC','Brother-Captain','Ordo Malleus'),
	 (20,4,3,'EA','Eversor Assassin','Ordo Malleus'),
	 (21,5,3,'DH','Daemonhunter','Ordo Malleus');


INSERT INTO map (file_name,screen_name,player_count) VALUES
	 ('2p_ashesoftyphon','Ashes of Typhon',2),
	 ('2p_calderisrefinery','Calderis Refinery',2),
	 ('1v1typhonlowlands','Green Tooth Gorge',2),
	 ('2p_typhonswamp','Green Tooth Jungle',2),
	 ('2p_typhonswampredux','Green Tooth Jungle Redux',2),
	 ('2p_hive_city','Hive City',2),
	 ('2p_icestationobelis','Ice Station Obelis',2),
	 ('2p_judgmentofcarrion','Judgment of Carrion',2),
	 ('2p_legishighstratum','Legis High Stratum',2),
	 ('2p_leviathanhive','Leviathan Hive',2),
	 ('2p_outerreaches','Outer Reaches',2),
	 ('2p_questsdisaster','Quest’s Heresy',2),
	 ('2p_calderisdunes','Siwal Frontier',2),
	 ('2p_bay_of_grots','Bay of Grots',2),
	 ('2p_calderis river','Calderis River',2),
	 ('2p_catachanfort','Catachan Fortress',2),
	 ('2p_fedrid_folly','Fedrid Folly',2),
	 ('2p_highrisks','High Risks',2),
	 ('2p_mirage','Mirage',2),
	 ('2p_plaza_primarus','Plaza Primarus',2),
	 ('2p_sector95','Sector 95',2),
	 ('2p_smallwhitewalls','White Walls',2),
	 ('2p_tallarnravine','Tallarn Ravine',2),
	 ('2p_wrazgotsleadfoot','Wrazgot''s Leadfoot',2),
	 ('2p_paletoothgorge','Pale Tooth Gorge',2),
	 ('jarilosforge','Jarilo''s Forge',2),
	 ('2p_leviathan_eye','Leviathan Eye',2),
	 ('2p_imperial_plaza','Imperial Plaza',2),
	 ('2p_lugganath_glacier','Lugganath Glacier',2),
	 ('2p_typhonswampreduxfixedversion','Green Tooth Jungle Redux - Fixed',2),
	 ('2p_calderis_desert','Calderis Desert',2),
	 ('2p_vulcan_pits','Vulcan Pits',2),
	 ('2p_fortuna_sanctum','Fortuna Sanctum',2),
	 ('2p_seraph_palace','Seraph Palace',2),
	 ('2p_selenon_island','Selenon Island',2),
	 ('2p_ocean_base','Ocean Base',2),
	 ('2p_tiamets_paradise','Tiamets Paradise',2),
	 ('2p_doom_of_tularis','Doom of Tularis',2),
	 ('2p_fen_dunloch','Fen Dunloch',2),
	 ('2p_calderis_jungle','Calderis Jungle',2),
	 ('2p_tiamets_paradise','Tiamets Paradise',2),
	 ('2p_ashesoftyphon_redux','Ashes of Typhon redux',2),
	 ('2p_solstice_plateau','Solstice Plateau',2);
