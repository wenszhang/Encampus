/**
 * Component for the text area used in text editor
 */
use leptos::{component, view, IntoView};

#[component]
pub fn TextAreaIcon() -> impl IntoView {
    view! {
      <div class="flex justify-center pl-5">
        <div class="flex items-center h-12 cursor-not-allowed">
          // Bold
          <svg class="mr-2 w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16">
            <path d="M4 2h4.5a3.501 3.501 0 0 1 2.852 5.53A3.499 3.499 0 0 1 9.5 14H4a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1m1 7v3h4.5a1.5 1.5 0 0 0 0-3Zm3.5-2a1.5 1.5 0 0 0 0-3H5v3Z" />
          </svg>

          // Image Icon
          <svg class="mr-2 w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M19.999 4h-16c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2m-13.5 3a1.5 1.5 0 1 1 0 3a1.5 1.5 0 0 1 0-3m5.5 10h-7l4-5l1.5 2l3-4l5.5 7z" />
          </svg>

          // Line Spacing Icon
          <svg class="mr-2 w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M10 8h11c.6 0 1-.4 1-1s-.4-1-1-1H10c-.6 0-1 .4-1 1s.4 1 1 1m-4.3 7.3V8.7c.2.2.4.3.6.3c.3 0 .5-.1.7-.2c.4-.4.5-1 .1-1.4l-1.7-2C5.2 5.1 5 5 4.7 5s-.6.1-.8.4l-1.7 2c-.3.4-.3 1 .2 1.4c.4.3.9.3 1.3 0v6.6c-.4-.3-.9-.4-1.3 0s-.5 1-.1 1.4l1.7 2c.1.1.4.2.7.2s.6-.1.8-.4l1.7-2c.4-.4.3-1.1-.1-1.4c-.5-.3-1.1-.3-1.4.1M21 11H10c-.6 0-1 .4-1 1s.4 1 1 1h11c.6 0 1-.4 1-1s-.4-1-1-1m0 5H10c-.6 0-1 .4-1 1s.4 1 1 1h11c.6 0 1-.4 1-1s-.4-1-1-1" />
          </svg>

          <svg
            class="mr-2 w-5 h-5"
            xmlns="http://www.w3.org/2000/svg"
            width="1em"
            height="1em"
            viewBox="0 0 24 24"
          >
            <path
              fill="currentColor"
              d="M8.7 15.9L4.8 12l3.9-3.9a.984.984 0 0 0 0-1.4a.984.984 0 0 0-1.4 0l-4.59 4.59a.996.996 0 0 0 0 1.41l4.59 4.6c.39.39 1.01.39 1.4 0a.984.984 0 0 0 0-1.4m6.6 0l3.9-3.9l-3.9-3.9a.984.984 0 0 1 0-1.4a.984.984 0 0 1 1.4 0l4.59 4.59c.39.39.39 1.02 0 1.41l-4.59 4.6a.984.984 0 0 1-1.4 0a.984.984 0 0 1 0-1.4"
            />
          </svg>

          // Quote Icon
          <svg class="mr-2 w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
              fill-rule="evenodd"
              d="M9.213 12.75c-.084.774-.308 1.392-.698 1.94c-.523.731-1.4 1.414-2.85 2.14a.75.75 0 1 0 .67 1.34c1.55-.774 2.673-1.591 3.4-2.61c.741-1.036 1.015-2.203 1.015-3.56V7.5A1.75 1.75 0 0 0 9 5.75H5A1.75 1.75 0 0 0 3.25 7.5V11c0 .966.784 1.75 1.75 1.75zm10 0c-.084.774-.308 1.392-.698 1.94c-.523.731-1.4 1.414-2.85 2.14a.75.75 0 1 0 .67 1.34c1.55-.774 2.673-1.591 3.4-2.61c.741-1.036 1.015-2.203 1.015-3.56V7.5A1.75 1.75 0 0 0 19 5.75h-4a1.75 1.75 0 0 0-1.75 1.75V11c0 .966.784 1.75 1.75 1.75z"
              clip-rule="evenodd"
            />
          </svg>

          // Redo
          <svg
            class="mr-2 w-5 h-5"
            xmlns="http://www.w3.org/2000/svg"
            width="1em"
            height="1em"
            viewBox="-1 -2 24 24"
          >
            <path
              fill="currentColor"
              d="m19.347 7.24l.847-1.266a.984.984 0 0 1 1.375-.259c.456.31.58.93.277 1.383L19.65 10.38a.984.984 0 0 1-1.375.259L14.97 8.393a1.002 1.002 0 0 1-.277-1.382a.984.984 0 0 1 1.375-.26l1.344.915C16.428 4.386 13.42 2 9.863 2c-4.357 0-7.89 3.582-7.89 8s3.533 8 7.89 8c.545 0 .987.448.987 1s-.442 1-.987 1C4.416 20 0 15.523 0 10S4.416 0 9.863 0c4.504 0 8.302 3.06 9.484 7.24"
            />
          </svg>

          // Undo
          <svg class="mr-2 w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48">
            <g
              fill="none"
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="4"
            >
              <path d="M11.272 36.728A17.943 17.943 0 0 0 24 42c9.941 0 18-8.059 18-18S33.941 6 24 6c-4.97 0-9.47 2.015-12.728 5.272C9.614 12.93 6 17 6 17" />
              <path d="M6 9v8h8" />
            </g>
          </svg>
        </div>
      </div>
    }
}
