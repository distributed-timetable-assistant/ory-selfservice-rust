use crate::kratos::models::*;
use leptos::prelude::*;

#[component]
pub fn KratosMessages(messages: Option<Vec<UiText>>) -> AnyView {
    let messages = messages.unwrap_or_default();
    if messages.is_empty() {
        return ().into_any();
    }

    view! {
        <div class="space-y-2 mb-4">
            {messages.into_iter().map(|msg| {
                let alert_class = match msg.text_type.as_str() {
                    "error" => "bg-red-950/50 border border-red-500/50 text-red-200 p-3 rounded-lg text-sm",
                    "info" => "bg-blue-950/50 border border-blue-500/50 text-blue-200 p-3 rounded-lg text-sm",
                    "success" => "bg-green-950/50 border border-green-500/50 text-green-200 p-3 rounded-lg text-sm",
                    _ => "bg-slate-800 border border-slate-700 text-slate-200 p-3 rounded-lg text-sm",
                };
                view! {
                    <div class=alert_class role="alert">
                        {msg.text}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }.into_any()
}

#[component]
pub fn KratosNodeInput(
    attr: UiNodeInputAttributes,
    meta: UiNodeMeta,
    messages: Vec<UiText>,
) -> AnyView {
    let name = attr.name.clone();
    let input_type = attr.input_type.clone();
    let value = attr
        .value
        .and_then(|v| {
            if v.is_null() {
                None
            } else if let serde_json::Value::String(s) = v {
                Some(s)
            } else {
                Some(v.to_string())
            }
        })
        .unwrap_or_default();

    let disabled = attr.disabled;
    let required = attr.required.unwrap_or(false);

    // Label determination
    let label_text = attr
        .label
        .or(meta.label)
        .map(|l| l.text)
        .unwrap_or_else(|| {
            if input_type == "submit" || input_type == "button" {
                "".to_string()
            } else {
                name.clone()
            }
        });

    if input_type == "hidden" {
        return view! {
            <input type="hidden" name=name.clone() value=value.clone() />
        }
        .into_any();
    }

    if input_type == "submit" || input_type == "button" {
        let is_primary = name == "method"
            || name == "provider"
            || name.contains("password")
            || name.contains("totp")
            || name.contains("webauthn")
            || name == "submit";

        let btn_class = if is_primary {
            "w-full py-2.5 px-4 bg-blue-600 hover:bg-blue-500 disabled:bg-blue-800/50 disabled:text-slate-400 text-white font-semibold rounded-lg shadow-lg hover:shadow-blue-500/20 transition-all duration-200 text-sm cursor-pointer"
        } else {
            "w-full py-2.5 px-4 bg-slate-800 hover:bg-slate-700 disabled:bg-slate-800/50 disabled:text-slate-400 text-slate-200 font-semibold border border-slate-700 rounded-lg transition-all duration-200 text-sm cursor-pointer"
        };

        let submit_value = if !value.is_empty() {
            value.clone()
        } else {
            name.clone()
        };

        return view! {
            <div class="mt-4">
                <button
                    type="submit"
                    name=name.clone()
                    value=submit_value
                    disabled=disabled
                    class=btn_class
                >
                    {label_text}
                </button>
            </div>
        }
        .into_any();
    }

    let field_class = "w-full px-3.5 py-2 bg-slate-900 border border-slate-700 focus:border-blue-500 focus:ring-1 focus:ring-blue-500 text-slate-100 placeholder-slate-500 rounded-lg text-sm transition-colors duration-200 outline-none";
    let placeholder_text = format!("Enter {}...", name.replace("_", " "));

    view! {
        <div class="space-y-1.5 mt-3">
            <label class="block text-xs font-semibold text-slate-400 uppercase tracking-wider">
                {label_text}
                {if required { " *" } else { "" }}
            </label>
            <input
                type=input_type
                name=name
                value=value
                disabled=disabled
                required=required
                class=field_class
                placeholder=placeholder_text
            />
            {if !messages.is_empty() {
                view! {
                    <div class="space-y-1 mt-1">
                        {messages.into_iter().map(|msg| view! {
                            <p class="text-xs text-red-400">{msg.text}</p>
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
    .into_any()
}

#[component]
pub fn KratosNode(node: UiNode) -> AnyView {
    let messages = node.messages.clone();
    let meta = node.meta.clone();

    match node.attributes {
        UiNodeAttributes::Input(attr) => view! {
            <KratosNodeInput attr=attr meta=meta messages=messages />
        }.into_any(),
        UiNodeAttributes::Anchor(attr) => view! {
            <div class="my-4 text-center">
                <a
                    id=attr.id
                    href=attr.href
                    class="text-sm text-blue-400 hover:text-blue-300 hover:underline transition-colors"
                >
                    {attr.title.text}
                </a>
            </div>
        }.into_any(),
        UiNodeAttributes::Image(attr) => view! {
            <div class="my-4 flex justify-center">
                <img
                    id=attr.id
                    src=attr.src
                    width=attr.width
                    height=attr.height
                    alt=attr.alt.unwrap_or_default()
                    class="max-w-full h-auto rounded"
                />
            </div>
        }.into_any(),
        UiNodeAttributes::Text(attr) => view! {
            <div class="my-3 p-3 bg-slate-900 border border-slate-800 rounded-lg text-sm text-slate-300">
                <p>{attr.text.text}</p>
            </div>
        }.into_any(),
        UiNodeAttributes::Script(attr) => view! {
            <script
                id=attr.id
                src=attr.src
                type=attr.script_type
                async=attr.async_src
                crossorigin=attr.crossorigin
                integrity=attr.integrity
                referrerpolicy=attr.referrerpolicy
            ></script>
        }.into_any(),
        UiNodeAttributes::Division(_attr) => view! {
            <div class="my-3"></div>
        }.into_any(),
        UiNodeAttributes::Unknown => ().into_any(),
    }
}

#[component]
pub fn KratosForm(ui: UiContainer) -> impl IntoView {
    let action = ui.action.clone();
    let method = ui.method.clone();

    view! {
        <form action=action method=method class="space-y-4">
            <KratosMessages messages=ui.messages />
            {ui.nodes.into_iter().map(|node| view! {
                <KratosNode node=node />
            }).collect::<Vec<_>>()}
        </form>
    }
}
