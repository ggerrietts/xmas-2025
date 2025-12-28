use crate::models::get_page_cards;
use yew::prelude::*;

#[function_component(PrintComponent)]
pub fn print_component() -> Html {
    let cards = get_page_cards();

    html! {
        <div class="print-container">
            <style>
                {r#"
                    @media print {
                        body {
                            margin: 0;
                            padding: 0;
                        }
                        .print-container {
                            margin: 0;
                            padding: 0;
                        }
                    }

                    @page {
                        size: 3.75in 5.25in;
                        margin: 0;
                    }

                    .card {
                        width: 3.75in;
                        height: 5.25in;
                        page-break-after: always;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        justify-content: space-between;
                        padding: 0.25in;
                        box-sizing: border-box;
                        font-family: Georgia, serif;
                        border: 1px solid #ccc;
                        page-break-inside: avoid;
                    }

                    .card-content {
                        max-width: 3.25in;
                        max-height: 4.75in;
                        text-align: center;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        gap: 0.3in;
                    }

                    .card-title {
                        font-size: 14px;
                        font-weight: bold;
                        color: #2d5016;
                    }

                    .card-location {
                        font-size: 16px;
                        font-weight: bold;
                        color: #1a1a1a;
                    }

                    .card-message {
                        font-size: 12px;
                        line-height: 1.4;
                        color: #333;
                        margin: 1em 0;
                    }

                    .card-codeword {
                        font-size: 14px;
                        font-weight: bold;
                        color: #c41e3a;
                        font-family: monospace;
                        letter-spacing: 0.05em;
                    }

                    .card-art {
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        width: 200px;
                        height: 200px;
                    }

                    .card-art img {
                        max-width: 100%;
                        max-height: 100%;
                        width: auto;
                        height: auto;
                        object-fit: contain;
                    }

                    .card-footer {
                        font-size: 10px;
                        color: #666;
                        margin-bottom: 2em;
                    }
                "#}
            </style>

            {cards.iter().enumerate().map(|(idx, page)| {
                html! {
                    <div class="card" key={idx}>
                        <div class="card-content">
                            <div class="card-title">{"🎅 Congratulations! 🎄"}</div>

                            <div class="card-message">
                                {"You've found the clue"}
                                <br/>
                                <div class="card-location">
                                    {&page.location}
                                </div>
                            </div>

                            <div class="card-art">
                                <img
                                    src={format!("static/art-{}.png", idx % 6)}
                                    alt="Holiday clip art"
                                />
                            </div>

                            <div class="card-message">
                                {"The code word for the next phase is:"}
                                <br/>
                                <div class="card-codeword">
                                    {&page.next_url}
                                </div>
                            </div>
                        </div>
                    </div>
                }
            }).collect::<Html>()}
        </div>
    }
}
