//! Pantalla de primer inicio: obliga a elegir un nombre offline.

use gpui::{
    div, prelude::*, px, FontWeight, SharedString, Context, Window, ClickEvent,
    KeyDownEvent, FocusHandle,
};
use crate::i18n::tr;
use crate::line_edit::LineEdit; // o el componente de input que uses

pub struct LoginPage {
    pub name: String,
    pub error: Option<String>,
    pub focus: FocusHandle,
}

impl LoginPage {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            name: String::new(),
            error: None,
            focus: cx.focus_handle(),
        }
    }

    /// Valida el nombre offline (3-16 chars, a-zA-Z0-9_)
    pub fn validate(name: &str) -> Result<(), String> {
        let name = name.trim();
        if name.len() < 3 {
            return Err(tr("El nombre debe tener al menos 3 caracteres").into());
        }
        if name.len() > 16 {
            return Err(tr("El nombre no puede tener más de 16 caracteres").into());
        }
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(tr("Solo letras, números y guión bajo (_)").into());
        }
        Ok(())
    }
}

impl Render for LoginPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let name = self.name.clone();
        let has_error = self.error.is_some();

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(gpui::rgb(0x0a0a0c)) // fondo casi negro estilo OLED
            .child(
                div()
                    .w(px(420.))
                    .flex()
                    .flex_col()
                    .gap(px(24.))
                    .p(px(32.))
                    .rounded(px(16.))
                    .bg(gpui::rgb(0x141418))
                    .border_1()
                    .border_color(gpui::rgba(0xffffff.into(), 0.08))
                    // Logo / título
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(8.))
                            .child(
                                div()
                                    .text_size(px(28.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(gpui::white())
                                    .child(tr("Bienvenido a Horizon")),
                            )
                            .child(
                                div()
                                    .text_size(px(14.))
                                    .text_color(gpui::rgba(0xffffff.into(), 0.55))
                                    .child(tr("Elige cómo te verán en el juego")),
                            ),
                    )
                    // Input
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.))
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(gpui::rgba(0xffffff.into(), 0.7))
                                    .child(tr("Nombre de usuario")),
                            )
                            .child(
                                // Aquí usa tu LineEdit / input existente
                                // Ejemplo simplificado:
                                div()
                                    .id("login-name")
                                    .h(px(48.))
                                    .px(px(16.))
                                    .rounded(px(10.))
                                    .bg(gpui::rgb(0x1c1c22))
                                    .border_1()
                                    .border_color(if has_error {
                                        gpui::rgb(0xef4444)
                                    } else {
                                        gpui::rgba(0xffffff.into(), 0.1)
                                    })
                                    .flex()
                                    .items_center()
                                    .child(
                                        // Reemplaza por tu componente real de texto
                                        // Por ahora placeholder visual
                                        div()
                                            .text_size(px(15.))
                                            .text_color(gpui::white())
                                            .child(if name.is_empty() {
                                                SharedString::from(tr("Ej: Steve_123"))
                                            } else {
                                                SharedString::from(name.clone())
                                            }),
                                    ),
                            )
                            .when_some(self.error.clone(), |d, err| {
                                d.child(
                                    div()
                                        .text_size(px(12.))
                                        .text_color(gpui::rgb(0xef4444))
                                        .child(err),
                                )
                            }),
                    )
                    // Botón Continuar
                    .child(
                        div()
                            .id("login-continue")
                            .h(px(48.))
                            .w_full()
                            .rounded(px(10.))
                            .bg(gpui::rgb(0x22c55e)) // verde estilo AstralRinth
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.9))
                            .child(
                                div()
                                    .text_size(px(15.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(gpui::rgb(0x052e16))
                                    .child(tr("Continuar")),
                            )
                            .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                                match LoginPage::validate(&this.name) {
                                    Ok(()) => {
                                        // Aquí se guarda el nombre y se cierra la pantalla
                                        // (se conecta desde main.rs)
                                        cx.emit(LoginEvent::Continue(this.name.trim().to_string()));
                                    }
                                    Err(e) => {
                                        this.error = Some(e);
                                        cx.notify();
                                    }
                                }
                            })),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(gpui::rgba(0xffffff.into(), 0.4))
                            .text_center()
                            .child(tr("Solo se usará en modo offline. Puedes cambiarlo después.")),
                    ),
            )
    }
}

#[derive(Clone)]
pub enum LoginEvent {
    Continue(String),
}
