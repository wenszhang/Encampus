use leptos::*;
use leptos_tiptap::*;

#[component]
fn StyleButton(
    label: &'static str,
    msg: TiptapInstanceMsg,
    set_msg: impl Fn(TiptapInstanceMsg) + Clone + 'static,
    selection_has_set: impl Fn() -> bool + Clone + 'static,
) -> impl IntoView {
    view! {
      <button
        class="rounded border"
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
        {label}
      </button>
    }
}

#[component]
pub fn RichTextBox() -> impl IntoView {
    let (msg, set_msg) = create_signal(TiptapInstanceMsg::Noop);
    let (value, set_value) = create_signal(r#"<h1>This is a simple <em><s>paragraph</s></em> ... <strong>H1</strong>!</h1><p style="text-align: center"><strong>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, <mark>sed diam nonumy</mark> eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.</strong></p><p style="text-align: justify">Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.</p>"#.to_owned());
    let (selection, set_selection) = create_signal(TiptapSelectionState::default());

    let spacer = view! { <div class="w-3" /> };

    view! {
      <div class="bg-white rounded-lg border border-slate-400">
        <div class="flex flex-row flex-wrap gap-1 p-2 border-b border-slate-400">
          <StyleButton
            label="H1"
            msg=TiptapInstanceMsg::H1
            set_msg=set_msg
            selection_has_set=move || selection().h1
          />
          <StyleButton
            label="Bold"
            msg=TiptapInstanceMsg::Bold
            set_msg=set_msg
            selection_has_set=move || selection().h1
          />
          <StyleButton
            label="Italic"
            msg=TiptapInstanceMsg::Italic
            set_msg=set_msg
            selection_has_set=move || selection().italic
          />
          <StyleButton
            label="Strike"
            msg=TiptapInstanceMsg::Strike
            set_msg=set_msg
            selection_has_set=move || selection().strike
          />
          {spacer.clone()}
          <StyleButton
            label="Blockquote"
            msg=TiptapInstanceMsg::Blockquote
            set_msg=set_msg
            selection_has_set=move || selection().blockquote
          />
          <StyleButton
            label="Highlight"
            msg=TiptapInstanceMsg::Highlight
            set_msg=set_msg
            selection_has_set=move || selection().highlight
          />
          {spacer}
          <StyleButton
            label="Align left"
            msg=TiptapInstanceMsg::AlignLeft
            set_msg=set_msg
            selection_has_set=move || selection().align_left
          />
          <StyleButton
            label="Align center"
            msg=TiptapInstanceMsg::AlignCenter
            set_msg=set_msg
            selection_has_set=move || selection().align_center
          />
          <StyleButton
            label="Align right"
            msg=TiptapInstanceMsg::AlignRight
            set_msg=set_msg
            selection_has_set=move || selection().align_right
          />
          <StyleButton
            label="Align justify"
            msg=TiptapInstanceMsg::AlignJustify
            set_msg=set_msg
            selection_has_set=move || selection().align_justify
          />
        </div>

        <TiptapInstance
          id="id"
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
          class="block px-2 pb-2 w-auto h-auto whitespace-pre-wrap"
        />
      </div>
    }
}
