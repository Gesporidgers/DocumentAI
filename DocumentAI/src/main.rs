#![windows_subsystem = "windows"]
mod aiwork;
mod pdfread;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use gtk::{FileChooserDialog, Window, gio::prelude::FileExt, glib::value, prelude::{BoxExt, ButtonExt, DialogExtManual, FileChooserExt, GtkWindowExt, OrientableExt, WidgetExt}};
const APPID: &str = "ru.teovr.documentai";

struct VM {
    file1 : String,
    file2 : String,
    is_first_selected : bool,
    is_second_selected : bool,
    output_text : String
}

#[derive(Debug)]
enum AppMsg {
    OpenFile1,
    OpenFile2,
    Analyze,
    AnalysisResult(String),
    SetVisibilityMark1(bool),
    SetVisibilityMark2(bool),
    SetFile1(String),
    SetFile2(String)
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
                gtk::Box{
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 6,
                    gtk::Button {
                        set_hexpand: false,
                        set_halign: gtk::Align::Start,
                        set_size_request: (250, -1),
                        set_label: "Выберите эталонный документ:",
                        connect_clicked => AppMsg::OpenFile1
                    },
                    gtk::Image {
                        set_size_request: (32,32),
                        set_hexpand: true,
                        set_icon_size: gtk::IconSize::Large,
                        set_from_file: Some("assets/mark.svg"),
                        #[watch]
                        set_visible: model.is_first_selected
                    }
                },
                
                gtk::Box{
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 6,
                    gtk::Button {
                        set_hexpand: false,
                        set_halign: gtk::Align::Start,
                        set_size_request: (250, -1),
                        set_label: "Выберите документ для проверки:",
                        connect_clicked => AppMsg::OpenFile2
                    },
                    gtk::Image {
                        set_size_request: (32,32),
                        set_hexpand: true,
                        set_icon_size: gtk::IconSize::Large,
                        set_from_file: Some("assets/mark.svg"),
                        #[watch]
                        set_visible: model.is_second_selected,
                    }
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
        let model = VM { 
            file1: init.clone(), 
            file2: init.clone(), 
            is_first_selected: false,
            is_second_selected: false,
            output_text: "<i>Результат будет отображён здесь</i>".to_string()
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::OpenFile1 => {
                let dia = FileChooserDialog::new(Some("Выберите эталонный документ"), Some(&Window::new()), gtk::FileChooserAction::Open, &[("Открыть", gtk::ResponseType::Accept), ("Отмена", gtk::ResponseType::Cancel)]);
                dia.add_filter(&{
                    let filter = gtk::FileFilter::new();
                    filter.set_name(Some("PDF файлы"));
                    filter.add_pattern("*.pdf");
                    filter
                });
                let sender_clone = sender.clone();
                dia.run_async(move |dia, resp| {
                    if resp == gtk::ResponseType::Accept {
                        if let Some(path) = dia.file() {
                            let path = path.path().map(|p| p.to_string_lossy().to_string());
                            sender_clone.input(AppMsg::SetVisibilityMark1(true));
                            sender_clone.input(AppMsg::SetFile1(path.unwrap_or_default()));
                            
                        }
                        else
                        {
                            sender_clone.input(AppMsg::SetVisibilityMark1(false));
                        }
                    }
                    dia.close();
                    
                });
            }
            AppMsg::OpenFile2 => {
                let dia = FileChooserDialog::new(Some("Выберите документ для проверки"), Some(&Window::new()), gtk::FileChooserAction::Open, &[("Открыть", gtk::ResponseType::Accept), ("Отмена", gtk::ResponseType::Cancel)]);
                dia.add_filter(&{
                    let filter = gtk::FileFilter::new();
                    filter.set_name(Some("PDF файлы"));
                    filter.add_pattern("*.pdf");
                    filter
                });
                let mut fileclone = self.file2.clone();
                let sender_clone = sender.clone();
                dia.run_async(move |dia, resp| {
                    if resp == gtk::ResponseType::Accept {
                        if let Some(path) = dia.file() {
                            let path = path.path().map(|p| p.to_string_lossy().to_string());
                            sender_clone.input(AppMsg::SetVisibilityMark2(true));
                            sender_clone.input(AppMsg::SetFile2(path.unwrap_or_default()));
                        }
                        else
                        {
                            sender_clone.input(AppMsg::SetVisibilityMark2(false));
                        }
                    }
                    dia.close();
                });
            }

            AppMsg::Analyze => { 
                self.output_text = "<i>Загрузка...</i>".to_string();
                let file1 = self.file1.clone();
                let file2 = self.file2.clone();
                let sender = sender.clone();
                let filecontent1 = pdfread::read_all_pdf(&file1);
                let filecontent2 = pdfread::read_all_pdf(&file2);
                gtk::glib::MainContext::default().spawn_local(async move {
                    let result = aiwork::process(&filecontent1, &filecontent2).await;
                    sender.input(AppMsg::AnalysisResult(result));
                });
            }
            AppMsg::AnalysisResult(text) => {
                self.output_text = text;
            }
            AppMsg::SetVisibilityMark1(val) => {
                self.is_first_selected = val;
            }
            AppMsg::SetVisibilityMark2(val) => {
                self.is_second_selected = val;
            }
            AppMsg::SetFile1(val) => {
                self.file1 = val;
            }
            AppMsg::SetFile2(val) => {
                self.file2 = val;
            }
        }
    }
}

fn main() {
    let app = RelmApp::new(APPID);
    app.run::<VM>("".to_string());
}