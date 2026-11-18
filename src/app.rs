use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Title text="Khairul — Portfolio"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav>
            <span class="logo">K.</span>
            <ul>
                <li><a href="#about">About</a></li>
                <li><a href="#skills">Skills</a></li>
                <li><a href="#projects">Projects</a></li>
                <li><a href="#contact">Contact</a></li>
            </ul>
        </nav>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Nav/>

        <section class="hero">
            <h1>"Hi, I'm Khairul"</h1>
            <p>
                "Full-stack developer building fast, reliable apps with Rust, \
                 Java, Flutter and the web. Welcome to my corner of the internet."
            </p>
            <div>
                <a class="btn btn-primary" href="#projects">View Projects</a>
                <a class="btn btn-outline" href="#contact">Get in Touch</a>
            </div>
        </section>

        <section id="about" class="about">
            <h2>About Me</h2>
            <p class="sub">Who I am and what I do</p>
            <p>
                "I'm a developer who loves turning ideas into working software. \
                 My day-to-day stack spans systems programming in Rust, backend \
                 services in Java/Spring Boot and Laravel, cross-platform apps \
                 with Flutter, and desktop apps with Tauri and Electron. \
                 I care about clean code, good tooling, and shipping things that work."
            </p>
        </section>

        <section id="skills">
            <h2>Skills</h2>
            <p class="sub">Technologies I work with</p>
            <div class="skills-grid">
                <div class="card"><h3>Rust</h3><p>Leptos, Axum, Cargo, WASM</p></div>
                <div class="card"><h3>Java</h3><p>Spring Boot, Maven, Gradle, JDK 21</p></div>
                <div class="card"><h3>Web</h3><p>TypeScript, Node.js, React, Tailwind</p></div>
                <div class="card"><h3>Mobile</h3><p>Flutter, Dart, Android SDK</p></div>
                <div class="card"><h3>Desktop</h3><p>Tauri, Electron, GTK/Linux</p></div>
                <div class="card"><h3>Backend</h3><p>Laravel, PHP, MySQL, PostgreSQL</p></div>
            </div>
        </section>

        <section id="projects">
            <h2>Projects</h2>
            <p class="sub">"Things I've built"</p>
            <div class="projects-grid">
                <div class="card project-card">
                    <h3><a href="#">This Portfolio</a></h3>
                    <p>"A fast, server-rendered portfolio built with Rust and the Leptos framework, hydrated in the browser with WASM."</p>
                    <div class="tags"><span class="tag">Rust</span><span class="tag">Leptos</span><span class="tag">WASM</span></div>
                </div>
                <div class="card project-card">
                    <h3><a href="#">Spring Boot API</a></h3>
                    <p>"REST API boilerplate with Spring Boot 3, JPA, validation and PostgreSQL."</p>
                    <div class="tags"><span class="tag">Java</span><span class="tag">Spring Boot</span><span class="tag">PostgreSQL</span></div>
                </div>
                <div class="card project-card">
                    <h3><a href="#">Flutter App</a></h3>
                    <p>"Cross-platform mobile app built with Flutter and Dart."</p>
                    <div class="tags"><span class="tag">Flutter</span><span class="tag">Dart</span><span class="tag">Android</span></div>
                </div>
            </div>
        </section>

        <section id="contact" class="contact">
            <h2>Contact</h2>
            <p class="sub">"Let's work together"</p>
            <p>"Have a project in mind or just want to say hi? Reach out anytime."</p>
            <div class="links">
                <a href="mailto:hello@example.com">Email</a>
                <a href="https://github.com/">GitHub</a>
                <a href="https://linkedin.com/">LinkedIn</a>
            </div>
        </section>

        <footer>"Built with Rust & Leptos — © 2026 Khairul"</footer>
    }
}
