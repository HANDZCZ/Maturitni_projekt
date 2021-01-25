pub struct Game {
    props: Props,
    link: ComponentLink<Self>,
    data: GameData,

    get_ft: Option<FetchTask>,
    play_ft: Option<FetchTask>,
    it: IntervalTask,
}
use crate::base::Base;
use crate::notifications::*;
use crate::UserInfo;
use serde::{Deserialize, Serialize};
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchService, FetchTask, Request, Response};
use yew::services::interval::{IntervalService, IntervalTask};

#[derive(Deserialize)]
struct FetchGameData {
    players: Vec<(String, String)>,
    data: Vec<u8>,
    ended: bool,
    winner: Option<String>,
    last_played: String,
    moves_needed: u8,
}

#[derive(Default)]
struct GameData {
    players: Vec<(String, String)>,
    data: Vec<Croft>,
    ended: bool,
    winner: Option<String>,
    last_played: String,
    moves_needed: u8,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct Croft {
    x: u8,
    y: u8,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub game_id: String,
    pub model_callback: Callback<crate::Msg>,
}

#[derive(Clone)]
pub enum PlayState {
    Continue,
    Won(WonGameData),
    Tie,
}

impl PlayState {
    fn won(&self) -> Option<WonGameData> {
        match self {
            PlayState::Won(data) => Some((*data).clone()),
            _ => None,
        }
    }
    fn tie(&self) -> Option<bool> {
        match self {
            PlayState::Tie => Some(true),
            _ => None,
        }
    }
}

#[derive(Serialize, Clone)]
struct WonGameData {
    moves: Vec<Croft>,
    direction: WonDirection,
}

#[derive(Serialize, Clone)]
enum WonDirection {
    Horizontal,
    Vertical,
    DiagonalLR,
    DiagonalRL,
}

pub enum Msg {
    GetGame(bool),
    GotGame(String, bool),
    FailedGetGame(bool),
    Play { data: Croft, state: PlayState },
    Played { data: Croft, state: PlayState },
    FailedPlay,
    Clicked(u8, u8),
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetGame(true));
        let interval = IntervalService::spawn(
            std::time::Duration::new(3, 0),
            link.callback(|_| Msg::GetGame(false)),
        );
        Self {
            props,
            link,
            data: GameData::default(),
            play_ft: None,
            get_ft: None,
            it: interval,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Play { data, state } => {
                if let None = self.play_ft {
                    notification(
                        "Odesílám tah".to_owned(),
                        Position::BottomLeft,
                        Status::Primary,
                        None,
                    );
                    #[derive(Serialize)]
                    struct PlayGameData {
                        game_id: String,
                        #[serde(flatten)]
                        croft: Croft,
                        won: Option<WonGameData>,
                        tie: Option<bool>,
                    }
                    let send_data = PlayGameData {
                        game_id: self.props.game_id.clone(),
                        croft: data.clone(),
                        won: state.won(),
                        tie: state.tie(),
                    };
                    let req = Request::post(format!("{}/game/play", crate::DOMAIN))
                        .header("Content-Type", "application/json")
                        .body(Ok(serde_json::to_string(&send_data).unwrap()))
                        .unwrap();
                    let options = FetchOptions {
                        credentials: Some(yew::web_sys::RequestCredentials::Include),
                        ..FetchOptions::default()
                    };
                    let model_callback = self.props.model_callback.clone();
                    self.play_ft = Some(
                        FetchService::fetch_with_options(
                            req,
                            options,
                            self.link
                                .callback(move |response: Response<Result<String, _>>| {
                                    let (meta, body) = response.into_parts();
                                    let status = meta.status.as_u16();
                                    match status {
                                        200 => Msg::Played {
                                            data: data.clone(),
                                            state: state.clone(),
                                        },
                                        400 | 401 => {
                                            if let Ok(body) = body {
                                                notification(
                                                    body.clone(),
                                                    Position::BottomLeft,
                                                    Status::Danger,
                                                    None,
                                                );
                                                if status == 401
                                                    && body == "User not logged in".to_owned()
                                                {
                                                    model_callback.emit(crate::Msg::LoggedOut);
                                                }
                                            } else {
                                                notification(
                                                    "Server neposlal žádnou chybovou hlášku."
                                                        .to_owned(),
                                                    Position::BottomLeft,
                                                    Status::Warning,
                                                    None,
                                                );
                                            }
                                            Msg::FailedPlay
                                        }
                                        500 => {
                                            notification(
                                                "Nastala chyba serveru".to_owned(),
                                                Position::BottomLeft,
                                                Status::Warning,
                                                None,
                                            );
                                            Msg::FailedPlay
                                        }
                                        _ => {
                                            notification(
                                                "Nastala neimplementovaná chyba".to_owned(),
                                                Position::BottomLeft,
                                                Status::Warning,
                                                None,
                                            );
                                            Msg::FailedPlay
                                        }
                                    }
                                }),
                        )
                        .unwrap(),
                    );
                } else {
                    notification(
                        "Již se odesílá tah".to_owned(),
                        Position::BottomLeft,
                        Status::Warning,
                        None,
                    );
                }
                false
            }
            Msg::Played { data, state } => {
                self.play_ft = None;
                self.data.data.push(data);
                self.data.last_played = self.props.user_info.clone().unwrap().uuid;
                match state {
                    PlayState::Won(_) | PlayState::Tie => {
                        self.data.ended = true;
                    }
                    _ => {}
                }
                notification(
                    "Tah odehrán".to_owned(),
                    Position::BottomLeft,
                    Status::Success,
                    None,
                );
                true
            }
            Msg::FailedPlay => {
                self.play_ft = None;
                notification(
                    "Odehrání tahu selhalo".to_owned(),
                    Position::BottomLeft,
                    Status::Danger,
                    None,
                );
                false
            }
            Msg::GetGame(manual) => {
                if let None = self.get_ft {
                    if manual {
                        notification(
                            "Načítám hru".to_owned(),
                            Position::BottomLeft,
                            Status::Primary,
                            None,
                        );
                    }
                    let req =
                        Request::get(format!("{}/game/get/{}", crate::DOMAIN, self.props.game_id))
                            .body(Nothing)
                            .unwrap();
                    let options = FetchOptions {
                        credentials: Some(yew::web_sys::RequestCredentials::Include),
                        ..FetchOptions::default()
                    };
                    self.get_ft = Some(
                        FetchService::fetch_with_options(
                            req,
                            options,
                            self.link
                                .callback(move |response: Response<Result<String, _>>| {
                                    let (meta, body) = response.into_parts();
                                    match meta.status.as_u16() {
                                        200 => {
                                            if let Ok(body) = body {
                                                Msg::GotGame(body, manual)
                                            } else {
                                                notification(
                                                    "Server neposlal žádnou odpověď".to_owned(),
                                                    Position::BottomLeft,
                                                    Status::Warning,
                                                    None,
                                                );
                                                Msg::FailedGetGame(manual)
                                            }
                                        }
                                        400 => {
                                            if let Ok(body) = body {
                                                notification(
                                                    body,
                                                    Position::BottomLeft,
                                                    Status::Danger,
                                                    None,
                                                );
                                            } else if manual {
                                                notification(
                                                    "Server neposlal žádnou chybovou hlášku."
                                                        .to_owned(),
                                                    Position::BottomLeft,
                                                    Status::Warning,
                                                    None,
                                                );
                                            }
                                            Msg::FailedGetGame(manual)
                                        }
                                        500 => {
                                            if manual {
                                                notification(
                                                    "Nastala chyba serveru".to_owned(),
                                                    Position::BottomLeft,
                                                    Status::Warning,
                                                    None,
                                                );
                                            }
                                            Msg::FailedGetGame(manual)
                                        }
                                        _ => {
                                            if manual {
                                                notification(
                                                    "Nastala neimplementovaná chyba".to_owned(),
                                                    Position::BottomLeft,
                                                    Status::Warning,
                                                    None,
                                                );
                                            }
                                            Msg::FailedGetGame(manual)
                                        }
                                    }
                                }),
                        )
                        .unwrap(),
                    );
                }
                false
            }
            Msg::GotGame(data, manual) => {
                self.get_ft = None;
                let fetch_data: FetchGameData = serde_json::from_str(&data).unwrap();
                let game_data = GameData {
                    ended: fetch_data.ended,
                    winner: fetch_data.winner,
                    players: fetch_data.players,
                    last_played: fetch_data.last_played,
                    moves_needed: fetch_data.moves_needed,
                    data: bincode::deserialize(fetch_data.data.as_slice()).unwrap(),
                };
                let changed = self.data.data != game_data.data
                    || self.data.players.is_empty()
                    || self.data.winner != game_data.winner;
                if changed {
                    self.data = game_data;
                }
                if manual {
                    notification(
                        "Hra načtena".to_owned(),
                        Position::BottomLeft,
                        Status::Success,
                        None,
                    );
                }
                changed
            }
            Msg::FailedGetGame(manual) => {
                self.get_ft = None;
                if manual {
                    notification(
                        "Načítání hry selhalo".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                }
                false
            }
            Msg::Clicked(x, y) => {
                let on_move = {
                    let index = self
                        .data
                        .players
                        .iter()
                        .position(|(_name, id)| *id == self.data.last_played)
                        .unwrap_or_default()
                        + 1;
                    if index == self.data.players.len() {
                        0
                    } else {
                        index
                    }
                };

                if let Some(user_info) = &self.props.user_info {
                    if self.data.players[on_move].1 == user_info.uuid {
                        let mov = Croft { x, y };
                        let temp = vec![mov.clone()];
                        let moves = self
                            .data
                            .data
                            .iter()
                            .chain(temp.iter())
                            .rev()
                            .step_by(self.data.players.len())
                            .collect::<Vec<_>>();

                        fn check_moves(moves: Vec<&Croft>, needed: u8) -> PlayState {
                            use std::cmp;

                            if moves.len() == 30 * 30 {
                                return PlayState::Tie;
                            }

                            let Croft { x, y } = moves.first().unwrap();

                            let max_right = cmp::min(needed - 1, 29 - x);
                            let max_left = cmp::min(needed - 1, *x);
                            {
                                //Horizontal
                                let right = (1..=max_right)
                                    .take_while(|o| moves.contains(&&Croft { y: *y, x: x + o }))
                                    .count();

                                let left = (1..=max_left)
                                    .take_while(|o| moves.contains(&&Croft { y: *y, x: x - o }))
                                    .count();

                                log::info!("r{},l{},mr{},ml{}", right, left, max_right, max_left);
                                if (left + right + 1) >= needed as usize {
                                    return PlayState::Won(WonGameData {
                                        direction: WonDirection::Horizontal,
                                        moves: (0..=right)
                                            .map(|o| Croft {
                                                y: *y,
                                                x: x + o as u8,
                                            })
                                            .chain((1..=left).map(|o| Croft {
                                                y: *y,
                                                x: x - o as u8,
                                            }))
                                            .take(needed as usize)
                                            .collect(),
                                    });
                                }
                            }

                            let max_up = cmp::min(needed - 1, *y);
                            let max_down = cmp::min(needed - 1, 29 - y);
                            {
                                //Vertical
                                let up = (1..=max_up)
                                    .take_while(|o| moves.contains(&&Croft { x: *x, y: y - o }))
                                    .count();

                                let down = (1..=max_down)
                                    .take_while(|o| moves.contains(&&Croft { x: *x, y: y + o }))
                                    .count();

                                if (down + up + 1) >= needed as usize {
                                    return PlayState::Won(WonGameData {
                                        direction: WonDirection::Vertical,
                                        moves: (1..=up)
                                            .map(|o| Croft {
                                                x: *x,
                                                y: y - o as u8,
                                            })
                                            .chain((1..=down).map(|o| Croft {
                                                x: *x,
                                                y: y + o as u8,
                                            }))
                                            .take(needed as usize)
                                            .collect(),
                                    });
                                }
                            }

                            {
                                //DiagonalLR
                                let up_left = (1..=cmp::min(max_up, max_left))
                                    .take_while(|o| moves.contains(&&Croft { x: x - o, y: y - o }))
                                    .count();

                                let down_right = (1..=cmp::min(max_down, max_right))
                                    .take_while(|o| moves.contains(&&Croft { x: x + o, y: y + o }))
                                    .count();

                                if (down_right + up_left + 1) >= needed as usize {
                                    return PlayState::Won(WonGameData {
                                        direction: WonDirection::DiagonalLR,
                                        moves: (1..=up_left)
                                            .map(|o| Croft {
                                                x: x - o as u8,
                                                y: y - o as u8,
                                            })
                                            .chain((1..=down_right).map(|o| Croft {
                                                x: x + o as u8,
                                                y: y + o as u8,
                                            }))
                                            .take(needed as usize)
                                            .collect(),
                                    });
                                }
                            }

                            {
                                //DiagonalRL
                                let up_right = (1..=cmp::min(max_up, max_right))
                                    .take_while(|o| moves.contains(&&Croft { x: x + o, y: y - o }))
                                    .count();

                                let down_left = (1..=cmp::min(max_down, max_left))
                                    .take_while(|o| moves.contains(&&Croft { x: x - o, y: y + o }))
                                    .count();

                                if (down_left + up_right + 1) >= needed as usize {
                                    return PlayState::Won(WonGameData {
                                        direction: WonDirection::DiagonalRL,
                                        moves: (1..=up_right)
                                            .map(|o| Croft {
                                                x: x + o as u8,
                                                y: y - o as u8,
                                            })
                                            .chain((1..=down_left).map(|o| Croft {
                                                x: x - o as u8,
                                                y: y + o as u8,
                                            }))
                                            .take(needed as usize)
                                            .collect(),
                                    });
                                }
                            }

                            PlayState::Continue
                        }

                        self.link.send_message(Msg::Play {
                            data: mov,
                            state: check_moves(moves, self.data.moves_needed),
                        });
                    } else {
                        notification(
                            "Nejsi na tahu".to_owned(),
                            Position::BottomLeft,
                            Status::Danger,
                            None,
                        );
                    }
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed =
            self.props.user_info != props.user_info || self.props.game_id != props.game_id;
        if changed {
            self.props = props;
        }
        changed
    }

    fn view(&self) -> Html {
        let symbols_html: [Html; 4] = [
            html! { <span style="color: red;font-weight: bold">{ "X " }</span> },
            html! { <span style="color: green;font-weight: bold">{ "Y " }</span> },
            html! { <span style="color: blue;font-weight: bold">{ "Z " }</span> },
            html! { <span style="color: yellow;font-weight: bold">{ "F " }</span> },
        ];

        let on_move_html: Html =
            html! { <span style="color: orange;font-weight: bold;">{ " #Na tahu#" }</span> };

        let on_move = {
            let index = self
                .data
                .players
                .iter()
                .position(|(_name, id)| *id == self.data.last_played)
                .unwrap_or_default()
                + 1;
            if index == self.data.players.len() {
                0
            } else {
                index
            }
        };
        let players = self
            .data
            .players
            .iter()
            .enumerate()
            .map(|(i, (name, _id))| {
                let on_move = if i == on_move && !self.data.ended {
                    on_move_html.clone()
                } else {
                    html! { { "" } }
                };
                html! { <p>{ symbols_html[i].clone() }{ name }{ on_move }</p> }
            });

        let game = {
            let players = self.data.players.iter().rev().collect::<Vec<_>>();
            let last_played_index = players
                .iter()
                .position(|(_name, id)| self.data.last_played == *id)
                .unwrap_or_default();
            let data = self
                .data
                .data
                .iter()
                .rev()
                .enumerate()
                .map(|(i, croft)| {
                    let index = i % players.len();
                    (index, croft)
                })
                .collect::<Vec<_>>();
            let symbols_html = symbols_html
                .iter()
                .take(players.len())
                .rev()
                .skip(last_played_index)
                .chain(
                    symbols_html
                        .iter()
                        .skip(players.len() - last_played_index)
                        .take(last_played_index)
                        .rev(),
                )
                .collect::<Vec<_>>();
            (0..30)
                .map(|y| {
                    let row_data = (0..30).map(|x| {
                        data.iter()
                            .find(|(_i, croft)| croft.x == x && croft.y == y)
                            .map(|(i, _croft)| html! { <td>{ symbols_html[*i].clone() }</td> })
                            .unwrap_or({
                                if !self.data.ended
                                    && if let Some(user_info) = &self.props.user_info {
                                        self.data
                                            .players
                                            .iter()
                                            .any(|(_name, id)| *id == user_info.uuid)
                                    } else {
                                        false
                                    }
                                {
                                    let callback = self.link.callback(move |_| Msg::Clicked(x, y));
                                    html! { <td onclick=callback>{ " " }</td> }
                                } else {
                                    html! { <td>{ " " }</td> }
                                }
                            })
                    });
                    html! { <tr>{ for row_data }</tr> }
                })
                .collect::<Vec<_>>()
        };

        let status = if !self.data.ended {
            "Probíhá".to_owned()
        } else if let Some(winner) = &self.data.winner {
            format!(
                "{} je výherce",
                self.data
                    .players
                    .iter()
                    .find(|(_name, id)| id == winner)
                    .unwrap()
                    .0
            )
        } else {
            "Remíza".to_owned()
        };
        html! {
        <Base user_info=&self.props.user_info active_nav=None background_image="andre-benz-JBkwaYMuhdc-unsplash.jpg" model_callback=self.props.model_callback.clone()>
        <div class="uk-section-secondary uk-margin-medium uk-margin-medium-left uk-margin-medium-right uk-padding">
            <div class="uk-grid-divider uk-grid">
                <div class="uk-width-auto">
                    <style>{ r#"
                        table tr td {
                            font-size: 27px;
                            width: 40px;
                            text-align: center;
                            text-transform: uppercase;
                            border: 1px solid white;
                            height: 40px;
                        }

                        table {
                            margin: 0;
                            padding: 0;
                            border-collapse: collapse;
                        }

                        body > div {
                            padding-bottom: 1px;
                        }"# }
                    </style>
                    <table>
                        { for game }
                    </table>
                </div>
                <div class="uk-width-expand">
                    <article class="uk-article">
                        <h1 class="uk-article-title uk-nav-center uk-margin-medium-bottom">{ "Hráči" }</h1>
                        { for players }
                        <hr/>
                        <p>{ "Tahů k vítězství: " }{ self.data.moves_needed }</p>
                        <p>{ "Status: " }{ status }</p>
                    </article>
                </div>
            </div>
        </div>
        </Base>
        }
    }
}
