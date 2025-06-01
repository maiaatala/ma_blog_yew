use yew::prelude::*;

use crate::components::pagelayout::DefaultPageLayout;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <DefaultPageLayout>
            <form class="contact-form">
                <h2>{ "Formulário de contato" }</h2>
                <label for="name">{ "Nome" }</label>
                <input
                    type="text"
                    name="Name"
                    id="name"
                    placeholder="Como deseja ser chamado"
                    required=true
                />
                <label for="email">{ "Email" }</label>
                <input
                    type="email"
                    name="Email"
                    id="email"
                    placeholder="Email para resposta"
                    required=true
                />
                <label for="message">{ "Mensagem" }</label>
                <textarea
                    name="Message"
                    id="message"
                    placeholder="Escreva sua mensagem..."
                    rows="5"
                    required=true
                />
                <button id="submit-btn" type="submit">
                    <span class="when-not-loading">{ "Enviar Mensagem" }</span>
                    <span class="when-loading" style="display: none;">{ "Enviando..." }</span>
                </button>
                <p class="success-msg">{ "Mensagem enviada com sucesso!" }</p>
                <p class="error-msg">{ "Erro ao enviar mensagem." }</p>
            </form>

            <article class="about-article">
                <h2>{ "Sobre o Projeto" }</h2>
                <p>
                    { "Este site faz parte de uma tese de graduação cujo objetivo é avaliar o desempenho de diferentes abordagens para construção de aplicações web interativas. Para isso, foram desenvolvidas três versões de uma mesma aplicação com funcionalidades equivalentes, utilizando stacks e frameworks distintos." }
                </p>
                <p>
                    { "Esta versão foi construída com WebAssembly e Yew, uma biblioteca que permite escrever interfaces reativas usando Rust. O foco está na performance de carregamento e na interatividade nativa do navegador." }
                </p>
            </article>

            <article class="about-article">
                <h2>{ "Sobre a Implementação" }</h2>
                <p>
                    { "O protótipo inicial foi concebido no Figma, priorizando uma navegação fluida e foco em legibilidade de conteúdo. Todas as versões do projeto consomem dados de uma " }
                    <strong>{ "API REST comum" }</strong>
                    { ", hospedada e gerenciada via " }
                    <strong>{ "Railway" }</strong>
                    { ". O uso de uma API padronizada garante igualdade de condições nos testes de performance entre as abordagens." }
                </p>
                <p>
                    { "Esta aplicação utiliza " }
                    <strong>{ "Rust" }</strong>
                    { " compilado para WebAssembly via " }
                    <strong>{ "Yew" }</strong>
                    { ", permitindo uma experiência semelhante à de frameworks JavaScript, mas com os benefícios de segurança e performance do Rust. O deploy foi realizado com foco em ambientes de testes controlados, garantindo comparabilidade entre versões." }
                </p>
            </article>
        </DefaultPageLayout>
    }
}

