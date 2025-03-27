use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(HowToPlay)]
pub fn how_to_play() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.back());

    html! {
        <div class="flex flex-col mx-auto mt-12 max-w-md gap-3 items-center bg-base">
            <h1 class="text-2xl font-bold bg-base">{ "Rules" }</h1>
            <p class="bg-base">{ "Just like in regular tic-tac-toe, the two players (X and O) take turns, starting with X. The game starts with X playing wherever they want in any of the 81 empty spots. Thereafter, each player moves in the small board corresponding to the position of the previous move in its small board, as indicated in the figures." }</p>
            <p class="bg-base"> { "If a move is played so that it wins a small board by the rules of normal tic-tac-toe, then the entire small board is marked as won by the player in the larger board. Once a small board is won by a player or it is filled completely, no more moves may be played in that board. If a player is sent to such a board, then that player may play in any other board. Game play ends when either a player wins the larger board or there are no legal moves remaining, in which case the game is a draw." }</p>
            <p class="text-center bg-base">{ "Read more on "}<a class="text-secondary bg-base" href="https://en.wikipedia.org/wiki/Ultimate_tic-tac-toe">{ "Wikipedia" }</a></p>
            <p class="text-center bg-base">{ "Or watch this " } <a class="text-secondary bg-base" href="https://www.youtube.com/watch?v=_Na3a1ZrX7c">{ "video" }</a> { " by Vsauce." }</p>
            <button class="font-semibold text-sm bg-primary rounded-full shadow-sm px-4 py-2 max-w-fit bg-base" onclick={onclick}>{"Back"}</button>
        </div>
    }
}
