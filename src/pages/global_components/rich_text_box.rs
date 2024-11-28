use crate::resources::images::svgs::{
    aligncenter_icon::AlignCenterIcon, alignjustify_icon::AlignJustifyIcon,
    alignleft_icon::AlignLeftIcon, alignright_icon::AlignRightIcon,
    blockquote_icon::BlockquoteIcon, bold_icon::BoldIcon, enlargetext_icon::EnlargeTextIcon,
    highlighter_icon::HighlighterIcon, italic_icon::ItalicIcon,
    strikethrough_icon::StrikethroughIcon,
};
use leptos::*;
use leptos_tiptap::*;

#[component]
fn StyleButton(
    msg: TiptapInstanceMsg,
    set_msg: impl Fn(TiptapInstanceMsg) + Clone + 'static,
    selection_has_set: impl Fn() -> bool + Clone + 'static,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class="rounded border self-center flex justify-center"
            class=(
                "shadow-lg",
                {
                    let selection_has_set = selection_has_set.clone();
                    move || !selection_has_set()
                },
            )

            class=("shadow-inner", selection_has_set.clone())
            class=("bg-slate-400", selection_has_set)
            on:click=move |_| set_msg(msg.clone())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn RichTextBox(
    id: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
) -> impl IntoView {
    let (msg, set_msg) = create_signal(TiptapInstanceMsg::Noop);
    let (selection, set_selection) = create_signal(TiptapSelectionState::default());

    let spacer = view! { <div class="w-3"></div> };

    view! {
        <div class="bg-white rounded-lg border border-slate-400 h-full flex flex-col">
            <div class="flex flex-row flex-wrap gap-1 p-2 border-b border-slate-400">
                // Enlarge
                <StyleButton
                    msg=TiptapInstanceMsg::H1
                    set_msg=set_msg
                    selection_has_set=move || selection().h1
                >
                    <EnlargeTextIcon size="2em"/>
                </StyleButton>

                // Bold
                <StyleButton
                    msg=TiptapInstanceMsg::Bold
                    set_msg=set_msg
                    selection_has_set=move || selection().bold
                >
                    <BoldIcon size="2em"/>
                </StyleButton>

                // Italic
                <StyleButton
                    msg=TiptapInstanceMsg::Italic
                    set_msg=set_msg
                    selection_has_set=move || selection().italic
                >
                    <ItalicIcon size="2em"/>
                </StyleButton>

                // Strikethrough
                <StyleButton
                    msg=TiptapInstanceMsg::Strike
                    set_msg=set_msg
                    selection_has_set=move || selection().strike
                >
                    <StrikethroughIcon size="2em"/>
                </StyleButton>

                // Highlight
                <StyleButton
                    msg=TiptapInstanceMsg::Highlight
                    set_msg=set_msg
                    selection_has_set=move || selection().highlight
                >
                    <HighlighterIcon size="2em"/>
                </StyleButton>

                // Align Left
                {spacer}
                <StyleButton
                    msg=TiptapInstanceMsg::AlignLeft
                    set_msg=set_msg
                    selection_has_set=move || selection().align_left
                >
                    <AlignLeftIcon size="2em"/>
                </StyleButton>

                // Align Center
                <StyleButton
                    msg=TiptapInstanceMsg::AlignCenter
                    set_msg=set_msg
                    selection_has_set=move || selection().align_center
                >
                    <AlignCenterIcon size="2em"/>
                </StyleButton>

                // Align Right
                <StyleButton
                    msg=TiptapInstanceMsg::AlignRight
                    set_msg=set_msg
                    selection_has_set=move || selection().align_right
                >
                    <AlignRightIcon size="2em"/>
                </StyleButton>

                // Align Justify
                <StyleButton
                    msg=TiptapInstanceMsg::AlignJustify
                    set_msg=set_msg
                    selection_has_set=move || selection().align_justify
                >
                    <AlignJustifyIcon size="2em"/>
                </StyleButton>

            </div>

            <TiptapInstance
                id=id
                msg=msg
                disabled=false
                value=value
                set_value=move |v| {
                    set_value
                        .set(
                            match v {
                                TiptapContent::Html(content) => content,
                                TiptapContent::Json(content) => content,
                            },
                        )
                }

                on_selection_change=move |state| set_selection.set(state)
                class="flex flex-1 flex-col p-1"
            />
        </div>
    }
}

#[component]
pub fn TiptapContentWrapper(raw_html: String) -> impl IntoView {
    view! { <div class="tiptap" inner_html=raw_html></div> }
}
