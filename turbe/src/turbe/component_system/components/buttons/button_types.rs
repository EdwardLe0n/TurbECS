use crate::GameState;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ButtonTypes {
    Default,

    Misc,
    Test,

    // User made buttons
    Play,
    Quit,
    Restart,
    ToReady,
    ToBattle,
    ToShop,
    ToTitle,
    ToIntro,
    ToLiveFeed,

    Post,
    Discard,
    BattleWord,
    BattlePostNotif,

    BattlePostManager,
    BPMoveUp,
    BPMoveDown,

    Close,

    DeckSelect,

    SwipeRight,
    SwipeLeft,

    RandomPicture,
    RandomFirstWord,
    RandomSecondWord,
    RandomThirdWord,

    ThinkOfAWord,
    WorkOnInteractions,
    WorkingOnSomeInteraction,

    CancelPurchase,
    GetRecentlyNewWord,
    GetPlayNYCWord,
    GetRandomWord,
    GetRandomAdjective,
    GetRandomAdverb,
    GetRandomConjunction,
    GetRandomDeterminer,
    GetRandomInterjection,
    GetRandomNoun,
    GetRandomPreposition,
    GetRandomPronoun,
    GetRandomVerb,
    ShopWord,

}

impl ButtonTypes {

    pub fn can_still_interact(&self, _state : &mut GameState) -> bool {

        match self {
            ButtonTypes::BattlePostNotif => {return true;},
            ButtonTypes::Close => {return true;},
            _default => {return false;}
        }

    }

    pub fn get_string(&self) -> String {

        match self {
            ButtonTypes::GetRecentlyNewWord => {return "new words here!".to_string();}
            ButtonTypes::GetPlayNYCWord => {return "play nyc reccomendations".to_string();}
            ButtonTypes::GetRandomWord => {return "random word!".to_string();}
            ButtonTypes::GetRandomAdjective => {return "random adjective~".to_string();}
            ButtonTypes::GetRandomAdverb => {return "~random adverb".to_string();}
            ButtonTypes::GetRandomConjunction => {return "random_conjunction".to_string();}
            ButtonTypes::GetRandomDeterminer => {return "random->determiner".to_string();}
            ButtonTypes::GetRandomInterjection => {return "ran _ interjection _ dom".to_string();}
            ButtonTypes::GetRandomNoun => {return "noun".to_string();}
            ButtonTypes::GetRandomPreposition => {return "preposition random".to_string();}
            ButtonTypes::GetRandomPronoun => {return "PROnoun :3".to_string();}
            ButtonTypes::GetRandomVerb => {return "verb!? ~".to_string();}
            _default => {return "womp womp".to_string();}

        }

    }

}