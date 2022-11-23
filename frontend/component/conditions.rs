use yew::prelude::*;
use ztrix::condition::all_clear::AllClearType;
use ztrix::condition::chain::ChainConditions;
use ztrix::condition::chain::ChainScorer;
use ztrix::condition::event::Conditions;
use ztrix::condition::event::EventConditions;
use ztrix::condition::event::EventScorer;
use ztrix::condition::event::ReqOrMin;
use ztrix::condition::spin::SpinConditions;
use ztrix::condition::spin::SpinScorer;
use ztrix::condition::spin::SpinType;
use ztrix::game::PieceType;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub win_conditions: Conditions,
    pub end_conditions: Conditions,
}

fn zone_name(lines: usize) -> String {
    match lines {
        5 => "Pentrix".to_owned(),
        6 => "Hextrix".to_owned(),
        7 => "Septrix".to_owned(),
        8 => "Octorix".to_owned(),
        9 => "Pendecatrix".to_owned(),
        10 => "Decatrix".to_owned(),
        11 => "Undecatrix".to_owned(),
        12 => "Dodecatrix".to_owned(),
        13 => "Tridecatrix".to_owned(),
        14 => "Quadecatrix".to_owned(),
        15 => "Decapentrix".to_owned(),
        16 => "Decahextrix".to_owned(),
        17 => "Decaseptrix".to_owned(),
        18 => "Perfectrix".to_owned(),
        19 => "Penultimatrix".to_owned(),
        20 => "Ultimatrix".to_owned(),
        21 => "Kirbtrix".to_owned(),
        22 => "Impossitrix".to_owned(),
        23 => "Infinitrix".to_owned(),
        24 => "Electrix".to_owned(),
        25 => "Electrix+".to_owned(),
        26 => "Electrix++".to_owned(),
        l => format!{"{}-Trix", l},
    }
}

fn zone_clear_text(req_lines: ReqOrMin) -> String {
    match req_lines {
        ReqOrMin::Min(l) => match l {
            0 => "Zones".to_owned(),
            l => zone_name(l),
        }
        ReqOrMin::Req(l) => format! {"={}", zone_name(l)}
    }
}

fn line_clear_text(negate: bool, req_lines: ReqOrMin,
        req_piece: Option<PieceType>, req_all_clear: AllClearType,
        req_spin: Option<Option<SpinType>>, req_hard: Option<bool>,
        req_b2b: Option<bool>, req_combo: ReqOrMin) -> String {
    format! {
        "{}{}{}{}{}{}{}{}{}{}{}s",
        match negate {
            true => "Non ",
            false => "",
        },
        match req_combo {
            ReqOrMin::Req(c) => format! {"{}C ", c},
            ReqOrMin::Min(0) => "".to_owned(),
            ReqOrMin::Min(c) => format! {">{}C ", c},
        },
        match req_b2b {
            None => "",
            Some(false) => "Non-B2B ",
            Some(true) => "B2B ",
        },
        match req_hard {
            None => "",
            Some(false) => "Non-Hard ",
            Some(true) => "Hard "
        },
        match req_spin {
            None => "",
            Some(None) => "",
            Some(Some(SpinType::Full)) => "Full ",
            Some(Some(SpinType::Mini)) => "Mini ",
        },
        match req_piece {
            None => "",
            Some(PieceType::I) => "I",
            Some(PieceType::O) => "O",
            Some(PieceType::S) => "S",
            Some(PieceType::Z) => "Z",
            Some(PieceType::J) => "J",
            Some(PieceType::L) => "L",
            Some(PieceType::T) => "T",
        },
        match req_piece {
            None => "",
            Some(_) => match req_spin {
                None => " ",
                Some(_) => "-"
            }
        },
        match req_spin {
            None => "",
            Some(_) => "Spin "
        },
        match req_lines {
            ReqOrMin::Req(0) => "Zero".to_owned(),
            ReqOrMin::Req(1) => "Single".to_owned(),
            ReqOrMin::Req(2) => "Double".to_owned(),
            ReqOrMin::Req(3) => "Triple".to_owned(),
            ReqOrMin::Req(4) => "Quad".to_owned(),
            ReqOrMin::Req(n) => format! {"{}-Clear", n},
            ReqOrMin::Min(0) => "Placement".to_owned(),
            ReqOrMin::Min(1) => match req_all_clear {
                AllClearType::NONE => "Clear".to_owned(),
                _ => "".to_owned(),
            }
            ReqOrMin::Min(2) => "Double+".to_owned(),
            ReqOrMin::Min(3) => "Triple+".to_owned(),
            ReqOrMin::Min(4) => "Quad+".to_owned(),
            ReqOrMin::Min(n) => format! {"{}-Clear+", n},
        },
        match req_all_clear {
            AllClearType::NONE => "",
            _ => match req_lines {
                ReqOrMin::Min(1) => "",
                _ => " ",
            }
        },
        match req_all_clear {
            AllClearType::NONE => "",
            AllClearType::GRAY_CLEAR => "Gray-Clear",
            AllClearType::COLOR_CLEAR => "Color-Clear",
            AllClearType::ALL_CLEAR => "All-Clear",
        }
    }
}

fn chain_scorer_text(scorer: &ChainScorer) -> String {
    match scorer.clone() {
        ChainScorer::LineClear { req_lines, req_piece,
            req_all_clear, req_spin, req_hard,
            req_b2b, req_combo, negate } =>
            line_clear_text(negate, req_lines, req_piece, req_all_clear,
                req_spin, req_hard, req_b2b, req_combo),
        ChainScorer::ZoneClear { req_lines } => zone_clear_text(req_lines),
        ChainScorer::LinesCleared => "Lines".to_owned(),
        ChainScorer::DamageDealt { count_zone_damage } => match count_zone_damage {
            false => "Damage".to_owned(),
            true => "Zone Damage".to_owned(),
        },
        ChainScorer::JeapordyDealt => "Jeapordy".to_owned(),
    }
}

fn spin_scorer_text(scorer: &SpinScorer) -> String
{
    match scorer.clone() {
        SpinScorer::LineClear { req_lines, req_piece,
            req_all_clear, req_spin, req_hard, negate } =>
            line_clear_text(negate, req_lines, req_piece, req_all_clear,
                req_spin, req_hard, None, ReqOrMin::Min(0)),
        SpinScorer::ZoneClear { req_lines } => zone_clear_text(req_lines),
        SpinScorer::LinesCleared => "Lines".to_owned(),
    }
}

fn event_scorer_text(scorer: &EventScorer) -> String {
    match scorer.clone() {
        EventScorer::LineClear { req_lines, req_piece,
            req_all_clear, negate } =>
            line_clear_text(negate, req_lines, req_piece, req_all_clear,
                None, None,
                None, ReqOrMin::Min(0)),
        EventScorer::ZoneClear { req_lines } => zone_clear_text(req_lines),
        EventScorer::LinesCleared => "Lines".to_owned(),
    }
}

fn render_chain_conditions(conditions: &ChainConditions) -> Html {
    match conditions {
        ChainConditions::Condition(target, scorer) => html! {
            <p class={(target.score >= target.target).then_some("completed")}>
                {format! {"{}: {}/{}", chain_scorer_text(scorer),
                    target.score, target.target}}
            </p>
        }
    }
}

fn render_spin_conditions(conditions: &SpinConditions) -> Html {
    match conditions {
        SpinConditions::ZoneChainContext(handler, conditions) => html! {
            <div class="spin-context">
                <p>{if handler.b2b {
                    "B2B: On"
                } else {
                    "B2B: Off"
                }}</p>
                <p>{format! {"Combo: {}", handler.combo}}</p>
                //<hr class="separator"/>
                {if conditions.len() == 0 {
                    html! {
                        <p>{"None"}</p>
                    }
                } else {
                    conditions.iter()
                        .map(|c| render_chain_conditions(c))
                        .collect::<Html>()
                }}
                //<hr class="separator"/>
            </div>
        },
        SpinConditions::ChainContext(handler, conditions) => html! {
            <div class="spin-context">
                <p>{if handler.b2b {
                    "B2B: On"
                } else {
                    "B2B: Off"
                }}</p>
                <p>{format! {"Combo: {}", handler.combo}}</p>
                <p>{"No Zone"}</p>
                //<hr class="separator"/>
                {if conditions.len() == 0 {
                    html! {
                        <p>{"None"}</p>
                    }
                } else {
                    conditions.iter()
                        .map(|c| render_chain_conditions(c))
                        .collect::<Html>()
                }}
                //<hr class="separator"/>
            </div>
        },
        SpinConditions::Condition(target, scorer) => html! {
            <p class={(target.score >= target.target).then_some("completed")}>
                {format! {"{}: {}/{}", spin_scorer_text(scorer),
                    target.score, target.target}}
            </p>
        }
    }
}

fn render_event_conditions(conditions: &EventConditions) -> Html {
    match conditions {
        EventConditions::TSpinContext(_handler, conditions) => html! {
            <div class="spin-context">
                <p>{"Spins: T Only"}</p>
                //<hr class="separator"/>
                {if conditions.len() == 0 {
                    html! {
                        <p>{"None"}</p>
                    }
                } else {
                    conditions.iter()
                        .map(|c| render_spin_conditions(c))
                        .collect::<Html>()
                }}
                //<hr class="separator"/>
            </div>
        },
        EventConditions::AllSpinContext(_handler, conditions) => html! {
            <div class="spin-context">
                <p>{"Spins: All"}</p>
                //<hr class="separator"/>
                {if conditions.len() == 0 {
                    html! {
                        <p>{"None"}</p>
                    }
                } else {
                    conditions.iter()
                        .map(|c| render_spin_conditions(c))
                        .collect::<Html>()
                }}
                //<hr class="separator"/>
            </div>
        },
        EventConditions::Condition(target, scorer) => html! {
            <p class={(target.score >= target.target).then_some("completed")}>
                {format! {"{}: {}/{}", event_scorer_text(scorer),
                    target.score, target.target}}
            </p>
        }
    }
}

fn render_conditions(conditions: &Conditions) -> Html {
    if conditions.conditions.len() == 0 {
        html! {
            <p>{"None"}</p>
        }
    } else {
        conditions.conditions.iter()
            .map(|c| render_event_conditions(c))
            .collect::<Html>()
    }
}

#[function_component(ConditionsComponent)]
pub fn conditions(props: &Props) -> Html {
    html! {
        <div class="conditions">
            <div class="win-conditions">
                <p><strong>{"To Win:"}</strong></p>
                //<hr class="separator"/>
                {render_conditions(&props.win_conditions)}
                //<hr class="separator"/>
            </div>
            <div class="end-conditions">
                <p><strong>{"Ends When:"}</strong></p>
                //<hr class="separator"/>
                {render_conditions(&props.end_conditions)}
                //<hr class="separator"/>
            </div>
        </div>
    }
}