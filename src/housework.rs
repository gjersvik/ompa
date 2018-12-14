use chrono::{Duration, Utc, DateTime};
use std::collections::HashMap;
use actix::{
    Context,
    Actor,
};

use crate::ole_martin::{
    ActionType,
    Priority,
    Action,
    OleMartin,
    UpdateActions,
};

#[derive(Default)]
pub struct Chores{
    chores: HashMap<usize, Chore>,
}

impl Actor for Chores{
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        self.chores = get_chores();

        let actions = self.chores.iter().map(|(index, chore)|{
            Action {
                index: *index,
                name: chore.name.clone(),
                action_type: ActionType::Task( Priority::Important),
            }
        }).collect();

        OleMartin::addr().do_send(UpdateActions{name: "chores".to_string(), actions: actions});
    }
}

pub struct Chore {
    pub name: String,
    pub frequency: Duration,
    pub last_done: Option<DateTime<Utc>>,
}

impl Chore {
    fn new(name: &str, duration: Duration) -> Chore{
        Chore {name: name.to_string(), frequency: duration, last_done: None}
    }

    fn daily(name: &str) -> Chore {
        Chore::new(name, Duration::hours(18))
    }

    fn weekly(name: &str) -> Chore {
        Chore::new(name, Duration::days(6))
    }

    fn monthly(name: &str) -> Chore {
        Chore::new(name, Duration::days(30))
    }

    fn quarterly(name: &str) -> Chore {
        Chore::new(name, Duration::days(90))
    }

    fn yearly(name: &str) -> Chore {
        Chore::new(name, Duration::days(365))
    }
}


fn get_chores() -> HashMap<usize, Chore> {
    let mut chores = HashMap::new();
    chores.insert(100, Chore::daily("Vaske opp"));
    chores.insert(101, Chore::daily("Rydde"));
    chores.insert(102, Chore::daily("Dusj"));
    chores.insert(103, Chore::new("Tanpuss", Duration::hours(8)));

    chores.insert(200, Chore::new("Vaske klær", Duration::days(2)));
    chores.insert(201, Chore::weekly("Kjøkken benk"));
    chores.insert(202, Chore::weekly("Vaske stue"));
    chores.insert(203, Chore::weekly("Vaske gang"));
    chores.insert(204, Chore::weekly("Vaske kontor"));
    chores.insert(205, Chore::weekly("Vaske bad"));
    chores.insert(206, Chore::weekly("Vaske såverom"));
    chores.insert(207, Chore::weekly("Ta ut Søppel"));
    chores.insert(208, Chore::weekly("MailCall"));
    chores.insert(209, Chore::weekly("Vaske stue"));

    chores.insert(300, Chore::monthly("Vaske Skaptoper"));
    chores.insert(301, Chore::monthly("Sengklær"));
    chores.insert(302, Chore::monthly("Trapper"));
    chores.insert(303, Chore::monthly("Pcer"));
    chores.insert(304, Chore::monthly("Vaske dusj"));

    chores.insert(400, Chore::quarterly("Micro"));
    chores.insert(401, Chore::quarterly("Renske Vaskemakin"));
    chores.insert(402, Chore::quarterly("Komfyr"));
    chores.insert(403, Chore::quarterly("Sofa"));
    chores.insert(404, Chore::quarterly("Kjøleskap"));

    chores.insert(500, Chore::yearly("Stue - Tv benk"));
    chores.insert(501, Chore::yearly("Bad - Vegger"));
    chores.insert(502, Chore::yearly("Vaskeopp - Vifte skap"));
    chores.insert(503, Chore::yearly("Stue - Vegger"));
    chores.insert(504, Chore::yearly("Kontor - Tak"));
    chores.insert(505, Chore::yearly("Kjøkken - Under Vasken"));
    chores.insert(506, Chore::yearly("Stue - Gulv"));
    chores.insert(507, Chore::yearly("Såverom - Vegger"));
    chores.insert(508, Chore::yearly("Vaskeopp - Kjøken Skap"));
    chores.insert(509, Chore::yearly("Bad - Gulv"));
    chores.insert(510, Chore::yearly("Vaskeopp - Skuffer"));
    chores.insert(511, Chore::yearly("Kjøkken - Mat skap"));
    chores.insert(512, Chore::yearly("Kjøkken - Vifte skap"));
    chores.insert(513, Chore::yearly("Gang - Tak"));
    chores.insert(514, Chore::yearly("Kjøkken - Kjøken Skap"));
    chores.insert(515, Chore::yearly("Stue - Pc Bord"));
    chores.insert(516, Chore::yearly("Vaskeopp - Ting skap"));
    chores.insert(517, Chore::yearly("Såverom - Tak"));
    chores.insert(518, Chore::yearly("Kjøkken - Ting skap"));
    chores.insert(519, Chore::yearly("Stue - Tak"));
    chores.insert(520, Chore::yearly("Kontor - Viduer"));
    chores.insert(521, Chore::yearly("Kontor - Vaske Skap"));
    chores.insert(522, Chore::yearly("Kjøkken - Te skap"));
    chores.insert(523, Chore::yearly("Stue - Tv bord"));
    chores.insert(524, Chore::yearly("Kjøkken - Pc skap"));
    chores.insert(525, Chore::yearly("Kjøkken - Gryte skap"));
    chores.insert(526, Chore::yearly("Vaskeopp - Under Vasken"));
    chores.insert(527, Chore::yearly("Såverom - Klærskap"));
    chores.insert(528, Chore::yearly("Kontor - Pult"));
    chores.insert(529, Chore::yearly("Kjøkken - Skuffer"));
    chores.insert(530, Chore::yearly("Såverom - Gulv"));
    chores.insert(531, Chore::yearly("Gang - Gulv"));
    chores.insert(532, Chore::yearly("Kontor - Gulv"));
    chores.insert(533, Chore::yearly("Stue - Viduer"));
    chores.insert(534, Chore::yearly("Gang - Vegger"));
    chores.insert(535, Chore::yearly("Såverom - Viduer"));
    chores.insert(536, Chore::yearly("Vaskeopp - Gryte skap"));
    chores.insert(537, Chore::yearly("Kontor - Vegger"));
    chores.insert(538, Chore::yearly("Bad - Tak"));

    chores
}