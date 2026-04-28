#![windows_subsystem = "windows"]
mod aiwork;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
const APPID: &str = "ru.teovr.documentai";

struct VM {
    file1 : String,
    file2 : String,
    output_text : String
}

#[derive(Debug)]
enum AppMsg {
    OpenFile1,
    OpenFile2,
    Analyze,
    AnalysisResult(String)
}

#[relm4::component]
impl SimpleComponent for VM {
    type Init = String;

    type Input = AppMsg;
    type Output = ();
    view! {
        gtk::Window {
            set_title: Some("DocumentAI"),
            set_default_width: 600,
            set_default_height: 700,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 6,
                set_margin_all: 8,

                gtk::Button {
                    set_hexpand: false,
                    set_halign: gtk::Align::Start,
                    set_size_request: (250, -1),
                    set_label: "Выберите эталонный документ:",
                    connect_clicked => AppMsg::OpenFile1
                },
                gtk::Button {
                    set_hexpand: false,
                    set_halign: gtk::Align::Start,
                    set_size_request: (250, -1),
                    set_label: "Выберите документ для проверки:",
                    connect_clicked => AppMsg::OpenFile2
                },
                gtk::Separator {
                    set_margin_top: 5,
                    set_margin_bottom: 5
                },
                gtk::Label {
                    #[watch]
                    set_markup: &model.output_text,
                    inline_css: "background: #f0f0f0; border-radius: 5px;",
                    set_wrap: true,
                    set_hexpand: true,
                    set_vexpand: true,
                    set_justify: gtk::Justification::Left,
                    //set_halign: gtk::Align::Start,
                    //set_valign: gtk::Align::Start,
                    set_use_markup: true,
                },
                gtk::Button {
                    set_label: "Анализировать",
                    connect_clicked => AppMsg::Analyze
                },
            }
        }
    }

    fn init(
            init: Self::Init,
            root: Self::Root,
            _sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = VM { file1: init.clone(), file2: init.clone(), output_text: "<i>Результат будет отображён здесь</i>".to_string() };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::OpenFile1 => {}
            AppMsg::OpenFile2 => {}
            AppMsg::Analyze => { 
                self.output_text = "<i>Загрузка...</i>".to_string();
                let file1 = self.file1.clone();
                let file2 = self.file2.clone();
                let sender = sender.clone();
                
                gtk::glib::MainContext::default().spawn_local(async move {
                    let result = aiwork::process(&file1, &file2).await;
                    println!("{}",result);
                    sender.input(AppMsg::AnalysisResult(result));
                });
            }
            AppMsg::AnalysisResult(text) => {
                self.output_text = text;
            }
        }
    }
}

fn main() {
    let app = RelmApp::new(APPID);
    app.run::<VM>("".to_string());
}