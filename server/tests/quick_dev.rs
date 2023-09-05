use database::models::{
    action::ActionDto,
    message::MessageDto,
    replay_report::{ReplayReportDto, ReplayReportReporterDto, ReplayReporterPlayerDto},
};
use rand::Rng;
use serde_json::json;
#[tokio::test]
async fn quick_dev() -> Result<(), httpc_test::Error> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    // hc.do_get("/game").await?.print().await?;

    for _ in 0..1000 {
        let game = create_game();
        println!(
            "{:?} {:?}",
            game.players[0].relic_id, game.players[1].relic_id
        );
        hc.do_post("/reports", json!(game)).await?.print().await?;
    }

    Ok(())
}

fn create_game() -> ReplayReportDto {
    let mut rng = rand::thread_rng();

    let player_1 = get_random_player(0);
    let mut player_2 = get_random_player(1);

    while player_1.relic_id == player_2.relic_id {
        player_2 = get_random_player(1);
    }

    ReplayReportDto {
        aborted: false,
        actions: vec![
            ActionDto {
                relic_id: player_1.relic_id as i32,
                tick: 0,
                name: player_1.name.clone(),
                data: vec![3, 1, 233, 3, 0, 0, 16, 1, 113, 60, 5, 4, 255, 0, 0],
            },
            ActionDto {
                relic_id: player_2.relic_id as i32,
                tick: 1,
                name: player_2.name.clone(),
                data: vec![3, 1, 232, 3, 0, 0, 16, 1, 113, 60, 5, 4, 255, 0, 0],
            },
        ],
        dev: false,
        id: format!("{:?}", rng.gen_range(0..999999)),
        map: "2p_calderisrefinery".to_string(),
        reporter: ReplayReportReporterDto {
            date: "1970-01-01T00:00:00Z".to_string(),
            version: "1.0.2".to_string(),
        },
        replay: "".to_string(),
        mod_version: 10320,
        ranked: true,
        league: false,
        frames: 1000,
        ticks: 998,
        messages: vec![
            MessageDto {
                body: "Test Message".to_string(),
                receiver: "All".to_string(),
                sender: player_1.name.clone(),
                tick: 50,
                player_id: 235,
            },
            MessageDto {
                body: "Test Message2".to_string(),
                receiver: "All".to_string(),
                sender: player_2.name.clone(),
                tick: 51,
                player_id: 234,
            },
        ],
        players: vec![player_1, player_2],
        winner: rng.gen_range(0..=1),
    }
}

fn get_random_player(position: usize) -> ReplayReporterPlayerDto {
    let mut rng = rand::thread_rng();
    // pairs of relic and steam ids
    let ids = vec![
        (4321, 123456, "Player 1"),
        (1234, 654321, "Player 2"),
        (9876, 78910, "Player 3"),
        (6789, 91987, "Player 4"),
        (753951, 159357, "Player 5"),
        (456321, 987123, "Player 6"),
    ];

    let index = rng.gen_range(0..ids.len());

    ReplayReporterPlayerDto {
        relic_id: ids[index].0,
        hero: rng.gen_range(1..15),
        race: rng.gen_range(0..=4),
        name: ids[index].2.into(),
        steam_id: ids[index].1,
        team: position,
        sim_id: position,
        slot: position,
    }
}
