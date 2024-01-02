use askama::Template;

#[derive(Template)]
#[template(path = "+error.svelte.template", escape = "none")]
pub struct ErrorDotSvelte {}

#[derive(Template)]
#[template(path = "+layout.server.ts.template", escape = "none")]
pub struct LayoutDotServer {}

#[derive(Template)]
#[template(path = "+layout.svelte.template", escape = "none")]
pub struct LayoutDotSvelte {}

#[derive(Template)]
#[template(path = "+layout.ts.template", escape = "none")]
pub struct LayoutDotTs {}

#[derive(Template)]
#[template(path = "+page.server.ts.template", escape = "none")]
pub struct PageDotServer {}

#[derive(Template)]
#[template(path = "+page.svelte.template", escape = "none")]
pub struct PageDotSvelte {}

#[derive(Template)]
#[template(path = "+page.ts.template", escape = "none")]
pub struct PageDotTs {}

#[derive(Template)]
#[template(path = "+server.ts.template", escape = "none")]
pub struct ServerDotTs {}
