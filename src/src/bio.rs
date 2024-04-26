use leptos::*;

#[component]
pub fn Bio() -> impl IntoView {

    view! {
        <div id="bio">
            <h1>About Me</h1>
            <img src="/assets/NicoFancyDrink.jpg" alt="Nico Judge" />
            <p>
                I am a Full Stack Software Engineer with a passion for creating fast, performative applications and tooling
                that help developers create better software. I have experience with a wide range of technologies and
                frameworks, including Rust, JavaScript, TypeScript, React, and WebAssembly. I am always looking for new
                opportunities to learn and grow as a developer and am excited to see what the future holds for me in the
                tech industry.
            </p>
            <p>
                This website is powered by Leptos, a Rust 
                web framework that uses WebAssembly and server-side rendering to
                create fast, modern web applications and is deplyoyed and ran on 
                a docker file hosted on a linux server.
            </p>
        </div>
    } 
}
