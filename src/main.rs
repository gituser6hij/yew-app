use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <h1 style="color: #0acaa6; font-size: 2.8em;">{ "ONE RUST APP" }</h1>
        <div>
            <h2 style="color: #0acaa6; font-size: 1.8em;
			text-align: left;">{"A rust web front end app wit Yul and Rust"}</h2>
            
<p style="font-size: 1.7em;


			margin: 30px;
			padding: 15px;
			border-radius: 15px;
			border: 6px solid rgba(10, 202, 166, 0.9);
			transition: all 0.2s ease-in-out;
			/* add transition effect */
			display: inline-block;
			width: auto;">{ "Time: 02:08 am" }</p>
            <p style="font-size: 1.7em;


			margin: 30px;
			padding: 15px;
			border-radius: 15px;
			border: 6px solid rgba(10, 202, 166, 0.9);
			transition: all 0.2s ease-in-out;
			/* add transition effect */
			display: inline-block;
			width: auto;">{ "Date: 2023 03 22 - wed" }</p>
            <p style="font-size: 1.7em;


			margin: 30px;
			padding: 15px;
			border-radius: 15px;
			border: 6px solid rgba(10, 202, 166, 0.9);
			transition: all 0.2s ease-in-out;
			/* add transition effect */
			display: inline-block;
			width: auto;">{ "web3 development / web3 security" }</p>
        </div>
        <div style="font-size: 1.7em;


			margin: 30px;
			padding: 15px;
			border-radius: 15px;
			border: 6px solid rgba(10, 202, 166, 0.9);
			transition: all 0.2s ease-in-out;
			/* add transition effect */
			display: inline-block;
			width: auto;">
            <h3>{ "user137: hi" }</h3>
            <a href="http://web3.auditutils.com"><img src="http://web3.auditutils.com/user137.PNG" alt="user137 avatar" /></a>
        </div>
    </>
}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
