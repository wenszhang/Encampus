/**
 * Replies component, displaying a list of replies
 */
use leptos::*;
use chrono::FixedOffset;
use crate::components::focused_post::Reply;
use crate::components::focused_post::DarkenedCard;

#[component]
pub fn Replies(get_replies: Vec<Reply>, sort: ReadSignal<String>) -> impl IntoView {
    let get_replies_clone = get_replies.clone();

    let sorted_replies = move || {
        let sort_order = sort.get();
        let mut sorted_replies = get_replies_clone.clone();

        match sort_order.as_str() {
            "Oldest First" => sorted_replies.sort_by(|a, b| a.time.cmp(&b.time)),
            "Newest First" => sorted_replies.sort_by(|a, b| b.time.cmp(&a.time)),
            _ => (),
        }

        sorted_replies
    };

    view! {
        <For
            each=sorted_replies
            key=|reply| reply.replyid
            let:reply
        >
            <DarkenedCard class="p-5 ">
                <p class="font-bold">
                    "Answered by "
                    {reply.author_name.clone()}
                    {format!("{}", reply.time.checked_add_offset(FixedOffset::west_opt(6 * 3600).unwrap()).unwrap().format(" at %l %p on %b %-d"))}
                    ":"
                </p>
                <br/>
                <p>{reply.contents.clone()}</p>
            </DarkenedCard>
        </For>
    }
}
