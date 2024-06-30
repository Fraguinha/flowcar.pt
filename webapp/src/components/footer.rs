use chrono::Datelike;
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="position-relative bottom text-center text-bg-light">
            <div class="container">
                <div class="row justify-content-around py-5">
                    <div class="col-12 col-lg-4 py-3">
                        <h5>Contacts</h5>
                        <a href="tel:+351939699442">{"939 699 442"}</a>
                        <br/>
                        <a href="tel:+351939699447">{"939 699 447"}</a>
                        <br/>
                        <a href="mailto:geral@flowcar.pt">{"geral@flowcar.pt"}</a>
                    </div>
                    <div class="col-12 col-lg-4 py-3">
                        <h5>Links</h5>
                        <div class="row justify-content-center pt-3">
                            <div class="col-auto">
                                <a href="https://www.facebook.com/p/Flow-Car-Studio-100084819075644/">
                                    <i class="bi bi-facebook"></i>
                                </a>
                            </div>
                            <div class="col-auto">
                                <a href="https://www.instagram.com/flowcarstudio/">
                                    <i class="bi bi-instagram"></i>
                                </a>
                            </div>
                            <div class="col-auto">
                                <a href="https://www.youtube.com/@FlowCarStudio">
                                    <i class="bi bi-youtube"></i>
                                </a>
                            </div>
                        </div>
                    </div>
                    <div class="col-12 col-lg-4 py-3">
                        <h5>Address</h5>
                        <a href="https://maps.google.com/?saddr=Current+Location&daddr=Flow+Car+Studio&hl=pt-PT">
                            {"R. Rio 207, 4475-493 Maia, Portugal"}
                        </a>
                    </div>
                </div>
                <div class="row justify-content-center">
                    <div class="col-auto p-3 text-bg-light">
                        {format!("© {} Flow Car Studio, created by: ", chrono::Utc::now().year())}
                        <a href="https://fraguinha.com/">{"Fraguinha"}</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}
