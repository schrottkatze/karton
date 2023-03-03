use crate::args::ARGS;

markup::define!{
    Create() {
        @BasePage {
            body: markup::new! {
                form [ 
                    id = "pasta-form",
                    action = "upload",
                    method = "POST",
                    enctype = "multipart/form-data",
                ] {
                    br;
                }
            }
        }
    }
    BasePage<Body: markup::Render>( body: Body ) {
        @markup::doctype()
        html {
            head {
                title { @ARGS.title }
                meta[charset = "utf-8"];
                meta[
                    name = "viewport", 
                    content = "width=device-width, initial-scale=1.0"
                ];
                link[ 
                    rel = "stylesheet",
                    href = if let Some(custom_css_url) = ARGS.custom_css.as_ref() {
                        custom_css_url.clone()
                    } else {
                        format!("{}/static/water.css", ARGS.public_path)
                    }
                ];
            }
            body[
                style = format!(
                    "margin:auto;padding-left: 0.5rem;padding-right: 0.5rem;line-height:1.5;font-size: 1.1em;{}", 
                    if ARGS.wide { "1080px" } else { "800px" }
                ) 
            ] {
                br;
                b[ style = "margin-right: 0.5rem" ] {
                    @if !ARGS.hide_logo {
                        a[ href = format!("{}/", ARGS.public_path) ] {
                            img[ 
                                width = 26,
                                style = "margin-bottom: -6px; margin-right: 0.5rem;",
                                src = format!("{}/static/logo.png", ARGS.public_path)
                            ];
                        }
                        @ARGS.title
                   }
                }
                a[ href = format!("{}/", ARGS.public_path), style = "margin-right: 0.5rem; margin-left: 0.5rem" ] "New"
                a[ href = format!("{}/pastalist", ARGS.public_path), style = "margin-right: 0.5rem; margin-left: 0.5rem" ] "List"
                a[ href = format!("{}/info", ARGS.public_path), style = "margin-right: 0.5rem; margin-left: 0.5rem" ] "Info"
                hr;

                @body

                hr;
                // TODO: footer text but like with proper markdown and customizability
                p[ style = "font-size:smaller" ] {
                    @if let Some(footer_text) = ARGS.footer_text.as_ref() {
                        @footer_text
                    } else {
                        a[ href = "https://microbin.eu" ] "MicroBin"
                        " by Dániel Szabó and the FOSS Community."
                        "Let's keep the Web "
                        b "compact" ", "
                        b "accessible" " and "
                        b "humane" "!"
                    }
                }
            }
        }
    }
}
